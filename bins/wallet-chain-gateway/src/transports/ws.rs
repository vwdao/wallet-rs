use std::time::Duration;

use futures::{SinkExt, StreamExt};
use salvo::websocket::{Message, WebSocket};
use serde_json::Value;
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::client::{ClientRequestBuilder, IntoClientRequest};
use tokio_tungstenite::tungstenite::Message as UpstreamMessage;
use tokio_tungstenite::connect_async;
use wallet_error::{AppError, AppResult};

use super::ws_bridge;
use super::EndpointHeaders;

fn ws_request(
    url: &str,
    headers: &EndpointHeaders,
) -> Result<http::Request<()>, AppError> {
    let uri: http::Uri = url
        .parse()
        .map_err(|e| AppError::InvalidArgument(format!("invalid ws url '{url}': {e}")))?;
    let mut builder = ClientRequestBuilder::new(uri);
    for (name, value) in headers {
        builder = builder.with_header(name.clone(), value.clone());
    }
    let request = builder
        .into_client_request()
        .map_err(|e| AppError::InvalidArgument(format!("build ws request: {e}")))?;
    Ok(request)
}

pub async fn execute(
    url: &str,
    body: &Value,
    headers: &EndpointHeaders,
    request_timeout: Duration,
) -> AppResult<Value> {
    let payload = serde_json::to_string(body)
        .map_err(|e| AppError::internal(format!("encode json: {e}")))?;

    let request = ws_request(url, headers)?;
    let connect = connect_async(request);
    let (ws, _) = timeout(request_timeout, connect)
        .await
        .map_err(|_| AppError::Unavailable("ws connect timeout".into()))?
        .map_err(|e| AppError::Unavailable(e.to_string()))?;

    let (mut write, mut read) = ws.split();
    write
        .send(UpstreamMessage::Text(payload.into()))
        .await
        .map_err(|e| AppError::Unavailable(e.to_string()))?;

    let response = timeout(request_timeout, read.next())
        .await
        .map_err(|_| AppError::Unavailable("ws read timeout".into()))?
        .ok_or_else(|| AppError::Unavailable("ws closed before response".into()))?
        .map_err(|e| AppError::Unavailable(e.to_string()))?;

    let text = match response {
        UpstreamMessage::Text(t) => t.to_string(),
        UpstreamMessage::Binary(b) => String::from_utf8(b.to_vec())
            .map_err(|e| AppError::Unavailable(format!("invalid ws binary response: {e}")))?,
        _ => return Err(AppError::Unavailable("unexpected ws message type".into())),
    };

    let v: Value = serde_json::from_str(&text)
        .map_err(|e| AppError::Unavailable(format!("invalid json response: {e}")))?;
    if v.get("error").is_some() {
        return Err(AppError::Unavailable(format!("rpc error: {}", v["error"])));
    }
    Ok(v)
}

pub async fn tunnel(client_ws: WebSocket, url: &str, headers: &EndpointHeaders) -> AppResult<()> {
    let request = ws_request(url, headers)?;
    let (upstream, _) = connect_async(request)
        .await
        .map_err(|e| AppError::Unavailable(e.to_string()))?;

    let (mut client_tx, mut client_rx) = client_ws.split();
    let (mut upstream_tx, mut upstream_rx) = upstream.split();

    let client_to_upstream = async move {
        while let Some(Ok(msg)) = client_rx.next().await {
            if ws_bridge::is_close(&msg) {
                let _ = upstream_tx.send(UpstreamMessage::Close(None)).await;
                break;
            }
            if ws_bridge::is_control(&msg) {
                continue;
            }
            let bytes = ws_bridge::payload_bytes(&msg);
            let upstream_msg = if msg.is_text() {
                UpstreamMessage::Text(String::from_utf8(bytes).unwrap_or_default().into())
            } else {
                UpstreamMessage::Binary(bytes.into())
            };
            if upstream_tx.send(upstream_msg).await.is_err() {
                break;
            }
        }
    };

    let upstream_to_client = async move {
        while let Some(result) = upstream_rx.next().await {
            let msg = match result {
                Ok(UpstreamMessage::Text(text)) => Message::text(text.to_string()),
                Ok(UpstreamMessage::Binary(bin)) => Message::binary(bin.to_vec()),
                Ok(UpstreamMessage::Ping(data)) => Message::ping(data.to_vec()),
                Ok(UpstreamMessage::Pong(data)) => Message::pong(data.to_vec()),
                Ok(UpstreamMessage::Close(_)) => break,
                Ok(_) => continue,
                Err(_) => break,
            };
            if client_tx.send(msg).await.is_err() {
                break;
            }
        }
    };

    tokio::select! {
        _ = client_to_upstream => {},
        _ = upstream_to_client => {},
    }
    Ok(())
}
