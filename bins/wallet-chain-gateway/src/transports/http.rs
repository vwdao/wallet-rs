use std::time::Duration;

use serde_json::Value;
use wallet_error::{AppError, AppResult};

pub async fn execute(
    client: &reqwest::Client,
    url: &str,
    body: &Value,
    timeout: Duration,
) -> AppResult<Value> {
    let resp = client
        .post(url)
        .json(body)
        .timeout(timeout)
        .send()
        .await
        .map_err(|e| AppError::Unavailable(e.to_string()))?;
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
    if v.get("error").is_some() {
        return Err(AppError::Unavailable(format!("rpc error: {}", v["error"])));
    }
    Ok(v)
}
