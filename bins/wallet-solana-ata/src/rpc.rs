use anyhow::{anyhow, bail, Context, Result};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use reqwest::Client;
use serde_json::{json, Value};
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;
use std::str::FromStr;
use std::time::Duration;

const RETRYABLE_CODE_429: i64 = 429;
const RETRYABLE_CODE_NODE_BEHIND: i64 = -32005;

#[derive(Debug, thiserror::Error)]
pub enum CallError {
    #[error("rpc transient error: {0}")]
    Transient(String),
    #[error("rpc rejected: {0}")]
    Rejected(String),
    #[error("transaction blockhash not found")]
    BlockhashNotFound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Confirmation {
    Confirmed,
    TimedOut,
}

#[derive(Debug, Clone)]
pub struct AccountInfo {
    pub lamports: u64,
    pub owner: Pubkey,
    #[allow(dead_code)]
    pub data: Vec<u8>,
    #[allow(dead_code)]
    pub executable: bool,
}

pub struct Rpc {
    http: Client,
    url: String,
    commitment: String,
    skip_preflight: bool,
    call_delay_ms: u64,
    max_retries: u32,
}

impl Rpc {
    pub fn new(
        url: String,
        commitment: String,
        skip_preflight: bool,
        call_delay_ms: u64,
        max_retries: u32,
    ) -> Self {
        Self {
            http: Client::new(),
            url,
            commitment,
            skip_preflight,
            call_delay_ms,
            max_retries,
        }
    }

    async fn call_once(&self, method: &str, params: Value) -> Result<Value, CallError> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params,
        });
        let resp = self
            .http
            .post(&self.url)
            .json(&body)
            .send()
            .await
            .map_err(|e| CallError::Transient(e.to_string()))?;
        let value: Value = resp
            .json()
            .await
            .map_err(|e| CallError::Transient(e.to_string()))?;
        if let Some(err) = value.get("error") {
            let code = err.get("code").and_then(Value::as_i64).unwrap_or(0);
            let message = err
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("unknown rpc error")
                .to_string();
            let inner = err.pointer("/data/err").and_then(Value::as_str).unwrap_or("");
            if inner == "BlockhashNotFound" || message.contains("BlockhashNotFound") {
                return Err(CallError::BlockhashNotFound);
            }
            if code == RETRYABLE_CODE_429
                || code == RETRYABLE_CODE_NODE_BEHIND
                || message.contains("Node is behind")
            {
                return Err(CallError::Transient(message));
            }
            return Err(CallError::Rejected(format!("{code} {message}")));
        }
        value
            .get("result")
            .cloned()
            .ok_or_else(|| CallError::Rejected("rpc response missing result".into()))
    }

    async fn call(&self, method: &str, params: Value) -> Result<Value> {
        let mut attempt = 0u32;
        loop {
            if self.call_delay_ms > 0 {
                tokio::time::sleep(Duration::from_millis(self.call_delay_ms)).await;
            }
            match self.call_once(method, params.clone()).await {
                Ok(value) => return Ok(value),
                Err(CallError::Transient(msg)) => {
                    attempt += 1;
                    if attempt > self.max_retries {
                        bail!("rpc {method} failed after {} retries: {msg}", self.max_retries);
                    }
                    let backoff_ms = 500 * 2u64.pow(attempt.saturating_sub(1));
                    tracing::warn!(
                        method,
                        attempt,
                        error = %msg,
                        backoff_ms,
                        "transient rpc error, retrying"
                    );
                    tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                }
                Err(call_err) => return Err(anyhow!(call_err)),
            }
        }
    }

    pub async fn get_account(&self, addr: &Pubkey) -> Result<Option<AccountInfo>> {
        let r = self
            .call(
                "getAccountInfo",
                json!([addr.to_string(), { "encoding": "base64", "commitment": self.commitment }]),
            )
            .await?;
        let val = &r["value"];
        if val.is_null() {
            return Ok(None);
        }
        let owner = val
            .get("owner")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("getAccountInfo {addr}: missing owner"))?
            .parse()
            .context("parse owner pubkey")?;
        let lamports = val.get("lamports").and_then(Value::as_u64).unwrap_or(0);
        let executable = val.get("executable").and_then(Value::as_bool).unwrap_or(false);
        let data = parse_data(&val["data"])?;
        Ok(Some(AccountInfo {
            lamports,
            owner,
            data,
            executable,
        }))
    }

    pub async fn get_latest_blockhash(&self) -> Result<Hash> {
        let r = self
            .call(
                "getLatestBlockhash",
                json!([{ "commitment": self.commitment }]),
            )
            .await?;
        let bh = r
            .pointer("/value/blockhash")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("getLatestBlockhash: missing blockhash"))?;
        Hash::from_str(bh).context("parse blockhash")
    }

    pub async fn send_transaction(&self, tx: &Transaction) -> Result<Signature, CallError> {
        let raw = bincode::serialize(tx).map_err(|e| CallError::Rejected(e.to_string()))?;
        let encoded = BASE64_STANDARD.encode(&raw);
        let r = self
            .call_once(
                "sendTransaction",
                json!([encoded, {
                    "encoding": "base64",
                    "preflightCommitment": self.commitment,
                    "skipPreflight": self.skip_preflight,
                }]),
            )
            .await?;
        let sig = r
            .as_str()
            .ok_or_else(|| CallError::Rejected("sendTransaction returned unexpected payload".into()))?;
        Signature::from_str(sig).map_err(|e| CallError::Rejected(e.to_string()))
    }

    pub async fn confirm(&self, sig: &Signature, timeout: Duration) -> Result<Confirmation> {
        let deadline = tokio::time::Instant::now() + timeout;
        loop {
            let r = self
                .call(
                    "getSignatureStatuses",
                    json!([[sig.to_string()], { "searchTransactionHistory": true }]),
                )
                .await
                .context("getSignatureStatuses")?;
            if let Some(entry) = r["value"][0].as_object() {
                if let Some(err) = entry.get("err") {
                    if !err.is_null() {
                        bail!("transaction {sig} failed: {err}");
                    }
                }
                match entry.get("confirmationStatus").and_then(Value::as_str) {
                    Some("finalized") | Some("confirmed") => return Ok(Confirmation::Confirmed),
                    _ => {}
                }
            }
            if tokio::time::Instant::now() >= deadline {
                return Ok(Confirmation::TimedOut);
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

fn parse_data(data: &Value) -> Result<Vec<u8>> {
    let s = match data {
        Value::String(s) => s.clone(),
        Value::Array(items) => items
            .first()
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| anyhow!("unexpected account data array"))?,
        _ => bail!("unexpected account data: {data}"),
    };
    BASE64_STANDARD.decode(s).context("decode account data")
}