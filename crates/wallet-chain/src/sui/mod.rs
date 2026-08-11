//! Sui chain implementation.
//!
//! Sui's public JSON-RPC surface (https://docs.sui.io/sui-api-ref) is regular
//! JSON-RPC 2.0. Block-level tx syncing is checkpoint-based:
//!
//! - `sui_getLatestCheckpointSequenceNumber` → current checkpoint seq (tip)
//! - `suix_queryTransactionBlocks` → full tx list of a checkpoint (paginated)
//! - `suix_getReferenceGasPrice` → fee estimation
//!
//! Sui serializes all `BigInt` values (checkpoint seq, gas price, balances)
//! as decimal *strings*; the helpers in this module always parse both string
//! and JSON-number encodings.

use crate::provider::RpcPool;
use crate::traits::{BalanceReader, BlockSource, GasEstimator, TokenBalance, TxBroadcaster};
use async_trait::async_trait;
use base64::Engine as _;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::collections::HashMap;
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

/// Sui serializes every `BigInt`/`u64` field as a decimal string (e.g.
/// `"740"` for the reference gas price, `"44123456"` for a checkpoint
/// sequence). Some providers return a plain JSON number instead; accept both.
fn parse_bigint(v: &Value) -> Option<u64> {
    v.as_str()
        .and_then(|s| s.parse::<u64>().ok())
        .or_else(|| v.as_u64())
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
    /// Sui's canonical chain head is the latest checkpoint sequence number
    /// (monotonic, grows to millions). This is what wallet-sync uses for the
    /// cursor; the reference gas price must NOT be used as a height since it
    /// is small and non-monotonic.
    ///
    /// Most providers expose only the legacy `sui_` alias of this method (a
    /// common quirk of public fullnodes); fall back to the modern `suix_`
    /// name for providers that only expose that one.
    async fn tip(&self) -> AppResult<u64> {
        let result = match self
            .rpc("sui_getLatestCheckpointSequenceNumber", json!([]))
            .await
        {
            Ok(v) => v,
            Err(e) if is_method_not_found(&e) => {
                self.rpc("suix_getLatestCheckpointSequenceNumber", json!([]))
                    .await?
            }
            Err(e) => return Err(e),
        };
        parse_bigint(&result).ok_or_else(|| {
            AppError::Unavailable(
                "sui_getLatestCheckpointSequenceNumber returned no sequence".into(),
            )
        })
    }

    /// Fetch every transaction in a Sui checkpoint.
    ///
    /// `height` is interpreted as the checkpoint sequence number. We use a
    /// single paginated `suix_queryTransactionBlocks` filtered by that
    /// sequence — with `showInput`/`showEffects`/`showBalanceChanges` the
    /// response already carries the full `SuiTransactionBlockResponse` per tx,
    /// so there is no need for the (provider-fragile) `suix_getCheckpoint`
    /// digest lookup or N per-tx `getTransactionBlock` round trips.
    async fn fetch_block_txs(&self, height: u64) -> AppResult<Vec<NormalizedTx>> {
        let mut out: Vec<NormalizedTx> = Vec::new();
        let mut next_cursor: Option<String> = None;
        loop {
            let mut params = serde_json::json!({
                "filter": { "Checkpoint": height.to_string() },
                "options": {
                    "showInput": true,
                    "showEffects": true,
                    "showBalanceChanges": true,
                },
                "limit": 50,
            });
            if let Some(c) = &next_cursor {
                params["cursor"] = serde_json::Value::String(c.clone());
            }
            let page = self.rpc("suix_queryTransactionBlocks", params).await?;
            if let Some(data) = page.get("data").and_then(|d| d.as_array()) {
                for v in data {
                    let digest = v.get("digest").and_then(|d| d.as_str()).unwrap_or_default();
                    let tx = parse_sui_tx(digest, height, v);
                    // Skip txs that moved no coins. Gas-only contract calls
                    // and system/validator txs aren't transfers and would
                    // otherwise pollute the transfer history and stats.
                    if tx.value.raw == rust_decimal::Decimal::ZERO {
                        continue;
                    }
                    out.push(tx);
                }
            }
            let has_next = page
                .get("hasNextPage")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if !has_next {
                break;
            }
            match page.get("nextCursor").and_then(|v| v.as_str()) {
                Some(n) if Some(n) != next_cursor.as_deref() => {
                    next_cursor = Some(n.to_string());
                }
                // nextCursor missing or not advancing: stop rather than loop
                // forever on a provider that doesn't implement pagination.
                _ => break,
            }
        }
        Ok(out)
    }
}

/// True when the RPC error means the method is not exposed by the endpoint
/// (JSON-RPC `-32601`), which is how providers signal a missing `sui_`/`suix_`
/// alias. Used to fall back across the two prefixes.
fn is_method_not_found(e: &AppError) -> bool {
    e.to_string().contains("-32601")
}

/// Convert a `SuiTransactionBlockResponse` (the shape returned by
/// `suix_getTransactionBlock` and by each entry of
/// `suix_queryTransactionBlocks` with the show flags set) into a
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

    let gas_used = result
        .pointer("/effects/gasUsed/computationCost")
        .and_then(parse_mist_string);

    // `to` / `value` / `contract_address` come from `balanceChanges`: a
    // transfer is a coin type the sender spent and some other address
    // received. Gas-only SUI movement is excluded (see `derive_transfer`).
    let deltas = parse_balance_changes(result);
    let sender = from.as_ref().map(|a| a.as_str().to_string());
    let (to, value, contract_address) = match derive_transfer(&deltas, sender.as_deref(), gas_used)
    {
        Some((owner, amount, coin_type)) => (
            Some(Address::new(owner)),
            // Almost every Sui coin uses 9 decimals (MIST); non-standard
            // token decimals are not exposed via balanceChanges, so 9 is the
            // best available default.
            wallet_types::Amount::new(rust_decimal::Decimal::from(amount.unsigned_abs()), 9),
            coin_type.map(Address::new),
        ),
        None => (None, wallet_types::Amount::zero(9), None),
    };

    let method = parse_tx_method(result);

    NormalizedTx {
        hash: TxHash::new(digest),
        from,
        to,
        value,
        gas_fee: gas_used.map(|g| wallet_types::Amount::new(rust_decimal::Decimal::from(g), 9)),
        block_number: height,
        status,
        raw: result.clone(),
        contract_address,
        log_index: None,
        method,
    }
}

/// Extract a human-readable method for a Sui transaction block.
///
/// The kind is usually only `"ProgrammableTransaction"`, so the real signal
/// lives in the PTB command list (`transaction.data.transaction.transactions`).
/// `MoveCall` commands report `module::function` — the closest Sui analogue
/// to an EVM method selector (`transfer::transfer`, `token::transfer`, ...);
/// other commands use their type name (`TransferObjects`, `SplitCoins`,
/// `MergeCoins`, `Pay`, ...). When only the kind string is available, fall
/// back to it.
fn parse_tx_method(result: &Value) -> Option<String> {
    let tx = result.pointer("/transaction/data/transaction")?;
    if let Some(s) = tx.as_str() {
        return Some(s.to_string());
    }
    let commands = tx.pointer("/transactions")?.as_array()?;
    let first = commands.first()?;
    let (name, _) = first.as_object()?.iter().next()?;
    let method = match name.as_str() {
        "MoveCall" => {
            let module = first.pointer("/MoveCall/module").and_then(|v| v.as_str());
            let function = first.pointer("/MoveCall/function").and_then(|v| v.as_str());
            match (module, function) {
                (Some(m), Some(f)) => format!("{m}::{f}"),
                _ => name.to_string(),
            }
        }
        other => other.to_string(),
    };
    Some(method)
}

/// One row of a `SuiTransactionBlockResponse.balanceChanges` array.
struct BalanceDelta {
    owner: String,
    amount: i128,
    coin_type: String,
}

/// Coin type of the native Sui token. Native-coin transfers keep
/// `contract_address` empty; every other coin type is a token transfer.
const SUI_COIN_TYPE: &str = "0x2::sui::SUI";

/// Parse `balanceChanges` into per-coin deltas. Rows without a parseable
/// owner or amount are dropped — they carry no transfer signal anyway.
fn parse_balance_changes(result: &Value) -> Vec<BalanceDelta> {
    result
        .get("balanceChanges")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|ch| {
                    let owner = ch
                        .pointer("/owner/AddressOwner")
                        .and_then(|v| v.as_str())
                        .or_else(|| ch.pointer("/owner/ObjectOwner").and_then(|v| v.as_str()));
                    let amount = ch
                        .get("amount")
                        .and_then(|v| v.as_str())
                        .and_then(|s| s.parse::<i128>().ok());
                    let coin_type = ch.get("coinType").and_then(|v| v.as_str());
                    match (owner, amount, coin_type) {
                        (Some(o), Some(a), Some(c)) => Some(BalanceDelta {
                            owner: o.to_string(),
                            amount: a,
                            coin_type: c.to_string(),
                        }),
                        _ => None,
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Pick the primary transfer out of a checkpoint's `balanceChanges`.
///
/// Sui reports *every* coin balance delta — including the gas payment to
/// the validator — as `balanceChanges`. A transfer is a coin type the
/// sender spent and some other address received. The SUI movement that only
/// covers the gas fee (`sender`'s SUI spend equals the gas fee) is excluded
/// so pure contract calls don't masquerade as transfers.
///
/// Returns `(to, amount, coin_type)` for the largest qualifying delta, or
/// `None` when the tx did not move any coins beyond gas (value 0).
fn derive_transfer(
    deltas: &[BalanceDelta],
    sender: Option<&str>,
    gas_fee: Option<u64>,
) -> Option<(String, i128, Option<String>)> {
    // Total amount the sender spent per coin type (negative sums).
    let mut spent: HashMap<&str, i128> = HashMap::new();
    for d in deltas {
        if sender.is_some_and(|s| d.owner == s) && d.amount < 0 {
            *spent.entry(d.coin_type.as_str()).or_insert(0) += d.amount;
        }
    }
    let gas = gas_fee.map(|g| g as i128);
    let mut best: Option<(String, i128, Option<String>)> = None;
    for d in deltas {
        if d.amount <= 0 || sender.is_some_and(|s| d.owner == s) {
            continue;
        }
        // Only count coin types the sender actually spent (the recipient's
        // positive delta must be matched by the sender losing the same coin).
        let spent_amt = spent.get(d.coin_type.as_str()).copied().unwrap_or(0);
        if spent_amt >= 0 {
            continue;
        }
        // SUI that only pays the gas fee is income to the validator, not a
        // transfer. With a real native transfer the sender's SUI spend
        // exceeds the fee (sent amount + gas).
        if d.coin_type == SUI_COIN_TYPE && gas.is_some_and(|g| spent_amt == -g) {
            continue;
        }
        let contract = (d.coin_type != SUI_COIN_TYPE).then(|| d.coin_type.clone());
        if best.as_ref().is_none_or(|(_, amt, _)| d.amount > *amt) {
            best = Some((d.owner.clone(), d.amount, contract));
        }
    }
    best
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
            .and_then(|v| parse_bigint(&v).map(|n| n as u128));

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
    use super::{parse_bigint, parse_mist_string, parse_sui_tx, parse_tx_method, sui_tx_status, SuiChain};
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
    fn parse_bigint_handles_sui_string_and_number() {
        // Sui returns BigInts as strings; some providers send numbers.
        assert_eq!(parse_bigint(&json!("44123456")), Some(44123456));
        assert_eq!(parse_bigint(&json!("740")), Some(740));
        assert_eq!(parse_bigint(&json!(44123456u64)), Some(44123456));
        assert_eq!(parse_bigint(&json!(0)), Some(0));
        assert_eq!(parse_bigint(&json!("-1")), None);
        assert_eq!(parse_bigint(&json!("not-a-number")), None);
        assert_eq!(parse_bigint(&json!(null)), None);
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
                { "coinType": "0x2::sui::SUI", "amount": "-2500000", "owner": { "AddressOwner": "0xsender" } },
                { "coinType": "0x2::sui::SUI", "amount": "2500000", "owner": { "AddressOwner": "0xother" } }
            ]
        });
        let tx = parse_sui_tx("abc", 7, &raw);
        assert_eq!(tx.from.as_ref().unwrap().as_str(), "0xsender");
        assert_eq!(tx.to.as_ref().unwrap().as_str(), "0xother");
        assert!(tx.contract_address.is_none());
        assert_eq!(tx.value, Amount::new(rust_decimal::Decimal::from(2_500_000u64), 9));
        assert_eq!(tx.gas_fee.as_ref().unwrap().raw, rust_decimal::Decimal::from(1_000_000u64));
        assert_eq!(tx.status, TxStatus::Success);
        assert_eq!(tx.method.as_deref(), Some("ProgrammableTransaction"));
        assert_eq!(tx.block_number, 7);
    }

    #[test]
    fn sui_parse_sui_tx_token_transfer_sets_contract_and_value() {
        let raw = json!({
            "digest": "abc",
            "transaction": { "data": { "sender": "0xsender" } },
            "effects": {
                "status": { "status": "success" },
                "gasUsed": { "computationCost": "1000" }
            },
            "balanceChanges": [
                { "coinType": "0x2::sui::SUI", "amount": "-1000", "owner": { "AddressOwner": "0xsender" } },
                { "coinType": "0x2::sui::SUI", "amount": "1000", "owner": { "AddressOwner": "0xvalidator" } },
                { "coinType": "0xabc::usdc::USDC", "amount": "-500000", "owner": { "AddressOwner": "0xsender" } },
                { "coinType": "0xabc::usdc::USDC", "amount": "500000", "owner": { "AddressOwner": "0xrecipient" } }
            ]
        });
        let tx = parse_sui_tx("abc", 7, &raw);
        assert_eq!(tx.to.as_ref().unwrap().as_str(), "0xrecipient");
        assert_eq!(tx.contract_address.as_ref().unwrap().as_str(), "0xabc::usdc::USDC");
        assert_eq!(tx.value, Amount::new(rust_decimal::Decimal::from(500_000u64), 9));
    }

    #[test]
    fn sui_parse_sui_tx_gas_only_has_zero_value() {
        // Pure contract call: the sender's only SUI movement is the gas fee,
        // so no transfer is derived and value stays 0 (filtered by the syncer).
        let raw = json!({
            "digest": "abc",
            "transaction": { "data": { "sender": "0xsender" } },
            "effects": {
                "status": { "status": "success" },
                "gasUsed": { "computationCost": "1000" }
            },
            "balanceChanges": [
                { "coinType": "0x2::sui::SUI", "amount": "-1000", "owner": { "AddressOwner": "0xsender" } },
                { "coinType": "0x2::sui::SUI", "amount": "1000", "owner": { "AddressOwner": "0xvalidator" } }
            ]
        });
        let tx = parse_sui_tx("abc", 7, &raw);
        assert_eq!(tx.value.raw, rust_decimal::Decimal::ZERO);
        assert!(tx.to.is_none());
        assert!(tx.contract_address.is_none());
    }

    #[test]
    fn sui_parse_sui_tx_native_transfer_ignores_gas_income() {
        // Native SUI transfer with gas: recipient's +2000 is the transfer,
        // validator's +1000 is gas-only and must not become `to`.
        let raw = json!({
            "digest": "abc",
            "transaction": { "data": { "sender": "0xsender" } },
            "effects": {
                "status": { "status": "success" },
                "gasUsed": { "computationCost": "1000" }
            },
            "balanceChanges": [
                { "coinType": "0x2::sui::SUI", "amount": "-3000", "owner": { "AddressOwner": "0xsender" } },
                { "coinType": "0x2::sui::SUI", "amount": "2000", "owner": { "AddressOwner": "0xrecipient" } },
                { "coinType": "0x2::sui::SUI", "amount": "1000", "owner": { "AddressOwner": "0xvalidator" } }
            ]
        });
        let tx = parse_sui_tx("abc", 7, &raw);
        assert_eq!(tx.to.as_ref().unwrap().as_str(), "0xrecipient");
        assert_eq!(tx.value, Amount::new(rust_decimal::Decimal::from(2_000u64), 9));
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
    fn sui_parse_tx_method_uses_ptb_command_names() {
        let raw = json!({
            "transaction": {
                "data": {
                    "transaction": {
                        "kind": "ProgrammableTransaction",
                        "inputs": [],
                        "transactions": [
                            { "TransferObjects": { "objects": [{ "Input": 0 }], "address": { "Input": 1 } } },
                            { "SplitCoins": { "coin": { "Input": 0 }, "amounts": [{ "Input": 1 }] } }
                        ]
                    }
                }
            }
        });
        assert_eq!(parse_tx_method(&raw).as_deref(), Some("TransferObjects"));
    }

    #[test]
    fn sui_parse_tx_method_move_call_uses_module_function() {
        let raw = json!({
            "transaction": {
                "data": {
                    "transaction": {
                        "kind": "ProgrammableTransaction",
                        "inputs": [],
                        "transactions": [
                            { "MoveCall": { "package": "0x2", "module": "transfer", "function": "transfer" } }
                        ]
                    }
                }
            }
        });
        assert_eq!(parse_tx_method(&raw).as_deref(), Some("transfer::transfer"));
    }

    #[test]
    fn sui_parse_tx_method_falls_back_to_kind_string() {
        let raw = json!({
            "transaction": {
                "data": { "transaction": "ProgrammableTransaction" }
            }
        });
        assert_eq!(parse_tx_method(&raw).as_deref(), Some("ProgrammableTransaction"));
    }

    #[test]
    fn sui_stub_chain_is_constructible() {
        // Smoke test: the chain struct can be built from a stub URL. We don't
        // make any network call here; the goal is to catch regressions in
        // the `RpcPool::new` / endpoint config wiring.
        let _ = stub_chain();
    }
}
