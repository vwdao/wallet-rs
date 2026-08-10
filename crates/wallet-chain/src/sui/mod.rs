//! Sui chain implementation.
//!
//! Sui's public JSON-RPC surface (https://docs.sui.io/sui-api-ref) is regular
//! JSON-RPC 2.0, but the methods we wire up here are the read-side of the
//! wallet API only:
//!
//! - `suix_getBalance`            → native MIST balance for an address
//! - `suix_getCoinBalance`        → token balance (coin type) for an address
//! - `suix_getReferenceGasPrice`  → tip-side metadata (used as a soft "tip")
//! - `sui_executeTransactionBlock` → broadcast a signed tx BCS blob (base64)
//!
//! Block-level tx syncing (`suix_queryTransactionBlocks`,
//! `suix_getCheckpoint`) is intentionally not wired — Sui's checkpoint model
//! is materially different from account chains and is out of scope for the
//! initial chain-gateway pass.

use crate::provider::RpcPool;
use crate::traits::{BalanceReader, BlockSource, GasEstimator, TokenBalance, TxBroadcaster};
use async_trait::async_trait;
use base64::Engine as _;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::sync::Arc;
use wallet_config::ChainRuntimeConfig;
use wallet_error::{AppError, AppResult};
use wallet_types::{
    Address, Amount, ChainIndex, GasEstimate, GasEstimateRequest, NormalizedTx, TxHash, TxStatus,
};

#[derive(Debug)]
pub struct SuiChain {
    pub chain_index: ChainIndex,
    pub pool: RpcPool,
}

impl SuiChain {
    pub fn new(cfg: &ChainRuntimeConfig) -> AppResult<Arc<Self>> {
        Ok(Arc::new(Self {
            chain_index: ChainIndex(cfg.chain_index),
            pool: RpcPool::new(cfg.endpoints.clone())?,
        }))
    }

    /// Send a JSON-RPC 2.0 request to the next healthy Sui endpoint.
    async fn rpc(&self, method: &str, params: Value) -> AppResult<Value> {
        let url = self.pool.next_url()?.to_string();
        let body = json!({ "jsonrpc": "2.0", "id": 1, "method": method, "params": params });
        let resp = self
            .pool
            .client()
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                self.pool.mark_failure(&url);
                AppError::Unavailable(format!("sui rpc {url}: {e}"))
            })?;
        let status = resp.status();
        let v: Value = resp.json().await.map_err(|e| {
            self.pool.mark_failure(&url);
            AppError::Unavailable(format!(
                "sui rpc {url}: non-JSON response (http {status}): {e}"
            ))
        })?;
        // Treat any non-2xx as a hard error so the syncer surfaces the real
        // cause (auth, network, upstream 5xx) instead of silently returning
        // tip=0. This is critical for chain-gateway fronting: 401/403
        // responses carry an `{"error": ...}` body but a 2xx status.
        if !status.is_success() {
            self.pool.mark_failure(&url);
            return Err(AppError::Unavailable(format!(
                "sui rpc {url}: http {status}: {v}"
            )));
        }
        // Sui embeds errors as `{"jsonrpc":"2.0","error":{"code":..,"message":..}}`
        // (top-level), not under a `result` envelope.
        if let Some(err) = v.get("error").filter(|e| !e.is_null()) {
            self.pool.mark_failure(&url);
            return Err(AppError::Unavailable(format!("sui rpc {url}: {err}")));
        }
        self.pool.mark_success(&url);
        Ok(v.get("result").cloned().unwrap_or(Value::Null))
    }
}

#[async_trait]
impl BalanceReader for SuiChain {
    /// Native SUI balance in MIST (1 SUI = 10^9 MIST).
    async fn native_balance(&self, addr: &Address) -> AppResult<Amount> {
        let result = self
            .rpc(
                "suix_getBalance",
                json!({ "owner": addr.as_str(), "coinType": "0x2::sui::SUI" }),
            )
            .await?;
        let mist = result
            .get("totalBalance")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<u64>().ok())
            .or_else(|| result.get("totalBalance").and_then(|v| v.as_u64()))
            .unwrap_or(0);
        Ok(Amount::new(Decimal::from(mist), 9))
    }
}

#[async_trait]
impl TokenBalance for SuiChain {
    /// Token balance (any coin type, e.g. `0x...::coin::COIN`) for `wallet`.
    async fn token_balance(&self, wallet: &Address, token: &Address) -> AppResult<Amount> {
        let result = self
            .rpc(
                "suix_getCoinBalance",
                json!({
                    "owner": wallet.as_str(),
                    "coinType": token.as_str(),
                }),
            )
            .await?;
        let raw = result
            .get("totalBalance")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<u64>().ok())
            .or_else(|| result.get("totalBalance").and_then(|v| v.as_u64()))
            .unwrap_or(0);
        // Caller picks the right `decimals` from the token registry; default
        // to 9 (most Sui tokens use 9) so the value is at least usable.
        Ok(Amount::new(Decimal::from(raw), 9))
    }
}

#[async_trait]
impl TxBroadcaster for SuiChain {
    /// Broadcast a signed transaction block. `raw` is the BCS-serialized
    /// transaction bytes (Base64 per the Sui JSON-RPC contract).
    async fn send_raw(&self, raw: &[u8]) -> AppResult<TxHash> {
        let tx_bytes = base64::engine::general_purpose::STANDARD.encode(raw);
        let result = self
            .rpc(
                "sui_executeTransactionBlock",
                json!({
                    "txBytes": tx_bytes,
                    "signatures": [],
                    "options": { "showEffects": true, "showEvents": true },
                    "requestType": "WaitForLocalExecution",
                }),
            )
            .await?;
        let digest = result
            .get("digest")
            .and_then(|d| d.as_str())
            .unwrap_or_default()
            .to_string();
        let status = result
            .pointer("/effects/status/status")
            .and_then(|s| s.as_str())
            .unwrap_or("success");
        if !matches!(status, "success" | "Success") {
            return Err(AppError::Unavailable(format!(
                "sui executeTransactionBlock failed: {status}"
            )));
        }
        Ok(TxHash::new(digest))
    }
}

#[async_trait]
impl BlockSource for SuiChain {
    /// Sui checkpoints are the canonical "tip" — used as a soft height so the
    /// chain-gateway can route around lagging endpoints. The actual value
    /// returned is the reference gas price (small u64) since it is a cheap
    /// and universally-available RPC. Callers needing a real checkpoint
    /// number should add a dedicated `suix_getLatestCheckpointSequenceNumber`
    /// call later.
    async fn tip(&self) -> AppResult<u64> {
        let result = self.rpc("suix_getReferenceGasPrice", json!({})).await?;
        Ok(result.as_u64().unwrap_or(0))
    }

    /// Fetch the list of transaction digests in a Sui checkpoint and enrich
    /// each with `suix_getTransactionBlock` so the rest of the sync pipeline
    /// can use the same `NormalizedTx` shape as the account-based chains.
    ///
    /// `height` is interpreted as the checkpoint sequence number
    /// (`suix_getCheckpoint` accepts it as a positional argument).
    async fn fetch_block_txs(&self, height: u64) -> AppResult<Vec<NormalizedTx>> {
        // 1. Get the checkpoint by sequence number.
        let checkpoint = self
            .rpc("suix_getCheckpoint", json!([height]))
            .await?;
        let digest_opt = checkpoint.get("digest").and_then(|d| d.as_str());
        let digest = match digest_opt {
            Some(d) => d.to_string(),
            None => return Ok(Vec::new()),
        };

        // 2. Pull the digests in the checkpoint (paginated by 50 to stay
        //    under most providers' request caps).
        let digests: Vec<String> = self
            .rpc(
                "suix_queryTransactionBlocks",
                json!({
                    "filter": { "Checkpoint": digest },
                    "options": { "showInput": false, "showEffects": true },
                    "limit": 50,
                }),
            )
            .await
            .and_then(|r| {
                r.get("data")
                    .and_then(|d| d.as_array())
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v.get("digest").and_then(|d| d.as_str()))
                            .map(|s| s.to_string())
                            .collect()
                    })
                    .ok_or_else(|| AppError::Unavailable("missing data array".into()))
            })?;

        // 3. Resolve each digest in detail. We collect in order so the index
        //    in the returned Vec matches the position in the checkpoint.
        let mut out = Vec::with_capacity(digests.len());
        for (idx, d) in digests.into_iter().enumerate() {
            match self.fetch_sui_tx(&d, height, idx).await {
                Ok(tx) => out.push(tx),
                Err(e) => {
                    // Don't abort the whole batch on a single miss — the
                    // chain-gateway already accounts for partial sync via
                    // `rpc_block_height`.
                    tracing::warn!(error = %e, digest = %d, "suix_getTransactionBlock failed");
                }
            }
        }
        Ok(out)
    }
}

impl SuiChain {
    /// Resolve a single Sui transaction digest into a `NormalizedTx`.
    async fn fetch_sui_tx(&self, digest: &str, height: u64, idx: usize) -> AppResult<NormalizedTx> {
        let result = self
            .rpc(
                "suix_getTransactionBlock",
                json!({
                    "digest": digest,
                    "options": {
                        "showInput": true,
                        "showEffects": true,
                        "showEvents": false,
                        "showObjectChanges": false,
                        "showBalanceChanges": true,
                    },
                }),
            )
            .await?;
        // Pure function — the rest of the conversion is in `parse_sui_tx`,
        // which is also exercised by unit tests with a hand-built JSON.
        let _ = idx;
        Ok(parse_sui_tx(digest, height, &result))
    }
}

/// Convert the JSON shape returned by `suix_getTransactionBlock` into a
/// `NormalizedTx`. Pure function — no I/O — so it can be tested without
/// mocking the RPC pool.
fn parse_sui_tx(digest: &str, height: u64, result: &Value) -> NormalizedTx {
    let status = result
        .pointer("/effects/status/status")
        .and_then(|v| v.as_str())
        .unwrap_or("success");
    let status = sui_tx_status(status);

    let from = result
        .pointer("/transaction/data/sender")
        .and_then(|v| v.as_str())
        .map(Address::new);

    let mut value = wallet_types::Amount::zero(9);
    if let Some(changes) = result.get("balanceChanges").and_then(|v| v.as_array()) {
        let mut negative: i128 = 0;
        for ch in changes {
            let owner = ch
                .pointer("/owner/AddressOwner")
                .and_then(|v| v.as_str())
                .or_else(|| ch.pointer("/owner/ObjectOwner").and_then(|v| v.as_str()));
            if owner.as_deref() == from.as_ref().map(Address::as_str) {
                if let Some(amount) = ch.get("amount").and_then(|v| v.as_str()) {
                    if let Ok(n) = amount.parse::<i128>() {
                        if n < 0 {
                            negative += n;
                        }
                    }
                }
            }
        }
        if negative < 0 {
            let abs = (-negative) as u128;
            value = wallet_types::Amount::new(rust_decimal::Decimal::from(abs), 9);
        }
    }

    let gas_used = result
        .pointer("/effects/gasUsed/computationCost")
        .and_then(parse_mist_string);

    let method = result
        .pointer("/transaction/data/transaction")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    NormalizedTx {
        hash: TxHash::new(digest),
        from,
        to: None,
        value,
        gas_fee: gas_used.map(|g| wallet_types::Amount::new(rust_decimal::Decimal::from(g), 9)),
        block_number: height,
        status,
        raw: result.clone(),
        contract_address: None,
        log_index: None,
        method,
    }
}

/// Parse a "12345" / "-123" string into a u64 (negative clamped to 0).
fn parse_mist_string(v: &serde_json::Value) -> Option<u64> {
    if let Some(s) = v.as_str() {
        if let Ok(n) = s.parse::<i128>() {
            return Some(if n < 0 { 0 } else { n as u64 });
        }
        return None;
    }
    v.as_u64()
}

/// Mark a Sui tx as success/failed based on the `effects.status.status` field.
#[allow(dead_code)]
pub(crate) fn sui_tx_status(status: &str) -> TxStatus {
    if status.eq_ignore_ascii_case("success") {
        TxStatus::Success
    } else {
        TxStatus::Failed
    }
}

#[async_trait]
impl GasEstimator for SuiChain {
    /// Estimate gas for a Sui transaction by dry-running it. The full signed
    /// transaction block must be provided in `tx.extras.txBytes` (base64 BCS),
    /// along with the matching `extras.signatures` (base64 array).
    ///
    /// Sui exposes a separate `suix_getReferenceGasPrice` call for the gas
    /// price (in MIST per computation unit); the dry-run response only carries
    /// the units and total cost, so we query both in parallel.
    async fn estimate_gas(&self, tx: &GasEstimateRequest) -> AppResult<GasEstimate> {
        let extras = tx.extras.as_ref().ok_or_else(|| {
            AppError::InvalidArgument(
                "sui estimate_gas: extras.txBytes is required (sign the full tx first)"
                    .into(),
            )
        })?;
        let tx_bytes = extras
            .get("txBytes")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                AppError::InvalidArgument("sui estimate_gas: extras.txBytes missing".into())
            })?;
        let signatures: Vec<String> = extras
            .get("signatures")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|s| s.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        // Fire the reference gas price query and the dry-run in parallel.
        let (gas_price_res, dry_run_res) = tokio::join!(
            self.rpc("suix_getReferenceGasPrice", json!([])),
            self.rpc(
                "suix_dryRunTransactionBlock",
                json!({
                    "txBytes": tx_bytes,
                    "signatures": signatures,
                }),
            ),
        );

        let gas_price = gas_price_res
            .ok()
            .and_then(|v| v.as_u64().map(|n| n as u128));

        let dry = dry_run_res?;
        let status = dry
            .pointer("/effects/status/status")
            .and_then(|v| v.as_str())
            .unwrap_or("failure");
        if !matches!(status, "success" | "Success") {
            let err = dry
                .pointer("/effects/status/error")
                .cloned()
                .unwrap_or(serde_json::Value::Null);
            return Err(AppError::Unavailable(format!(
                "suix_dryRunTransactionBlock: {status} ({}); tx would fail",
                err
            )));
        }

        let computation = dry
            .pointer("/effects/gasUsed/computationCost")
            .and_then(parse_mist_string);
        let storage = dry
            .pointer("/effects/gasUsed/storageCost")
            .and_then(parse_mist_string);
        let rebate = dry
            .pointer("/effects/gasUsed/storageRebate")
            .and_then(parse_mist_string);

        let gas_used = computation;
        let fee_native: Option<u128> = match (computation, storage, rebate) {
            (Some(c), Some(s), Some(r)) => Some(c as u128 + s as u128 - r as u128),
            (Some(c), Some(s), None) => Some(c as u128 + s as u128),
            (Some(c), None, _) => Some(c as u128),
            _ => None,
        };

        // Sui's gas unit is "compute units" (1 unit = 1 MIST at the reference
        // price). We mirror that into `gas_limit` so callers that only know
        // the EVM shape still see a sensible upper bound.
        let gas_limit = gas_used.unwrap_or(0);

        Ok(GasEstimate {
            gas_limit,
            max_fee_per_gas: gas_price,
            max_priority_fee_per_gas: None,
            gas_price,
            fee_native,
            gas_used,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_mist_string, parse_sui_tx, sui_tx_status, SuiChain};
    use serde_json::json;
    use wallet_types::{Amount, TxStatus};

    fn stub_chain() -> SuiChain {
        SuiChain {
            chain_index: wallet_types::ChainIndex(784),
            pool: crate::provider::RpcPool::new(vec![wallet_config::RpcEndpoint {
                url: "https://example.org".into(),
                weight: 1,
            }])
            .unwrap(),
        }
    }

    #[test]
    fn sui_tx_status_success() {
        assert_eq!(sui_tx_status("success"), TxStatus::Success);
        assert_eq!(sui_tx_status("SUCCESS"), TxStatus::Success);
    }

    #[test]
    fn sui_tx_status_failure() {
        assert_eq!(sui_tx_status("failure"), TxStatus::Failed);
        assert_eq!(sui_tx_status(""), TxStatus::Failed);
    }

    #[test]
    fn parse_mist_string_handles_string_and_number() {
        assert_eq!(parse_mist_string(&json!("12345")), Some(12345));
        assert_eq!(parse_mist_string(&json!("-1")), Some(0)); // negative clamped
        assert_eq!(parse_mist_string(&json!(99u64)), Some(99));
        assert_eq!(parse_mist_string(&json!("garbage")), None);
    }

    #[test]
    fn sui_parse_sui_tx_extracts_sender_balance_change_and_ptb_kind() {
        let raw = json!({
            "digest": "abc",
            "transaction": {
                "data": {
                    "sender": "0xsender",
                    "transaction": "ProgrammableTransaction"
                }
            },
            "effects": {
                "status": { "status": "success" },
                "gasUsed": {
                    "computationCost": "1000000",
                    "storageCost": "500000",
                    "storageRebate": "100000"
                }
            },
            "balanceChanges": [
                { "owner": { "AddressOwner": "0xsender" }, "amount": "-2500000" },
                { "owner": { "AddressOwner": "0xother" }, "amount": "2500000" }
            ]
        });
        let tx = parse_sui_tx("abc", 7, &raw);
        assert_eq!(tx.from.as_ref().unwrap().as_str(), "0xsender");
        assert_eq!(tx.value, Amount::new(rust_decimal::Decimal::from(2_500_000u64), 9));
        assert_eq!(tx.gas_fee.as_ref().unwrap().raw, rust_decimal::Decimal::from(1_000_000u64));
        assert_eq!(tx.status, TxStatus::Success);
        assert_eq!(tx.method.as_deref(), Some("ProgrammableTransaction"));
        assert_eq!(tx.block_number, 7);
    }

    #[test]
    fn sui_parse_sui_tx_marks_failed_effects() {
        let raw = json!({
            "digest": "x",
            "transaction": { "data": { "sender": "0x" } },
            "effects": { "status": { "status": "failure" } }
        });
        let tx = parse_sui_tx("x", 1, &raw);
        assert_eq!(tx.status, TxStatus::Failed);
    }

    #[test]
    fn sui_stub_chain_is_constructible() {
        // Smoke test: the chain struct can be built from a stub URL. We don't
        // make any network call here; the goal is to catch regressions in
        // the `RpcPool::new` / endpoint config wiring.
        let _ = stub_chain();
    }
}
