use std::time::Duration;

use bytes::{Buf, BufMut, Bytes};
use futures::{SinkExt, StreamExt};
use salvo::websocket::WebSocket;
use serde_json::Value;
use tonic::codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};
use tonic::transport::Channel;
use tonic::{Request, Status};
use wallet_error::{AppError, AppResult};

use super::ws_bridge;

#[derive(Clone, Default)]
struct BytesCodec;

impl Codec for BytesCodec {
    type Encode = Bytes;
    type Decode = Bytes;
    type Encoder = BytesEncoder;
    type Decoder = BytesDecoder;

    fn encoder(&mut self) -> Self::Encoder {
        BytesEncoder
    }

    fn decoder(&mut self) -> Self::Decoder {
        BytesDecoder
    }
}

struct BytesEncoder;

impl Encoder for BytesEncoder {
    type Item = Bytes;
    type Error = Status;

    fn encode(&mut self, item: Self::Item, buf: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        buf.reserve(item.len());
        buf.put_slice(item.as_ref());
        Ok(())
    }
}

struct BytesDecoder;

impl Decoder for BytesDecoder {
    type Item = Bytes;
    type Error = Status;

    fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        if buf.remaining() == 0 {
            return Ok(None);
        }
        Ok(Some(buf.copy_to_bytes(buf.remaining())))
    }
}

pub fn grpc_to_http_url(raw: &str) -> AppResult<String> {
    if raw.starts_with("grpcs://") {
        Ok(raw.replacen("grpcs://", "https://", 1))
    } else if raw.starts_with("grpc://") {
        Ok(raw.replacen("grpc://", "http://", 1))
    } else {
        Err(AppError::InvalidArgument(format!("not a grpc url: {raw}")))
    }
}

fn grpc_method_path(raw: &str) -> AppResult<String> {
    let http_url = grpc_to_http_url(raw)?;
    let parsed = url::Url::parse(&http_url)
        .map_err(|e| AppError::InvalidArgument(format!("invalid grpc url: {e}")))?;
    let path = parsed.path().trim_matches('/');
    if path.is_empty() {
        return Err(AppError::InvalidArgument(
            "grpc url must include /Service/Method path".into(),
        ));
    }
    Ok(format!("/{path}"))
}

pub async fn probe_connect(url: &str, timeout: Duration) -> AppResult<()> {
    let http_url = grpc_to_http_url(url)?;
    Channel::from_shared(http_url)
        .map_err(|e| AppError::InvalidArgument(e.to_string()))?
        .timeout(timeout)
        .connect_timeout(timeout)
        .connect()
        .await
        .map_err(|e| AppError::Unavailable(e.to_string()))?;
    Ok(())
}

pub async fn execute(
    http: &reqwest::Client,
    url: &str,
    body: &Value,
    request_timeout: Duration,
) -> AppResult<Value> {
    let http_url = grpc_to_http_url(url)?;
    let method_path = grpc_method_path(url)?;

    let payload = serde_json::to_vec(body)
        .map_err(|e| AppError::internal(format!("encode json: {e}")))?;
    let mut frame = Vec::with_capacity(payload.len() + 5);
    frame.push(0);
    frame.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    frame.extend_from_slice(&payload);

    let target = format!("{http_url}{method_path}");
    let resp = http
        .post(&target)
        .header("content-type", "application/grpc")
        .header("te", "trailers")
        .body(frame)
        .timeout(request_timeout)
        .send()
        .await
        .map_err(|e| AppError::Unavailable(e.to_string()))?;

    if !resp.status().is_success() {
        return Err(AppError::Unavailable(format!(
            "grpc returned {}",
            resp.status()
        )));
    }

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| AppError::Unavailable(e.to_string()))?;
    decode_grpc_json_response(&bytes)
}

fn decode_grpc_json_response(bytes: &[u8]) -> AppResult<Value> {
    if bytes.len() < 5 {
        return Err(AppError::Unavailable("grpc response too short".into()));
    }
    let len = u32::from_be_bytes(bytes[1..5].try_into().unwrap()) as usize;
    if bytes.len() < 5 + len {
        return Err(AppError::Unavailable("grpc response truncated".into()));
    }
    let payload = &bytes[5..5 + len];
    let v: Value = serde_json::from_slice(payload)
        .map_err(|e| AppError::Unavailable(format!("invalid grpc json: {e}")))?;
    if v.get("error").is_some() {
        return Err(AppError::Unavailable(format!("rpc error: {}", v["error"])));
    }
    Ok(v)
}

pub async fn tunnel(client_ws: WebSocket, url: &str) -> AppResult<()> {
    let http_url = grpc_to_http_url(url)?;
    let method_path = grpc_method_path(url)?;
    let channel = Channel::from_shared(http_url)
        .map_err(|e| AppError::InvalidArgument(e.to_string()))?
        .connect()
        .await
        .map_err(|e| AppError::Unavailable(e.to_string()))?;

    let path = http::uri::PathAndQuery::from_maybe_shared(method_path)
        .map_err(|e| AppError::InvalidArgument(e.to_string()))?;
    let mut grpc = tonic::client::Grpc::new(channel);
    let (tx, rx) = tokio::sync::mpsc::channel::<Bytes>(32);
    let request = Request::new(tokio_stream::wrappers::ReceiverStream::new(rx));

    let mut response = grpc
        .streaming(request, path, BytesCodec)
        .await
        .map_err(|e| AppError::Unavailable(e.to_string()))?
        .into_inner();

    let (mut client_tx, mut client_rx) = client_ws.split();

    let send_task = async move {
        while let Some(Ok(msg)) = client_rx.next().await {
            if ws_bridge::is_close(&msg) {
                break;
            }
            if ws_bridge::is_control(&msg) {
                continue;
            }
            let bytes = Bytes::from(ws_bridge::payload_bytes(&msg));
            if tx.send(bytes).await.is_err() {
                break;
            }
        }
    };

    let recv_task = async move {
        loop {
            match response.message().await {
                Ok(Some(bytes)) => {
                    let payload = ws_bridge::to_text_or_binary(&bytes);
                    if client_tx.send(payload).await.is_err() {
                        break;
                    }
                }
                Ok(None) => break,
                Err(_) => break,
            }
        }
    };

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
    Ok(())
}
