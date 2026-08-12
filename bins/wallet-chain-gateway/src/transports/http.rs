use std::time::Duration;

use serde_json::Value;
use wallet_error::{AppError, AppResult};

use super::{rpc_error, EndpointHeaders};

pub async fn execute(
    client: &reqwest::Client,
    url: &str,
    body: &Value,
    headers: &EndpointHeaders,
    timeout: Duration,
) -> AppResult<Value> {
    let mut req = client.post(url).json(body).timeout(timeout);
    for (name, value) in headers {
        req = req.header(name, value);
    }
    let resp = req
        .send()
        .await
        .map_err(|e| AppError::Unavailable(e.to_string()))?;
    // 429 means the node is alive but over quota (common on free/public
    // gateways like Tatum). Surface it distinctly so the health checker can
    // keep the endpoint in rotation instead of marking it unhealthy.
    if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
        return Err(AppError::TooManyRequests);
    }
    if !resp.status().is_success() {
        return Err(AppError::Unavailable(format!(
            "rpc returned {}",
            resp.status()
        )));
    }
    let v: Value = resp
        .json()
        .await
        .map_err(|e| AppError::Unavailable(e.to_string()))?;
    if let Some(err) = rpc_error(&v) {
        return Err(AppError::Unavailable(format!("rpc error: {err}")));
    }
    Ok(v)
}
