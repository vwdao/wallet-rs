use std::time::Duration;

use futures::{SinkExt, StreamExt};
use salvo::websocket::WebSocket;
use serde_json::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;
use url::Url;
use wallet_error::{AppError, AppResult};

use super::{rpc_error, ws_bridge};

pub async fn execute(url: &Url, body: &Value, request_timeout: Duration) -> AppResult<Value> {
    let host = url
        .host_str()
        .ok_or_else(|| AppError::InvalidArgument("tcp url missing host".into()))?;
    let port = url.port().unwrap_or(80);
    let addr = format!("{host}:{port}");

    let connect = TcpStream::connect(&addr);
    let mut stream = timeout(request_timeout, connect)
        .await
        .map_err(|_| AppError::Unavailable("tcp connect timeout".into()))?
        .map_err(|e| AppError::Unavailable(e.to_string()))?;

    let payload = serde_json::to_string(body)
        .map_err(|e| AppError::internal(format!("encode json: {e}")))?;
    let framed = format!("{payload}\n");

    timeout(request_timeout, stream.write_all(framed.as_bytes()))
        .await
        .map_err(|_| AppError::Unavailable("tcp write timeout".into()))?
        .map_err(|e| AppError::Unavailable(e.to_string()))?;

    let mut buf = Vec::with_capacity(4096);
    let mut chunk = [0u8; 1024];
    let read_deadline = request_timeout;
    loop {
        let n = timeout(read_deadline, stream.read(&mut chunk))
            .await
            .map_err(|_| AppError::Unavailable("tcp read timeout".into()))?
            .map_err(|e| AppError::Unavailable(e.to_string()))?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&chunk[..n]);
        if buf.contains(&b'\n') {
            break;
        }
        if buf.len() > 1_048_576 {
            return Err(AppError::Unavailable("tcp response too large".into()));
        }
    }

    let line = buf
        .split(|b| *b == b'\n')
        .next()
        .ok_or_else(|| AppError::Unavailable("empty tcp response".into()))?;
    let text = std::str::from_utf8(line)
        .map_err(|e| AppError::Unavailable(format!("invalid utf8 response: {e}")))?;
    let v: Value = serde_json::from_str(text)
        .map_err(|e| AppError::Unavailable(format!("invalid json response: {e}")))?;
    if let Some(err) = rpc_error(&v) {
        return Err(AppError::Unavailable(format!("rpc error: {err}")));
    }
    Ok(v)
}

pub async fn tunnel(client_ws: WebSocket, url: &Url) -> AppResult<()> {
    let host = url
        .host_str()
        .ok_or_else(|| AppError::InvalidArgument("tcp url missing host".into()))?;
    let port = url.port().unwrap_or(80);
    let addr = format!("{host}:{port}");

    let upstream = TcpStream::connect(&addr)
        .await
        .map_err(|e| AppError::Unavailable(e.to_string()))?;
    let (mut tcp_read, mut tcp_write) = upstream.into_split();

    let (mut client_tx, mut client_rx) = client_ws.split();

    let client_to_tcp = async move {
        while let Some(Ok(msg)) = client_rx.next().await {
            if ws_bridge::is_close(&msg) || ws_bridge::is_control(&msg) {
                if ws_bridge::is_close(&msg) {
                    break;
                }
                continue;
            }
            let bytes = ws_bridge::payload_bytes(&msg);
            if tcp_write.write_all(&bytes).await.is_err() {
                break;
            }
        }
    };

    let tcp_to_client = async move {
        let mut buf = [0u8; 4096];
        loop {
            let n = match tcp_read.read(&mut buf).await {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };
            if client_tx
                .send(ws_bridge::to_text_or_binary(&buf[..n]))
                .await
                .is_err()
            {
                break;
            }
        }
    };

    tokio::select! {
        _ = client_to_tcp => {},
        _ = tcp_to_client => {},
    }
    Ok(())
}
