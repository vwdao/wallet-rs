use bytes::Bytes;
use http::{HeaderMap, HeaderValue, Method, StatusCode};
use http_body_util::{BodyExt, Full, StreamBody};
use hyper::body::{Frame, Incoming};
use hyper::service::service_fn;
use hyper_util::rt::{TokioExecutor, TokioIo};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tonic::Request;
use wallet_db::{ChainGatewayKey, Db};
use wallet_error::{AppError, AppResult};

use crate::proxy::{check_ip_policy, check_rate, resolve_chain_index};
use crate::stats::StatsEvent;
use crate::transports::grpc::{grpc_to_http_url, metadata_key};
use crate::transports::{endpoint_for_grpc, BytesCodec, EndpointHeaders};
use crate::Gw;
use serde_json::json;

const MAX_GRPC_MESSAGE_SIZE: usize = 256 * 1024 * 1024;

type BoxBody = http_body_util::combinators::BoxBody<Bytes, Infallible>;

pub fn spawn(state: Gw, addr: SocketAddr) {
    tokio::spawn(async move {
        let listener = match tokio::net::TcpListener::bind(addr).await {
            Ok(l) => l,
            Err(e) => {
                tracing::error!("grpc proxy bind {addr} failed: {e}");
                return;
            }
        };
        tracing::info!("chain-gateway grpc proxy on {addr}");
        loop {
            let (stream, peer) = match listener.accept().await {
                Ok(s) => s,
                Err(e) => {
                    tracing::warn!("grpc proxy accept failed: {e}");
                    break;
                }
            };
            let state = state.clone();
            tokio::spawn(async move {
                let io = TokioIo::new(stream);
                let svc = service_fn(move |req| grpc_serve(req, state.clone(), peer));
                let builder = hyper::server::conn::http2::Builder::new(TokioExecutor::new());
                if let Err(e) = builder.serve_connection(io, svc).await {
                    tracing::debug!("grpc proxy connection error: {e}");
                }
            });
        }
    });
}

struct ProxyOutcome {
    code: u32,
    message: String,
    payload: Option<Bytes>,
    api_key: String,
    chain_index: i64,
    method: String,
    upstream_url: String,
}

impl ProxyOutcome {
    fn err(code: u32, message: impl Into<String>, method: &str) -> Self {
        Self {
            code,
            message: message.into(),
            payload: None,
            api_key: String::new(),
            chain_index: 0,
            method: method.to_string(),
            upstream_url: String::new(),
        }
    }

    fn ok(payload: Bytes, api_key: &str, chain_index: i64, method: &str, upstream_url: &str) -> Self {
        Self {
            code: 0,
            message: String::new(),
            payload: Some(payload),
            api_key: api_key.to_string(),
            chain_index,
            method: method.to_string(),
            upstream_url: upstream_url.to_string(),
        }
    }
}

async fn grpc_serve(
    req: http::Request<Incoming>,
    state: Gw,
    peer: SocketAddr,
) -> Result<http::Response<BoxBody>, Infallible> {
    if req.method() != Method::POST {
        return Ok(http::Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(empty_body())
            .unwrap());
    }
    let content_type = req
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    if !content_type.starts_with("application/grpc") {
        return Ok(http::Response::builder()
            .status(StatusCode::UNSUPPORTED_MEDIA_TYPE)
            .body(empty_body())
            .unwrap());
    }

    let client_ip = peer.ip().to_string();
    let started = Instant::now();
    let outcome = proxy_call(req, &state, client_ip.clone()).await;
    let latency = started.elapsed().as_millis() as i32;

    if !outcome.api_key.is_empty() {
        state.stats.record(StatsEvent {
            api_key: outcome.api_key.clone(),
            chain_index: outcome.chain_index,
            client_ip: Some(client_ip.clone()),
            protocol: Some("grpc".into()),
            method: Some(outcome.method.clone()),
            status_code: if outcome.payload.is_some() { 200 } else { 503 },
            latency_ms: latency,
            error_msg: if outcome.code == 0 {
                None
            } else {
                Some(outcome.message.clone())
            },
        });
    }

    if state.settings.get().log_requests {
        let log = json!({
            "type": "grpc_proxy_request",
            "method": &outcome.method,
            "api_key": &outcome.api_key,
            "upstream_url": &outcome.upstream_url,
            "status_code": outcome.code,
            "latency_ms": latency,
            "client_ip": &client_ip,
        });
        tracing::info!(log = %log, "grpc proxy request");
    }

    Ok(grpc_response(outcome.code, &outcome.message, outcome.payload))
}

async fn proxy_call(req: http::Request<Incoming>, state: &Gw, client_ip: String) -> ProxyOutcome {
    let method = req.uri().path().to_string();

    if !state.settings.is_ready() {
        return ProxyOutcome::err(14, "gateway not ready", &method);
    }

    let api_key = match extract_api_key(req.headers()) {
        Some(k) => k,
        None => {
            return ProxyOutcome::err(
                16,
                "missing api key (x-api-key / TRON-PRO-API-KEY metadata)",
                &method,
            )
        }
    };

    let x_chain = req
        .headers()
        .get("x-chain")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let body = match req.into_body().collect().await {
        Ok(b) => b.to_bytes(),
        Err(e) => return ProxyOutcome::err(13, format!("failed to read body: {e}"), &method),
    };
    let payload = match parse_grpc_message(&body) {
        Ok(p) => p,
        Err(msg) => return ProxyOutcome::err(3, msg, &method),
    };

    let cfg = state.settings.get();
    if cfg.global_rate_limit_per_min > 0 {
        if let Err(e) = check_rate(&state.rate, "_global", cfg.global_rate_limit_per_min) {
            return ProxyOutcome::err(grpc_code(&e), e.to_string(), &method);
        }
    }

    let mut db = state.db.clone_inner();
    let mut key_row = match ChainGatewayKey::filter_by_api_key(&api_key).get(&mut db).await {
        Ok(r) => r,
        Err(_) => return ProxyOutcome::err(16, "invalid api key", &method),
    };
    if !key_row.enabled {
        return ProxyOutcome::err(16, "api key disabled", &method);
    }
    if let Err(e) = check_ip_policy(&key_row, Some(&client_ip)) {
        return ProxyOutcome::err(grpc_code(&e), e.to_string(), &method);
    }
    if let Err(e) = check_rate(&state.rate, &api_key, key_row.rate_limit_per_min.max(0) as usize)
    {
        return ProxyOutcome::err(grpc_code(&e), e.to_string(), &method);
    }

    let chain_index =
        match resolve_grpc_chain(&state.db, x_chain.as_deref(), &key_row, &method).await {
            Ok(c) => c,
            Err(e) => return ProxyOutcome::err(grpc_code(&e), e.to_string(), &method),
        };
    if !key_row.allowed_chains.is_empty() && !key_row.allowed_chains.contains(&chain_index) {
        return ProxyOutcome::err(16, "chain not allowed for api key", &method);
    }

    let next_total_requests = key_row.total_requests.saturating_add(1);
    {
        let mut db = state.db.clone_inner();
        let _ = key_row
            .update()
            .total_requests(next_total_requests)
            .exec(&mut db)
            .await;
    }

    let selection = match state
        .router
        .select_endpoint(chain_index, None, &key_row.allowed_tier, &cfg, false, false)
        .await
    {
        Ok(s) => s,
        Err(e) => return ProxyOutcome::err(grpc_code(&e), e.to_string(), &method),
    };

    match forward_unary(
        &selection.url,
        &method,
        &payload,
        &selection.headers,
        Duration::from_secs(cfg.rpc_timeout_secs),
    )
    .await
    {
        Ok(bytes) => {
            state.router.mark_success(&selection.url);
            ProxyOutcome::ok(bytes, &api_key, chain_index, &method, &selection.url)
        }
        Err((code, msg)) => {
            state.router.mark_failure(&selection.url);
            ProxyOutcome {
                code,
                message: msg,
                upstream_url: selection.url.clone(),
                ..ProxyOutcome::err(0, "", &method)
            }
            .with_key(&api_key, chain_index)
        }
    }
}

impl ProxyOutcome {
    fn with_key(mut self, api_key: &str, chain_index: i64) -> Self {
        self.api_key = api_key.to_string();
        self.chain_index = chain_index;
        self
    }
}

async fn forward_unary(
    endpoint_url: &str,
    client_method: &str,
    payload: &Bytes,
    headers: &EndpointHeaders,
    timeout: Duration,
) -> Result<Bytes, (u32, String)> {
    let (upstream_url, method_path) = match grpc_proxy_upstream(endpoint_url, client_method) {
        Ok(v) => v,
        Err(e) => return Err((grpc_code(&e), e.to_string())),
    };

    let channel = endpoint_for_grpc(&upstream_url, Some(timeout), None)
        .map_err(|e| (3, e.to_string()))?
        .connect_lazy();

    let path = http::uri::PathAndQuery::from_maybe_shared(method_path)
        .map_err(|e| (3, e.to_string()))?;

    let mut grpc = tonic::client::Grpc::new(channel)
        .max_decoding_message_size(MAX_GRPC_MESSAGE_SIZE)
        .max_encoding_message_size(MAX_GRPC_MESSAGE_SIZE);

    grpc.ready()
        .await
        .map_err(|e| (14, e.to_string()))?;

    let mut request = Request::new(payload.clone());
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
            request.metadata_mut().insert(metadata_key(name), metadata_value);
        }
    }

    match grpc.unary(request, path, BytesCodec).await {
        Ok(resp) => Ok(resp.into_inner()),
        Err(status) => Err((i32::from(status.code()) as u32, status.message().to_string())),
    }
}

fn grpc_proxy_upstream(endpoint_url: &str, client_method: &str) -> AppResult<(String, String)> {
    let http_url = grpc_to_http_url(endpoint_url)?;
    let parsed = url::Url::parse(&http_url)
        .map_err(|e| AppError::InvalidArgument(format!("invalid grpc url: {e}")))?;
    if parsed.host_str().is_none() {
        return Err(AppError::InvalidArgument("grpc url missing host".into()));
    }
    let (scheme, rest) = http_url
        .split_once("://")
        .ok_or_else(|| AppError::InvalidArgument("invalid grpc url".into()))?;
    let authority = rest.split('/').next().unwrap_or_default();
    let origin = format!("{scheme}://{authority}");
    let base_path = parsed.path().trim_end_matches('/');
    let method_path = if base_path.is_empty() {
        client_method.to_string()
    } else {
        format!("{base_path}{client_method}")
    };
    Ok((origin, method_path))
}

async fn resolve_grpc_chain(
    db: &Db,
    x_chain: Option<&str>,
    key_row: &ChainGatewayKey,
    method: &str,
) -> AppResult<i64> {
    if key_row.allowed_chains.len() == 1 {
        return Ok(key_row.allowed_chains[0]);
    }
    if let Some(chain) = x_chain {
        if let Ok(idx) = chain.trim().parse::<i64>() {
            return Ok(idx);
        }
        return resolve_chain_index(db, chain.trim()).await;
    }
    if method.starts_with("/protocol.") {
        return resolve_chain_index(db, "tron").await;
    }
    Err(AppError::InvalidArgument(
        "cannot resolve chain: set x-chain metadata or bind the api key to a single chain".into(),
    ))
}

fn extract_api_key(headers: &HeaderMap) -> Option<String> {
    if let Some(v) = headers
        .get("x-api-key")
        .or_else(|| headers.get("TRON-PRO-API-KEY"))
        .or_else(|| headers.get("trongrid-pro-api-key"))
    {
        if let Ok(s) = v.to_str() {
            return Some(s.trim().to_string());
        }
    }
    if let Some(v) = headers.get("authorization") {
        if let Ok(s) = v.to_str() {
            if let Some(rest) = s.strip_prefix("Bearer ") {
                return Some(rest.trim().to_string());
            }
        }
    }
    None
}

fn grpc_code(e: &AppError) -> u32 {
    let code = match e {
        AppError::NotFound(_) => tonic::Code::NotFound,
        AppError::Unauthorized => tonic::Code::Unauthenticated,
        AppError::Forbidden => tonic::Code::PermissionDenied,
        AppError::TooManyRequests => tonic::Code::ResourceExhausted,
        AppError::InvalidArgument(_) | AppError::ChainNotSupported(_) => {
            tonic::Code::InvalidArgument
        }
        AppError::Unavailable(_) => tonic::Code::Unavailable,
        AppError::Unimplemented(_) => tonic::Code::Unimplemented,
        AppError::Internal(_) | AppError::Other(_) => tonic::Code::Internal,
    };
    i32::from(code) as u32
}

fn parse_grpc_message(whole: &[u8]) -> Result<Bytes, String> {
    if whole.len() < 5 {
        return Err("truncated grpc frame".into());
    }
    if whole[0] & 0x01 != 0 {
        return Err("compressed grpc messages are not supported".into());
    }
    let len = u32::from_be_bytes([whole[1], whole[2], whole[3], whole[4]]) as usize;
    if whole.len() < 5 + len {
        return Err("truncated grpc message".into());
    }
    Ok(Bytes::copy_from_slice(&whole[5..5 + len]))
}

fn frame_message(payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(payload.len() + 5);
    out.push(0);
    out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    out.extend_from_slice(payload);
    out
}

fn percent_encode(input: &str) -> String {
    let mut out = String::new();
    for &b in input.as_bytes() {
        if (0x20..=0x7e).contains(&b) && b != b'%' {
            out.push(b as char);
        } else {
            out.push('%');
            out.push_str(&format!("{b:02X}"));
        }
    }
    out
}

fn grpc_response(code: u32, message: &str, payload: Option<Bytes>) -> http::Response<BoxBody> {
    let mut trailers = HeaderMap::new();
    trailers.insert(
        "grpc-status",
        HeaderValue::from_str(&code.to_string()).unwrap(),
    );
    if !message.is_empty() {
        trailers.insert(
            "grpc-message",
            HeaderValue::from_str(&percent_encode(message)).unwrap(),
        );
    }

    let mut frames: Vec<Result<Frame<Bytes>, Infallible>> = Vec::new();
    if let Some(p) = payload {
        frames.push(Ok(Frame::data(Bytes::from(frame_message(&p)))));
    }
    frames.push(Ok(Frame::trailers(trailers)));

    let body = BoxBody::new(StreamBody::new(futures::stream::iter(frames)));
    http::Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/grpc")
        .body(body)
        .unwrap()
}

fn empty_body() -> BoxBody {
    BoxBody::new(Full::new(Bytes::new()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::Mutex;

    #[test]
    fn parse_grpc_message_round_trip() {
        let payload = b"hello tron".to_vec();
        let framed = frame_message(&payload);
        let parsed = parse_grpc_message(&framed).unwrap();
        assert_eq!(&parsed[..], &payload[..]);
        assert!(parse_grpc_message(&framed[..4]).is_err());
    }

    #[test]
    fn frame_message_writes_length_prefix() {
        let framed = frame_message(b"abc");
        assert_eq!(&framed[..5], &[0, 0, 0, 0, 3]);
        assert_eq!(&framed[5..], b"abc");
    }

    #[test]
    fn percent_encode_grpc_message() {
        assert_eq!(percent_encode("internal error"), "internal error");
        assert_eq!(percent_encode("100% sure"), "100%25 sure");
        assert_eq!(percent_encode("caf\u{e9}"), "caf%C3%A9");
    }

    #[test]
    fn grpc_proxy_upstream_keeps_origin_and_method() {
        let (origin, method) =
            grpc_proxy_upstream("grpcs://gmain.vvwallet.org:443", "/protocol.Wallet/GetTransactionByID")
                .unwrap();
        assert_eq!(origin, "https://gmain.vvwallet.org:443");
        assert_eq!(method, "/protocol.Wallet/GetTransactionByID");
    }

    #[test]
    fn grpc_proxy_upstream_keeps_port_and_prefixes_base_path() {
        let (origin, method) = grpc_proxy_upstream(
            "grpc://127.0.0.1:50051/api",
            "/protocol.Wallet/GetTransactionByID",
        )
        .unwrap();
        assert_eq!(origin, "http://127.0.0.1:50051");
        assert_eq!(method, "/api/protocol.Wallet/GetTransactionByID");
    }

    #[test]
    fn extract_api_key_accepts_x_api_key_and_bearer() {
        let mut headers = HeaderMap::new();
        assert!(extract_api_key(&headers).is_none());
        headers.insert("x-api-key", HeaderValue::from_static("gw_test"));
        assert_eq!(extract_api_key(&headers).as_deref(), Some("gw_test"));
        let mut headers = HeaderMap::new();
        headers.insert("authorization", HeaderValue::from_static("Bearer gw_bearer"));
        assert_eq!(extract_api_key(&headers).as_deref(), Some("gw_bearer"));
    }

    #[test]
    fn grpc_response_has_ok_trailer_and_no_body_when_no_payload() {
        let resp = grpc_response(0, "", None);
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.headers()["content-type"], "application/grpc");
    }

    #[test]
    fn grpc_response_sets_status_trailer_for_errors() {
        let resp = grpc_response(16, "invalid api key", None);
        assert_eq!(resp.status(), StatusCode::OK);
    }

    type TestBody = http_body_util::combinators::BoxBody<Bytes, Infallible>;

    fn test_grpc_frame(payload: &[u8]) -> Vec<u8> {
        frame_message(payload)
    }

    async fn spawn_upstream(
        fail: bool,
    ) -> (SocketAddr, Arc<Mutex<Vec<(String, String)>>>, tokio::task::JoinHandle<()>) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let seen = Arc::new(Mutex::new(Vec::new()));
        let seen_conn = seen.clone();

        let handle = tokio::spawn(async move {
            let builder = hyper::server::conn::http2::Builder::new(TokioExecutor::new());
            loop {
                let (stream, _) = match listener.accept().await {
                    Ok(s) => s,
                    Err(_) => break,
                };
                let builder = builder.clone();
                let seen = seen_conn.clone();
                tokio::spawn(async move {
                    let io = TokioIo::new(stream);
                    let svc = service_fn(move |req| {
                        let seen = seen.clone();
                        async move {
                            for (name, value) in req.headers() {
                                if let Ok(v) = value.to_str() {
                                    seen.lock()
                                        .unwrap()
                                        .push((name.to_string(), v.to_string()));
                                }
                            }
                            if let Some(auth) = req.uri().authority() {
                                seen.lock()
                                    .unwrap()
                                    .push((":authority".to_string(), auth.to_string()));
                            }
                            let mut trailers = HeaderMap::new();
                            if fail {
                                trailers.insert(
                                    "grpc-status",
                                    HeaderValue::from_static("13"),
                                );
                                trailers.insert(
                                    "grpc-message",
                                    HeaderValue::from_static("internal error"),
                                );
                            } else {
                                trailers.insert("grpc-status", HeaderValue::from_static("0"));
                            }
                            let payload = test_grpc_frame(b"{\"result\":\"ok\"}");
                            let stream = futures::stream::iter(vec![
                                Ok::<_, Infallible>(Frame::data(Bytes::from(payload))),
                                Ok::<_, Infallible>(Frame::trailers(trailers)),
                            ]);
                            let body = TestBody::new(StreamBody::new(stream));
                            Ok::<_, Infallible>(
                                http::Response::builder()
                                    .status(200)
                                    .header("content-type", "application/grpc")
                                    .body(body)
                                    .unwrap(),
                            )
                        }
                    });
                    let _ = builder.serve_connection(io, svc).await;
                });
            }
        });

        (addr, seen, handle)
    }

    #[tokio::test]
    async fn forward_unary_returns_payload_and_forwards_headers() {
        let (addr, seen, handle) = spawn_upstream(false).await;

        let mut headers = EndpointHeaders::new();
        headers.insert("x-token".to_string(), "wt_secret".to_string());

        let url = format!("grpc://{addr}");
        let out = forward_unary(
            &url,
            "/protocol.Wallet/GetTransactionByID",
            &Bytes::from(b"request-payload".to_vec()),
            &headers,
            Duration::from_secs(5),
        )
        .await
        .expect("forward should succeed");

        assert_eq!(&out[..], b"{\"result\":\"ok\"}");

        let recorded = seen.lock().unwrap();
        let map: std::collections::HashMap<&str, &str> = recorded
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(
            map.get(":authority").copied(),
            Some(addr.to_string().as_str()),
            "端口应保留在 :authority"
        );
        assert_eq!(map.get("x-token").copied(), Some("wt_secret"));
        assert_eq!(map.get("content-type").copied(), Some("application/grpc"));

        handle.abort();
    }

    #[tokio::test]
    async fn forward_unary_surfaces_upstream_grpc_status() {
        let (addr, _seen, handle) = spawn_upstream(true).await;

        let url = format!("grpc://{addr}");
        let err = forward_unary(
            &url,
            "/protocol.Wallet/GetTransactionByID",
            &Bytes::from(b"request-payload".to_vec()),
            &EndpointHeaders::new(),
            Duration::from_secs(5),
        )
        .await
        .unwrap_err();

        assert_eq!(err.0, 13);
        assert!(err.1.contains("internal error"), "got: {}", err.1);

        handle.abort();
    }
}
