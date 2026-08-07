use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::Duration;

use bytes::{Buf, BufMut, Bytes};
use futures::{SinkExt, StreamExt};
use salvo::websocket::WebSocket;
use serde_json::Value;
use tonic::codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};
use tonic::{Request, Status};
use wallet_error::{AppError, AppResult};

use super::{rpc_error, ws_bridge, EndpointHeaders};

#[derive(Clone, Default)]
pub(crate) struct BytesCodec;

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

pub(crate) struct BytesEncoder;

impl Encoder for BytesEncoder {
    type Item = Bytes;
    type Error = Status;

    fn encode(&mut self, item: Self::Item, buf: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        buf.reserve(item.len());
        buf.put_slice(item.as_ref());
        Ok(())
    }
}

pub(crate) struct BytesDecoder;

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
    endpoint_for_grpc(&http_url, None, Some(timeout))?
        .connect()
        .await
        .map_err(|e| AppError::Unavailable(e.to_string()))?;
    Ok(())
}

pub(crate) fn endpoint_for_grpc(
    http_url: &str,
    request_timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
) -> AppResult<tonic::transport::Endpoint> {
    let mut endpoint = tonic::transport::Endpoint::from_shared(http_url.to_string())
        .map_err(|e| AppError::InvalidArgument(e.to_string()))?;
    if http_url.starts_with("https://") {
        endpoint = endpoint
            .tls_config(tonic::transport::ClientTlsConfig::new().with_enabled_roots())
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;
    }
    if let Some(t) = request_timeout {
        endpoint = endpoint.timeout(t);
    }
    if let Some(ct) = connect_timeout {
        endpoint = endpoint.connect_timeout(ct);
    }
    Ok(endpoint)
}

const MAX_GRPC_MESSAGE_SIZE: usize = 256 * 1024 * 1024;

static METADATA_KEYS: LazyLock<Mutex<HashMap<String, &'static str>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub(crate) fn metadata_key(name: &str) -> &'static str {
    let lower = name.to_ascii_lowercase();
    let mut cache = METADATA_KEYS.lock().expect("metadata key cache poisoned");
    if let Some(key) = cache.get(&lower) {
        return key;
    }
    let leaked: &'static str = Box::leak(lower.clone().into_boxed_str());
    cache.insert(lower, leaked);
    leaked
}

pub async fn execute(
    _http: &reqwest::Client,
    url: &str,
    body: &Value,
    headers: &EndpointHeaders,
    request_timeout: Duration,
) -> AppResult<Value> {
    let http_url = grpc_to_http_url(url)?;
    let method_path = grpc_method_path(url)?;

    let payload = serde_json::to_vec(body)
        .map_err(|e| AppError::internal(format!("encode json: {e}")))?;

    let channel = endpoint_for_grpc(&http_url, Some(request_timeout), None)?
        .connect_lazy();

    let path = http::uri::PathAndQuery::from_maybe_shared(method_path)
        .map_err(|e| AppError::InvalidArgument(e.to_string()))?;

    let mut grpc = tonic::client::Grpc::new(channel)
        .max_decoding_message_size(MAX_GRPC_MESSAGE_SIZE)
        .max_encoding_message_size(MAX_GRPC_MESSAGE_SIZE);

    grpc.ready()
        .await
        .map_err(|e| AppError::Unavailable(e.to_string()))?;

    let mut request = Request::new(Bytes::from(payload));
    for (name, value) in headers {
        if name.eq_ignore_ascii_case("content-type") || name.eq_ignore_ascii_case("te") {
            continue;
        }
        if name.starts_with(':') {
            continue;
        }
        if let Ok(metadata_value) =
            value.parse::<tonic::metadata::MetadataValue<tonic::metadata::Ascii>>()
        {
            let key = metadata_key(name);
            request.metadata_mut().insert(key, metadata_value);
        }
    }

    let response = grpc
        .unary(request, path, BytesCodec)
        .await
        .map_err(|e| AppError::Unavailable(e.to_string()))?;

    decode_grpc_json_response(&response.into_inner())
}

fn decode_grpc_json_response(bytes: &[u8]) -> AppResult<Value> {
    let v: Value = serde_json::from_slice(bytes)
        .map_err(|e| AppError::Unavailable(format!("invalid grpc json: {e}")))?;
    if let Some(err) = rpc_error(&v) {
        return Err(AppError::Unavailable(format!("rpc error: {err}")));
    }
    Ok(v)
}

pub async fn tunnel(
    client_ws: WebSocket,
    url: &str,
    headers: &EndpointHeaders,
) -> AppResult<()> {
    let http_url = grpc_to_http_url(url)?;
    let method_path = grpc_method_path(url)?;
    let channel = endpoint_for_grpc(&http_url, None, None)?
        .connect()
        .await
        .map_err(|e| AppError::Unavailable(e.to_string()))?;

    let path = http::uri::PathAndQuery::from_maybe_shared(method_path)
        .map_err(|e| AppError::InvalidArgument(e.to_string()))?;
    let mut grpc = tonic::client::Grpc::new(channel);
    let (tx, rx) = tokio::sync::mpsc::channel::<Bytes>(32);
    let mut request = Request::new(tokio_stream::wrappers::ReceiverStream::new(rx));
    for (name, value) in headers {
        if name.starts_with(':') {
            continue;
        }
        if let Ok(metadata_value) =
            value.parse::<tonic::metadata::MetadataValue<tonic::metadata::Ascii>>()
        {
            let key: &'static str = Box::leak(name.to_ascii_lowercase().into_boxed_str());
            request.metadata_mut().insert(key, metadata_value);
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use http_body_util::combinators::BoxBody;
    use std::convert::Infallible;
    use std::sync::Arc;
    use std::net::SocketAddr;

    // ── URL / 端口解析 ─────────────────────────────────────────

    #[test]
    fn grpc_to_http_url_preserves_port_and_path() {
        assert_eq!(
            grpc_to_http_url("grpcs://gmain.vvwallet.org:443/protocol.Wallet/Wallet_GetNowBlock")
                .unwrap(),
            "https://gmain.vvwallet.org:443/protocol.Wallet/Wallet_GetNowBlock"
        );
        assert_eq!(
            grpc_to_http_url("grpcs://grpc.trongrid.io:50051/protocol.Wallet/Wallet_GetNowBlock")
                .unwrap(),
            "https://grpc.trongrid.io:50051/protocol.Wallet/Wallet_GetNowBlock"
        );
        assert_eq!(
            grpc_to_http_url("grpc://127.0.0.1:50051/protocol.Wallet/Wallet_GetNowBlock").unwrap(),
            "http://127.0.0.1:50051/protocol.Wallet/Wallet_GetNowBlock"
        );
        assert!(grpc_to_http_url("https://api.trongrid.io").is_err());
    }

    #[test]
    fn grpc_method_path_extracts_service_and_method() {
        assert_eq!(
            grpc_method_path("grpcs://gmain.vvwallet.org:443/protocol.Wallet/Wallet_GetNowBlock")
                .unwrap(),
            "/protocol.Wallet/Wallet_GetNowBlock"
        );
        assert_eq!(
            grpc_method_path("grpc://127.0.0.1:50051/grpc.health.v1.Health/Check").unwrap(),
            "/grpc.health.v1.Health/Check"
        );
        assert!(grpc_method_path("grpc://127.0.0.1:50051").is_err());
    }

    // ── 本地 HTTP/2 gRPC 服务器 ──────────────────────────────

    type TestBody = BoxBody<Bytes, Infallible>;

    fn grpc_frame_bytes(fail: bool) -> Vec<u8> {
        let payload = if fail {
            Vec::new()
        } else {
            serde_json::to_vec(&serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": {"block_header": {"raw_data": {"number": 12345}}}
            }))
            .unwrap()
        };
        let mut frame = Vec::with_capacity(payload.len() + 5);
        frame.push(0);
        frame.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        frame.extend_from_slice(&payload);
        frame
    }

    async fn grpc_test_handler(
        req: http::Request<hyper::body::Incoming>,
        seen: Arc<Mutex<Vec<(String, String)>>>,
        fail: bool,
    ) -> Result<http::Response<TestBody>, Infallible> {
        if let Some(auth) = req.uri().authority() {
            seen.lock()
                .unwrap()
                .push((":authority".to_string(), auth.to_string()));
        }
        for (name, value) in req.headers() {
            if let Ok(v) = value.to_str() {
                seen.lock()
                    .unwrap()
                    .push((name.to_string(), v.to_string()));
            }
        }

        let mut trailers = http::HeaderMap::new();
        if fail {
            trailers.insert("grpc-status", http::HeaderValue::from_static("13"));
            trailers.insert(
                "grpc-message",
                http::HeaderValue::from_static("internal error"),
            );
        } else {
            trailers.insert("grpc-status", http::HeaderValue::from_static("0"));
        }

        let stream = futures::stream::iter(vec![
            Ok::<_, Infallible>(hyper::body::Frame::data(Bytes::from(grpc_frame_bytes(fail)))),
            Ok::<_, Infallible>(hyper::body::Frame::trailers(trailers)),
        ]);
        let body =
            http_body_util::BodyExt::boxed(http_body_util::StreamBody::new(stream));

        Ok(http::Response::builder()
            .status(200)
            .header("content-type", "application/grpc")
            .body(body)
            .unwrap())
    }

    async fn spawn_test_grpc_server(
        fail: bool,
    ) -> (SocketAddr, Arc<Mutex<Vec<(String, String)>>>, tokio::task::JoinHandle<()>) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let seen = Arc::new(Mutex::new(Vec::new()));
        let seen_conn = seen.clone();

        let handle = tokio::spawn(async move {
            let executor = hyper_util::rt::TokioExecutor::new();
            let builder = hyper::server::conn::http2::Builder::new(executor);
            loop {
                let (stream, _) = match listener.accept().await {
                    Ok(s) => s,
                    Err(_) => break,
                };
                let builder = builder.clone();
                let seen = seen_conn.clone();
                tokio::spawn(async move {
                    let io = hyper_util::rt::TokioIo::new(stream);
                    let svc = hyper::service::service_fn(move |req| {
                        grpc_test_handler(req, seen.clone(), fail)
                    });
                    let _ = builder.serve_connection(io, svc).await;
                });
            }
        });

        (addr, seen, handle)
    }

    // ── 集成：端口 + 请求头转发 + 响应解析 ─────────────────────

    #[tokio::test]
    async fn execute_forwards_headers_and_port_over_h2() {
        let (addr, seen, handle) = spawn_test_grpc_server(false).await;

        let url = format!("grpc://{addr}/protocol.Wallet/Wallet_GetNowBlock");
        let mut headers = EndpointHeaders::new();
        headers.insert(
            "x-api-key".to_string(),
            "wt_cf22c321a278864e3aa6da52662acdd5s".to_string(),
        );
        headers.insert("x-forward-header".to_string(), "yes".to_string());

        let client = reqwest::Client::new();
        let v = execute(
            &client,
            &url,
            &serde_json::json!({"jsonrpc":"2.0","id":1,"method":"getnowblock","params":[]}),
            &headers,
            Duration::from_secs(5),
        )
        .await
        .expect("grpc call should succeed");

        assert_eq!(v["result"]["block_header"]["raw_data"]["number"], 12345);

        let recorded = seen.lock().unwrap();
        let map: HashMap<&str, &str> = recorded
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        let authority = addr.to_string();
        assert_eq!(
            map.get(":authority").copied(),
            Some(authority.as_str()),
            "grpc url 中的端口应出现在 :authority"
        );
        assert_eq!(map.get("x-api-key").copied(), Some("wt_cf22c321a278864e3aa6da52662acdd5s"));
        assert_eq!(map.get("x-forward-header").copied(), Some("yes"));
        assert_eq!(map.get("content-type").copied(), Some("application/grpc"));

        handle.abort();
    }

    #[tokio::test]
    async fn execute_surfaces_grpc_status_error() {
        let (addr, _seen, handle) = spawn_test_grpc_server(true).await;

        let url = format!("grpc://{addr}/protocol.Wallet/Wallet_GetNowBlock");
        let client = reqwest::Client::new();
        let err = execute(
            &client,
            &url,
            &serde_json::json!({"jsonrpc":"2.0","id":1,"method":"getnowblock","params":[]}),
            &EndpointHeaders::new(),
            Duration::from_secs(5),
        )
        .await
        .unwrap_err();

        let msg = err.to_string();
        assert!(msg.contains("internal error"), "got: {msg}");

        handle.abort();
    }
}
