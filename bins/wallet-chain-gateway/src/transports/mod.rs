mod grpc;
mod http;
mod tcp;
mod ws;
mod ws_bridge;

use std::time::Duration;

use salvo::websocket::WebSocket;
use serde_json::Value;
use wallet_error::AppResult;

use crate::protocol::{parse_endpoint_url, EndpointProtocol};

pub async fn execute_unary(
    http: &reqwest::Client,
    url: &str,
    body: &Value,
    timeout: Duration,
) -> AppResult<Value> {
    let (protocol, parsed) = parse_endpoint_url(url)?;
    match protocol {
        EndpointProtocol::Http => http::execute(http, url, body, timeout).await,
        EndpointProtocol::WebSocket => ws::execute(url, body, timeout).await,
        EndpointProtocol::Grpc => grpc::execute(http, url, body, timeout).await,
        EndpointProtocol::Tcp => tcp::execute(&parsed, body, timeout).await,
    }
}

pub async fn tunnel(
    client_ws: WebSocket,
    url: &str,
) -> AppResult<()> {
    let (protocol, parsed) = parse_endpoint_url(url)?;
    if !protocol.supports_ws_tunnel() {
        return Err(wallet_error::AppError::InvalidArgument(format!(
            "endpoint protocol does not support websocket tunnel: {url}"
        )));
    }
    match protocol {
        EndpointProtocol::WebSocket => ws::tunnel(client_ws, url).await,
        EndpointProtocol::Grpc => grpc::tunnel(client_ws, url).await,
        EndpointProtocol::Tcp => tcp::tunnel(client_ws, &parsed).await,
        EndpointProtocol::Http => unreachable!(),
    }
}

pub async fn probe(
    http: &reqwest::Client,
    url: &str,
    method: &str,
    family: &str,
) -> AppResult<(i32, Option<i64>)> {
    let (protocol, parsed) = parse_endpoint_url(url)?;
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": method,
        "params": [],
    });
    let start = std::time::Instant::now();
    let result = match protocol {
        EndpointProtocol::Http => {
            let v = http::execute(http, url, &body, Duration::from_secs(5)).await?;
            parse_probe_result(&v, family)
        }
        EndpointProtocol::WebSocket => {
            let v = ws::execute(url, &body, Duration::from_secs(5)).await?;
            parse_probe_result(&v, family)
        }
        EndpointProtocol::Grpc => {
            grpc::probe_connect(url, Duration::from_secs(5)).await?;
            Ok(None)
        }
        EndpointProtocol::Tcp => {
            let v = tcp::execute(&parsed, &body, Duration::from_secs(5)).await?;
            parse_probe_result(&v, family)
        }
    };
    let latency = start.elapsed().as_millis() as i32;
    match result {
        Ok(block_height) => Ok((latency, block_height)),
        Err(e) => Err(e),
    }
}

fn parse_probe_result(v: &Value, family: &str) -> wallet_error::AppResult<Option<i64>> {
    if v.get("error").is_some() {
        return Err(wallet_error::AppError::Unavailable(
            "rpc probe returned error".into(),
        ));
    }
    let result = v.get("result").cloned().unwrap_or(Value::Null);
    Ok(parse_block_height(&result, family))
}

fn parse_block_height(result: &Value, family: &str) -> Option<i64> {
    match family {
        "evm" => {
            let hex = result.as_str()?.trim_start_matches("0x");
            i64::from_str_radix(hex, 16).ok()
        }
        "bitcoin" => result.as_i64(),
        "solana" => result
            .as_i64()
            .or_else(|| result.get("value").and_then(|v| v.as_i64())),
        "tron" => result
            .get("block_header")
            .and_then(|h| h.get("raw_data"))
            .and_then(|r| r.get("number"))
            .and_then(|n| n.as_i64())
            .or_else(|| result.as_i64()),
        _ => {
            let hex = result.as_str()?.trim_start_matches("0x");
            i64::from_str_radix(hex, 16).ok()
        }
    }
}
