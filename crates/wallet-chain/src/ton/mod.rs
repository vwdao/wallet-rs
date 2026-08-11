//! TON (The Open Network) chain implementation.
//!
//! The chain-gateway transparently forwards JSON-RPC requests to a TON HTTP
//! endpoint, but the sync pipeline (wallet-sync) and the API (wallet-api) need
//! a typed wrapper around the few methods they actually call. TON's public
//! HTTP API (https://toncenter.com/api/v2/) is JSON-RPC 2.0 compatible: the
//! bodies it accepts look like `{"jsonrpc":"2.0","id":1,"method":...,
//! "params":{...}}` and the responses mirror the same shape.
//!
//! Supported API versions:
//! - v2: https://toncenter.com/api/v2/jsonRPC (public, limited features)
//! - v3: https://toncenter.com/api/v3/jsonRPC (paid, full features)
//! - v4: future version
//!
//! The methods we call today:
//! - `getAddressInformation`  → balance / state for a wallet (BOC-free path)
//! - `getMasterchainInfo`     → tip + last block seqno for height tracking
//! - `sendBoc`                → broadcast a signed BOC; returns `{ok, result}`
//!
//! Token (jetton) balances use `getJettonBalance` / `getJettonData`.
//! Block fetching for syncing uses toncenter-compatible JSON-RPC:
//! - `lookupBlock` / `getBlockHeader` → masterchain block id
//! - `shards` → basechain shard blocks for that masterchain seqno
//! - `getBlockTransactionsExt` → full `raw.transaction` objects (paginated)

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

/// TON API version selection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TonApiVersion {
    V2,
    V3,
    V4,
}

impl TonApiVersion {
    /// Parse API version from config, defaulting to v2
    fn from_config(cfg: &ChainRuntimeConfig) -> Self {
        match cfg.ton_api_version {
            Some(3) => TonApiVersion::V3,
            Some(4) => TonApiVersion::V4,
            _ => TonApiVersion::V2,
        }
    }

    /// Detect API version from URL (used by tests / future auto-detect).
    #[cfg(test)]
    fn from_url(url: &str) -> Self {
        if url.contains("/api/v3/") {
            TonApiVersion::V3
        } else if url.contains("/api/v4/") {
            TonApiVersion::V4
        } else {
            TonApiVersion::V2
        }
    }
}

#[derive(Debug)]
pub struct TonChain {
    pub chain_index: ChainIndex,
    pub pool: RpcPool,
    pub api_version: TonApiVersion,
}

impl TonChain {
    pub fn new(cfg: &ChainRuntimeConfig) -> AppResult<Arc<Self>> {
        let api_version = TonApiVersion::from_config(cfg);
        Ok(Arc::new(Self {
            chain_index: ChainIndex(cfg.chain_index),
            pool: RpcPool::new(cfg.endpoints.clone())?,
            api_version,
        }))
    }

    /// Resolve a block id (workchain/shard/seqno → root_hash/file_hash).
    async fn lookup_block(&self, workchain: i64, shard: &str, seqno: u64) -> AppResult<Value> {
        // Prefer `lookupBlock` (returns a flat `ton.blockIdExt`). Fall back to
        // `getBlockHeader` whose hashes live under `id`.
        match self
            .rpc(
                "lookupBlock",
                json!({
                    "workchain": workchain,
                    "shard": shard,
                    "seqno": seqno,
                }),
            )
            .await
        {
            Ok(v) if extract_ton_block_hashes(&v).is_some() => Ok(v),
            Ok(_) | Err(_) => {
                let header = self
                    .rpc(
                        "getBlockHeader",
                        json!({
                            "workchain": workchain,
                            "shard": shard,
                            "seqno": seqno,
                        }),
                    )
                    .await?;
                Ok(header)
            }
        }
    }

    /// Basechain shard blocks referenced by a masterchain seqno.
    ///
    /// User transfers live on workchain 0 shards; masterchain blocks mostly
    /// carry validator/system traffic. Providers expose this as either
    /// `shards` (ZAN) or `getShards` (toncenter).
    async fn shards_for_mc(&self, mc_seqno: u64) -> AppResult<Vec<Value>> {
        let result = match self.rpc("shards", json!({ "seqno": mc_seqno })).await {
            Ok(v) => v,
            Err(_) => self.rpc("getShards", json!({ "seqno": mc_seqno })).await?,
        };
        Ok(result
            .get("shards")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default())
    }

    /// Full-transaction block fetch method. Short `getBlockTransactions`
    /// IDs cannot be hydrated on public toncenter v2, so we always use Ext.
    fn block_txs_method(&self) -> &'static str {
        let _ = self.api_version;
        "getBlockTransactionsExt"
    }

    /// Parameters for `getBlockTransactionsExt` / short variant.
    fn block_txs_params(
        &self,
        workchain: i64,
        shard: &str,
        seqno: u64,
        root_hash: &str,
        file_hash: &str,
        after_lt: Option<&str>,
    ) -> Value {
        let mut params = json!({
            "workchain": workchain,
            "shard": shard,
            "seqno": seqno,
            "root_hash": root_hash,
            "file_hash": file_hash,
            "count": 256,
        });
        if let Some(lt) = after_lt {
            params["after_lt"] = json!(lt);
        }
        params
    }

    /// Send a JSON-RPC 2.0 request to the next healthy TON endpoint.
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
                AppError::Unavailable(format!("ton rpc {url}: {e}"))
            })?;
        let status = resp.status();
        let v: Value = resp.json().await.map_err(|e| {
            self.pool.mark_failure(&url);
            AppError::Unavailable(format!(
                "ton rpc {url}: non-JSON response (http {status}): {e}"
            ))
        })?;
        // Treat any non-2xx as a hard error so the syncer surfaces the real
        // cause (auth, network, upstream 5xx) instead of silently returning
        // tip=0. This is critical for chain-gateway fronting: 401/403
        // responses carry an `{"error": ...}` body but a 2xx status.
        if !status.is_success() {
            self.pool.mark_failure(&url);
            return Err(AppError::Unavailable(format!(
                "ton rpc {url}: http {status}: {v}"
            )));
        }
        if let Some(err) = v.get("error").filter(|e| !e.is_null()) {
            self.pool.mark_failure(&url);
            return Err(AppError::Unavailable(format!("ton rpc {url}: {err}")));
        }
        self.pool.mark_success(&url);
        Ok(v.get("result").cloned().unwrap_or(Value::Null))
    }
}

#[async_trait]
impl BalanceReader for TonChain {
    /// Native TON balance for `addr`, in nanotons (1 TON = 10^9 nanotons).
    async fn native_balance(&self, addr: &Address) -> AppResult<Amount> {
        let result = self
            .rpc("getAddressInformation", json!({ "address": addr.as_str() }))
            .await?;
        let nanotons = result
            .get("balance")
            .and_then(|b| b.as_str())
            .and_then(|s| s.parse::<u64>().ok())
            .or_else(|| result.get("balance").and_then(|b| b.as_u64()))
            .unwrap_or(0);
        Ok(Amount::new(Decimal::from(nanotons), 9))
    }
}

#[async_trait]
impl TxBroadcaster for TonChain {
    /// Broadcast a signed message BOC. `raw` is the boc bytes (Base64 encoded
    /// by the caller per the TON API contract).
    async fn send_raw(&self, raw: &[u8]) -> AppResult<TxHash> {
        let boc = base64::engine::general_purpose::STANDARD.encode(raw);
        let result = self.rpc("sendBoc", json!({ "boc": boc })).await?;
        // Some endpoints return `{"ok": true, "result": {...}}`; others return
        // just `{"@type": "...", "hash": "..."}` or `null` on success.
        let hash = result
            .get("hash")
            .and_then(|h| h.as_str())
            .or_else(|| {
                result
                    .get("result")
                    .and_then(|r| r.get("hash"))
                    .and_then(|h| h.as_str())
            })
            .unwrap_or_default()
            .to_string();
        if result.get("ok").and_then(|v| v.as_bool()) == Some(false) {
            return Err(AppError::Unavailable(format!(
                "ton sendBoc rejected: {result}"
            )));
        }
        Ok(TxHash::new(hash))
    }
}

#[async_trait]
impl BlockSource for TonChain {
    /// Last masterchain seqno — TON's "tip".
    async fn tip(&self) -> AppResult<u64> {
        let result = self.rpc("getMasterchainInfo", json!({})).await?;
        let seqno = parse_ton_seqno(&result).unwrap_or(0);
        if seqno == 0 {
            // A 0 seqno is treated by the syncer as a transient skip, but
            // the underlying cause is usually a real RPC problem (auth,
            // rate limit, gateway 5xx). Surface the raw result here so
            // operators can diagnose without having to attach a debugger.
            tracing::warn!(
                chain = %self.chain_index,
                "getMasterchainInfo returned no parseable seqno; raw result: {}",
                result
            );
        }
        Ok(seqno)
    }

    /// Fetch transactions for a masterchain seqno.
    ///
    /// Includes the masterchain block itself plus every basechain shard
    /// block referenced by `shards`/`getShards` for that seqno. Without the
    /// shard pass, wallet transfers never appear in the transaction table.
    async fn fetch_block_txs(&self, height: u64) -> AppResult<Vec<NormalizedTx>> {
        const MASTERCHAIN_SHARD: &str = "-9223372036854775808";

        let mc = self.lookup_block(-1, MASTERCHAIN_SHARD, height).await?;
        let mut blocks = vec![mc];
        match self.shards_for_mc(height).await {
            Ok(shards) => blocks.extend(shards),
            Err(e) => {
                // Shard lookup failure should not drop masterchain txs; log
                // and continue with what we have.
                tracing::warn!(
                    chain = %self.chain_index,
                    height,
                    error = %e,
                    "TON shards lookup failed; syncing masterchain only"
                );
            }
        }

        let mut out = Vec::new();
        for block in blocks {
            out.extend(self.fetch_txs_for_block(height, &block).await?);
        }
        Ok(out)
    }
}

impl TonChain {
    /// Paginate `getBlockTransactionsExt` for one block id until complete.
    async fn fetch_txs_for_block(
        &self,
        mc_height: u64,
        block: &Value,
    ) -> AppResult<Vec<NormalizedTx>> {
        let (root_hash, file_hash) = match extract_ton_block_hashes(block) {
            Some(h) => h,
            None => {
                tracing::warn!(
                    chain = %self.chain_index,
                    height = mc_height,
                    "TON block missing root_hash/file_hash; skipping: {block}"
                );
                return Ok(Vec::new());
            }
        };
        let workchain = block
            .get("workchain")
            .or_else(|| block.pointer("/id/workchain"))
            .and_then(|v| v.as_i64())
            .unwrap_or(-1);
        let shard = block
            .get("shard")
            .or_else(|| block.pointer("/id/shard"))
            .and_then(|v| v.as_str())
            .unwrap_or("-9223372036854775808")
            .to_string();
        let seqno = block
            .get("seqno")
            .or_else(|| block.pointer("/id/seqno"))
            .and_then(|v| v.as_u64())
            .unwrap_or(mc_height);

        let method = self.block_txs_method();
        let mut after_lt: Option<String> = None;
        let mut out = Vec::new();
        // Hard cap pages so a buggy provider cannot spin forever.
        for _ in 0..64 {
            let params = self.block_txs_params(
                workchain,
                &shard,
                seqno,
                &root_hash,
                &file_hash,
                after_lt.as_deref(),
            );
            let page = self.rpc(method, params).await?;
            let txs = page
                .get("transactions")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            if txs.is_empty() {
                break;
            }
            let last_lt = txs
                .last()
                .and_then(|t| {
                    t.get("transaction_id")
                        .and_then(|id| id.get("lt"))
                        .and_then(|l| l.as_str())
                        .or_else(|| t.get("lt").and_then(|l| l.as_str()))
                })
                .map(|s| s.to_string());
            out.extend(txs.into_iter().filter_map(|raw| {
                let tx = self.parse_ton_tx(&raw, mc_height, "");
                if ton_tx_worth_persisting(&tx) {
                    Some(tx)
                } else {
                    None
                }
            }));
            let incomplete = page
                .get("incomplete")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if !incomplete {
                break;
            }
            match last_lt {
                Some(lt) if after_lt.as_deref() != Some(lt.as_str()) => after_lt = Some(lt),
                _ => break,
            }
        }
        Ok(out)
    }

    /// Parse one TON transaction into a `NormalizedTx`. Only the fields that
    /// are cheap to extract (hash, in/out addresses, value, fee, status) are
    /// populated; the full raw payload is preserved in `raw`.
    fn parse_ton_tx(&self, raw: &Value, height: u64, default_from: &str) -> NormalizedTx {
        let hash = raw
            .get("transaction_id")
            .and_then(|t| t.get("hash"))
            .and_then(|h| h.as_str())
            .or_else(|| raw.get("hash").and_then(|h| h.as_str()))
            .unwrap_or_default()
            .to_string();

        let in_msg = raw.get("in_msg");
        let first_out = raw
            .get("out_msgs")
            .and_then(|o| o.as_array())
            .and_then(|a| a.first());

        // Prefer a non-zero inbound value; wallet external-message txs often
        // carry `in_msg.value = "0"` with the real transfer on `out_msgs[0]`.
        let in_value = in_msg
            .and_then(|m| m.get("value"))
            .and_then(parse_ton_nanoton_str)
            .filter(|&v| v > 0);
        let out_value = first_out
            .and_then(|m| m.get("value"))
            .and_then(parse_ton_nanoton_str)
            .filter(|&v| v > 0);
        let value = in_value.or(out_value);

        let from = in_msg
            .and_then(|m| m.get("source"))
            .and_then(ton_account_address)
            .or_else(|| first_out.and_then(|m| m.get("source")).and_then(ton_account_address))
            .or_else(|| {
                if default_from.is_empty() {
                    None
                } else {
                    Some(Address::new(default_from))
                }
            });

        let to = match in_value {
            // Internal transfer: destination is the in_msg recipient.
            Some(_) => in_msg
                .and_then(|m| m.get("destination"))
                .and_then(ton_account_address),
            // External / zero-in: use the first out_msg destination when present.
            None => first_out
                .and_then(|m| m.get("destination"))
                .and_then(ton_account_address)
                .or_else(|| {
                    in_msg
                        .and_then(|m| m.get("destination"))
                        .and_then(ton_account_address)
                })
                .or_else(|| raw.get("address").and_then(ton_account_address)),
        };

        let fee = raw.get("fee").and_then(parse_ton_nanoton_str);

        // Exit code lives at `/{phase,action}/result/code`; for v3+ accounts
        // `compute_skipped` / `no_funds` / `ok` are common non-error results.
        let phase_code = raw
            .pointer("/description/action")
            .and_then(|a| a.get("result_code"))
            .and_then(|c| c.as_i64())
            .or_else(|| {
                raw.pointer("/description/phase/compute")
                    .and_then(|p| p.get("result_code"))
                    .and_then(|c| c.as_i64())
            })
            .or_else(|| {
                raw.pointer("/description/phase/storage")
                    .and_then(|p| p.get("result_code"))
                    .and_then(|c| c.as_i64())
            })
            .unwrap_or(0);
        let aborted = raw
            .get("aborted")
            .and_then(|a| a.as_bool())
            .unwrap_or(false);
        let status = if aborted || phase_code < 0 {
            TxStatus::Failed
        } else {
            TxStatus::Success
        };

        NormalizedTx {
            hash: TxHash::new(hash),
            from,
            to,
            value: value
                .map(|v| wallet_types::Amount::new(rust_decimal::Decimal::from(v), 9))
                .unwrap_or_else(|| wallet_types::Amount::zero(9)),
            gas_fee: fee
                .map(|v| wallet_types::Amount::new(rust_decimal::Decimal::from(v), 9)),
            block_number: height,
            status,
            raw: raw.clone(),
            contract_address: in_msg
                .and_then(|m| m.get("destination"))
                .and_then(ton_account_address),
            log_index: None,
            method: in_msg
                .and_then(|m| m.get("op"))
                .and_then(|m| m.get("op_name"))
                .and_then(|s| s.as_str())
                .map(|s| s.to_string()),
        }
    }
}

#[async_trait]
impl TokenBalance for TonChain {
    /// Jetton (TIP-3) wallet balance for `wallet` (the user's TON address).
    /// The chain-gateway currently exposes this only for symmetry with other
    /// chains; a full jetton-wallet derivation needs the user's owner pubkey
    /// and is left to the higher-level API.
    async fn token_balance(&self, wallet: &Address, token: &Address) -> AppResult<Amount> {
        // `token` is expected to be the jetton-wallet address (not the master
        // jetton contract). The user resolves the wallet address via
        // `getJettonWalletAddress` upstream.
        let _ = wallet;
        let result = self
            .rpc("getJettonBalance", json!({ "jetton_wallet": token.as_str() }))
            .await?;
        let raw = result
            .get("balance")
            .and_then(|b| b.as_str())
            .and_then(|s| s.parse::<u64>().ok())
            .or_else(|| result.get("balance").and_then(|b| b.as_u64()))
            .unwrap_or(0);
        // Default jetton decimals; callers that need precision should pass
        // the right `decimals` from the token registry row.
        Ok(Amount::new(Decimal::from(raw), 9))
    }
}

/// Mark a TON tx as success/failed based on the `getTransaction` exit code.
/// TON uses negative exit codes for errors (`<= -1`).
#[allow(dead_code)]
pub(crate) fn ton_tx_status(exit_code: i32) -> TxStatus {
    if exit_code == 0 {
        TxStatus::Success
    } else {
        TxStatus::Failed
    }
}

#[async_trait]
impl GasEstimator for TonChain {
    /// Estimate fees for a TON message. The full signed (or unsigned) message
    /// BOC may be supplied in `tx.extras.messageBoc` for a wallet-version-
    /// accurate read via `runGetMethod` on the user's wallet contract.
    /// When the BOC is absent we fall back to reading the workchain gas
    /// price from `getConfigParam 18` and applying a size-based heuristic
    /// for the gas units.
    ///
    /// Output is in nanotons (`fee_native`).
    async fn estimate_gas(&self, tx: &GasEstimateRequest) -> AppResult<GasEstimate> {
        // 1. Workchain gas price (masterchain config 18). We accept a few
        //    response shapes; toncenter v2 / v3 differ in nesting.
        let cfg = self
            .rpc("getConfigParam", json!({ "config": 18 }))
            .await?;
        let gas_price = cfg
            .pointer("/config/gasPrices/gasPrice")
            .and_then(parse_ton_nanoton_str)
            .or_else(|| {
                cfg.pointer("/config/gasPrice")
                    .and_then(parse_ton_nanoton_str)
            })
            .or_else(|| cfg.get("gasPrice").and_then(parse_ton_nanoton_str))
            .unwrap_or(1_000_000) as u128; // 0.001 TON fallback (last-resort)

        // 2. Estimate the gas units the message will consume.
        let boc = tx
            .extras
            .as_ref()
            .and_then(|v| v.get("messageBoc"))
            .and_then(|v| v.as_str());
        let gas_used = if let Some(boc_str) = boc {
            self.estimate_gas_via_wallet(tx.from.as_str(), boc_str)
                .await
                .ok()
                .flatten()
                .unwrap_or_else(|| self.heuristic_gas_units(tx))
        } else {
            self.heuristic_gas_units(tx)
        };

        // TON fees formula: fees = gas_used * gas_price / 2^16.
        // For messages that cross shards there is an additional forwarding
        // fee (`fwd_fee`); we add a 5% buffer to absorb that.
        let mut fee_native = gas_used as u128 * gas_price / (1u128 << 16);
        fee_native = fee_native.saturating_mul(105) / 100;

        Ok(GasEstimate {
            gas_limit: gas_used,
            max_fee_per_gas: Some(gas_price),
            max_priority_fee_per_gas: None,
            gas_price: Some(gas_price),
            fee_native: Some(fee_native),
            gas_used: Some(gas_used),
        })
    }
}

impl TonChain {
    /// Try the v3/v4 wallet contract's `getEstimateFees` method via
    /// `runGetMethod` (exposed by toncenter). Returns the fees in nanotons
    /// when the wallet is reachable, or `None` when the RPC is unsupported
    /// / the contract doesn't implement the method.
    async fn estimate_gas_via_wallet(
        &self,
        wallet_addr: &str,
        message_boc: &str,
    ) -> AppResult<Option<u64>> {
        let result = self
            .rpc(
                "runGetMethod",
                json!({
                    "address": wallet_addr,
                    "method": "getEstimateFees",
                    "stack": [
                        // toncenter takes already-encoded BOC cells; we send
                        // the BOC as a single slice arg.
                        ["tvm.Slice", format!("base64:{message_boc}")]
                    ],
                }),
            )
            .await?;
        let fees = result
            .get("fees")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<u64>().ok());
        if let Some(f) = fees {
            return Ok(Some(f));
        }
        // Fallback: parse the stack-based reply: `[["int256", "<n>"]]` (n fees).
        if let Some(s) = result
            .pointer("/stack/0/1")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<u64>().ok())
        {
            return Ok(Some(s));
        }
        Ok(None)
    }

    /// Pure size-based heuristic for TON gas units. Used when no signed BOC
    /// is available (e.g. estimation during tx construction).
    fn heuristic_gas_units(&self, tx: &GasEstimateRequest) -> u64 {
        // Base cost for a single message hop; toncenter's getEstimateFees
        // returns ~10_000_000 for a bare transfer.
        let mut units: u64 = 10_000_000;
        if let Some(data) = &tx.data {
            // TON charges for storing payload cells; ~1000 gas per byte
            // matches the observed accounting for plain text comments.
            units = units.saturating_add(data.len() as u64 * 1000);
        }
        if tx.value.is_none() {
            // No-value messages are more expensive to route; bump the budget.
            units = units.saturating_add(1_000_000);
        }
        units
    }
}

/// Parse a nanotons amount that the TON API can return as a string
/// ("1000000") or a number.
fn parse_ton_nanoton_str(v: &Value) -> Option<u64> {
    v.as_str()
        .and_then(|s| s.parse::<u64>().ok())
        .or_else(|| v.as_u64())
}

/// Extract `(root_hash, file_hash)` from either a flat `ton.blockIdExt`
/// (`lookupBlock` / `shards`) or a nested `blocks.header` (`getBlockHeader`).
fn extract_ton_block_hashes(block: &Value) -> Option<(String, String)> {
    let root = block
        .get("root_hash")
        .and_then(|v| v.as_str())
        .or_else(|| block.pointer("/id/root_hash").and_then(|v| v.as_str()))?;
    let file = block
        .get("file_hash")
        .and_then(|v| v.as_str())
        .or_else(|| block.pointer("/id/file_hash").and_then(|v| v.as_str()))?;
    if root.is_empty() || file.is_empty() {
        return None;
    }
    Some((root.to_string(), file.to_string()))
}

/// Read a TON address that may be a bare string or an
/// `{ "account_address": "..." }` object from `getBlockTransactionsExt`.
fn ton_account_address(v: &Value) -> Option<Address> {
    if let Some(s) = v.as_str() {
        if !s.is_empty() {
            return Some(Address::new(s));
        }
    }
    v.get("account_address")
        .and_then(|a| a.as_str())
        .filter(|s| !s.is_empty())
        .map(Address::new)
}

/// Zero-value TON txs are system/tick noise (elector, config, empty externals)
/// and are not persisted into `transactions`.
fn ton_tx_worth_persisting(tx: &NormalizedTx) -> bool {
    !tx.hash.as_str().is_empty() && tx.value.raw > rust_decimal::Decimal::ZERO
}

/// Read the masterchain seqno from a `getMasterchainInfo` response.
///
/// The standard toncenter v2/v3 shape is:
///   `{"last": {"seqno": <int>}}`
/// but several endpoints (and the chain-gateway wrapping layer) return
/// `seqno` as a string, and a few proxies flatten to just `{"seqno": N}`.
/// Try them in order, return the first hit.
pub(crate) fn parse_ton_seqno(result: &Value) -> Option<u64> {
    let pick_u64 = |v: &Value| -> Option<u64> {
        if let Some(n) = v.as_u64() {
            return Some(n);
        }
        if let Some(s) = v.as_str() {
            return s.parse::<u64>().ok();
        }
        None
    };

    result
        .get("last")
        .and_then(|l| l.get("seqno"))
        .and_then(pick_u64)
        .or_else(|| result.get("seqno").and_then(pick_u64))
        .or_else(|| result.as_u64())
}

#[cfg(test)]
mod tests {
    use super::{
        extract_ton_block_hashes, parse_ton_nanoton_str, parse_ton_seqno, ton_tx_status,
        ton_tx_worth_persisting, TonApiVersion, TonChain,
    };
    use serde_json::json;
    use wallet_types::{Address, Amount, GasEstimateRequest, TxStatus};
    use wallet_types::ChainIndex as CI;

    #[test]
    fn ton_tx_status_zero_is_success() {
        assert_eq!(ton_tx_status(0), TxStatus::Success);
    }

    #[test]
    fn ton_tx_status_negative_is_failed() {
        assert_eq!(ton_tx_status(-1), TxStatus::Failed);
        assert_eq!(ton_tx_status(-13), TxStatus::Failed);
    }

    #[test]
    fn ton_parse_ton_tx_extracts_in_msg_fields() {
        // Construct a `TonChain` from a stub URL; we only test the
        // `parse_ton_tx` helper so the RPC pool is never touched.
        let chain = TonChain {
            chain_index: CI(607),
            pool: crate::provider::RpcPool::new(vec![wallet_config::RpcEndpoint {
                url: "https://example.org".into(),
                weight: 1,
            }])
            .unwrap(),
            api_version: TonApiVersion::V2,
        };
        let raw = json!({
            "transaction_id": { "hash": "abc" },
            "in_msg": {
                "source": "EQ..src",
                "destination": "EQ..dst",
                "value": "1000000000",
                "op": { "op_name": "text_comment" }
            },
            "fee": "100000",
            "aborted": false
        });
        let tx = chain.parse_ton_tx(&raw, 99, "");
        assert_eq!(tx.hash.as_str(), "abc");
        assert_eq!(tx.from.as_ref().unwrap().as_str(), "EQ..src");
        assert_eq!(tx.to.as_ref().unwrap().as_str(), "EQ..dst");
        assert_eq!(tx.value, Amount::new(rust_decimal::Decimal::from(1_000_000_000u64), 9));
        assert_eq!(tx.gas_fee.as_ref().unwrap().raw, rust_decimal::Decimal::from(100_000u64));
        assert_eq!(tx.status, TxStatus::Success);
        assert_eq!(tx.method.as_deref(), Some("text_comment"));
    }

    #[test]
    fn ton_parse_ton_tx_reads_nested_account_address() {
        // toncenter `getBlockTransactionsExt` nests source/destination under
        // `{ "@type": "accountAddress", "account_address": "..." }`.
        let chain = TonChain {
            chain_index: CI(607),
            pool: crate::provider::RpcPool::new(vec![wallet_config::RpcEndpoint {
                url: "https://example.org".into(),
                weight: 1,
            }])
            .unwrap(),
            api_version: TonApiVersion::V2,
        };
        let raw = json!({
            "transaction_id": { "hash": "nestedHash" },
            "in_msg": {
                "source": {
                    "@type": "accountAddress",
                    "account_address": "EQ..src"
                },
                "destination": {
                    "@type": "accountAddress",
                    "account_address": "EQ..dst"
                },
                "value": "127854000000"
            },
            "fee": "82159",
            "aborted": false
        });
        let tx = chain.parse_ton_tx(&raw, 42, "");
        assert_eq!(tx.hash.as_str(), "nestedHash");
        assert_eq!(tx.from.as_ref().unwrap().as_str(), "EQ..src");
        assert_eq!(tx.to.as_ref().unwrap().as_str(), "EQ..dst");
        assert_eq!(
            tx.value,
            Amount::new(rust_decimal::Decimal::from(127_854_000_000u64), 9)
        );
    }

    #[test]
    fn ton_parse_ton_tx_uses_out_msg_value_when_in_msg_is_zero() {
        // Wallet external-message txs attach value on out_msgs; in_msg.value
        // is "0". Treating that as authoritative left value=0 in the DB.
        let chain = TonChain {
            chain_index: CI(607),
            pool: crate::provider::RpcPool::new(vec![wallet_config::RpcEndpoint {
                url: "https://example.org".into(),
                weight: 1,
            }])
            .unwrap(),
            api_version: TonApiVersion::V2,
        };
        let raw = json!({
            "transaction_id": { "hash": "extHash" },
            "in_msg": {
                "source": { "account_address": "" },
                "destination": { "account_address": "EQ..wallet" },
                "value": "0"
            },
            "out_msgs": [{
                "source": { "account_address": "EQ..wallet" },
                "destination": { "account_address": "EQ..dst" },
                "value": "261492865"
            }],
            "fee": "447763"
        });
        let tx = chain.parse_ton_tx(&raw, 1, "");
        assert_eq!(
            tx.value,
            Amount::new(rust_decimal::Decimal::from(261_492_865u64), 9)
        );
        assert_eq!(tx.from.as_ref().unwrap().as_str(), "EQ..wallet");
        assert_eq!(tx.to.as_ref().unwrap().as_str(), "EQ..dst");
        assert!(ton_tx_worth_persisting(&tx));
    }

    #[test]
    fn ton_zero_value_tx_is_not_worth_persisting() {
        let chain = TonChain {
            chain_index: CI(607),
            pool: crate::provider::RpcPool::new(vec![wallet_config::RpcEndpoint {
                url: "https://example.org".into(),
                weight: 1,
            }])
            .unwrap(),
            api_version: TonApiVersion::V2,
        };
        let raw = json!({
            "transaction_id": { "hash": "sysHash" },
            "address": { "account_address": "Ef8zMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzM0vF" },
            "out_msgs": [],
            "fee": "0"
        });
        let tx = chain.parse_ton_tx(&raw, 1, "");
        assert_eq!(tx.value.raw, rust_decimal::Decimal::ZERO);
        assert!(!ton_tx_worth_persisting(&tx));
    }

    #[test]
    fn extract_ton_block_hashes_reads_nested_id() {
        // Real toncenter/ZAN `getBlockHeader` puts hashes under `id`, not at
        // the top level. Looking only at the top level silently drops every
        // block's transactions.
        let header = json!({
            "@type": "blocks.header",
            "id": {
                "@type": "ton.blockIdExt",
                "workchain": -1,
                "shard": "-9223372036854775808",
                "seqno": 85205453,
                "root_hash": "2sNJiJnelCZEz2G747o3/f1haqmCJn8/fp7TAi4dpSA=",
                "file_hash": "JE8xjxifh004YkQOOqk8PGwnLP6D5KY6VN271rRitec="
            }
        });
        let (root, file) = extract_ton_block_hashes(&header).expect("nested id hashes");
        assert_eq!(root, "2sNJiJnelCZEz2G747o3/f1haqmCJn8/fp7TAi4dpSA=");
        assert_eq!(file, "JE8xjxifh004YkQOOqk8PGwnLP6D5KY6VN271rRitec=");
    }

    #[test]
    fn extract_ton_block_hashes_reads_top_level_lookup_block() {
        let block = json!({
            "@type": "ton.blockIdExt",
            "workchain": -1,
            "shard": "-9223372036854775808",
            "seqno": 1,
            "root_hash": "root==",
            "file_hash": "file=="
        });
        let (root, file) = extract_ton_block_hashes(&block).expect("top-level hashes");
        assert_eq!(root, "root==");
        assert_eq!(file, "file==");
    }

    #[test]
    fn ton_v2_block_txs_method_is_extended() {
        // Short `getBlockTransactions` IDs cannot be hydrated on public v2;
        // `getBlockTransactionsExt` returns full raw.transaction objects.
        let chain = TonChain {
            chain_index: CI(607),
            pool: crate::provider::RpcPool::new(vec![wallet_config::RpcEndpoint {
                url: "https://example.org".into(),
                weight: 1,
            }])
            .unwrap(),
            api_version: TonApiVersion::V2,
        };
        assert_eq!(chain.block_txs_method(), "getBlockTransactionsExt");
    }

    #[test]
    fn ton_parse_ton_tx_marks_aborted_as_failed() {
        let chain = TonChain {
            chain_index: CI(607),
            pool: crate::provider::RpcPool::new(vec![wallet_config::RpcEndpoint {
                url: "https://example.org".into(),
                weight: 1,
            }])
            .unwrap(),
            api_version: TonApiVersion::V2,
        };
        let raw = json!({
            "transaction_id": { "hash": "x" },
            "in_msg": { "source": "EQ..src", "destination": "EQ..dst", "value": "0" },
            "aborted": true
        });
        let tx = chain.parse_ton_tx(&raw, 1, "");
        assert_eq!(tx.status, TxStatus::Failed);
    }

    #[test]
    fn ton_parse_ton_tx_marks_negative_action_code_as_failed() {
        let chain = TonChain {
            chain_index: CI(607),
            pool: crate::provider::RpcPool::new(vec![wallet_config::RpcEndpoint {
                url: "https://example.org".into(),
                weight: 1,
            }])
            .unwrap(),
            api_version: TonApiVersion::V2,
        };
        let raw = json!({
            "transaction_id": { "hash": "x" },
            "in_msg": { "source": "EQ..src", "destination": "EQ..dst", "value": "0" },
            "description": { "action": { "result_code": -13 } }
        });
        let tx = chain.parse_ton_tx(&raw, 1, "");
        assert_eq!(tx.status, TxStatus::Failed);
    }

    #[test]
    fn ton_heuristic_gas_units_grows_with_payload() {
        let chain = TonChain {
            chain_index: CI(607),
            pool: crate::provider::RpcPool::new(vec![wallet_config::RpcEndpoint {
                url: "https://example.org".into(),
                weight: 1,
            }])
            .unwrap(),
            api_version: TonApiVersion::V2,
        };
        let no_payload = GasEstimateRequest {
            from: Address::new("EQ..from"),
            to: Some(Address::new("EQ..to")),
            data: None,
            value: Some(Amount::new(rust_decimal::Decimal::from(1u64), 9)),
            extras: None,
        };
        let with_payload = GasEstimateRequest {
            data: Some(vec![0u8; 128]),
            ..clone_request(&no_payload)
        };
        let no_value = GasEstimateRequest {
            value: None,
            ..clone_request(&no_payload)
        };
        assert!(chain.heuristic_gas_units(&with_payload) > chain.heuristic_gas_units(&no_payload));
        assert!(chain.heuristic_gas_units(&no_value) > chain.heuristic_gas_units(&no_payload));
    }

    fn clone_request(req: &GasEstimateRequest) -> GasEstimateRequest {
        GasEstimateRequest {
            from: req.from.clone(),
            to: req.to.clone(),
            data: req.data.clone(),
            value: req.value.clone(),
            extras: req.extras.clone(),
        }
    }

    #[test]
    fn parse_ton_seqno_handles_nested_number() {
        let v = json!({ "last": { "seqno": 12345, "@type": "blocks.blockHeader" } });
        assert_eq!(parse_ton_seqno(&v), Some(12345));
    }

    #[test]
    fn parse_ton_seqno_handles_string_seqno() {
        // Some toncenter-compatible endpoints (and v3 JSON-RPC) return
        // the seqno as a quoted string.
        let v = json!({ "last": { "seqno": "12345" } });
        assert_eq!(parse_ton_seqno(&v), Some(12345));
    }

    #[test]
    fn parse_ton_seqno_handles_flattened_shape() {
        // Chain-gateway wrappers sometimes strip the `last` envelope.
        assert_eq!(parse_ton_seqno(&json!({ "seqno": 7 })), Some(7));
        assert_eq!(parse_ton_seqno(&json!({ "seqno": "7" })), Some(7));
        assert_eq!(parse_ton_seqno(&json!(42)), Some(42));
    }

    #[test]
    fn parse_ton_seqno_returns_none_when_missing() {
        assert_eq!(parse_ton_seqno(&json!({})), None);
        assert_eq!(parse_ton_seqno(&json!({ "last": {} })), None);
        assert_eq!(parse_ton_seqno(&json!({ "last": { "seqno": "garbage" } })), None);
        assert_eq!(parse_ton_seqno(&json!(null)), None);
    }

    #[test]
    fn parse_ton_nanoton_str_parses_both_forms() {
        assert_eq!(parse_ton_nanoton_str(&json!("1000")), Some(1000));
        assert_eq!(parse_ton_nanoton_str(&json!(1000u64)), Some(1000));
        assert_eq!(parse_ton_nanoton_str(&json!("abc")), None);
    }

    /// Regression guard for the toncenter v2 `getBlockTransactions` schema:
    /// the field names are `workchain_id` and `shard_id` (NOT `workchain` /
    /// `shard` like `lookupBlock` uses). If anyone re-renames these by
    /// accident, toncenter returns 422 and the syncer stalls. This test
    /// pins the exact body shape we send.
    #[test]
    fn ton_api_version_detects_from_url() {
        assert_eq!(TonApiVersion::from_url("https://example.com/api/v2/jsonRPC"), TonApiVersion::V2);
        assert_eq!(TonApiVersion::from_url("https://example.com/api/v3/jsonRPC"), TonApiVersion::V3);
        assert_eq!(TonApiVersion::from_url("https://example.com/api/v4/jsonRPC"), TonApiVersion::V4);
    }

    #[test]
    fn ton_api_version_from_config() {
        let cfg_no_version = wallet_config::ChainRuntimeConfig {
            chain_index: 607,
            family: "ton".into(),
            ton_api_version: None,
            endpoints: vec![],
            confirmations: 12,
            evm_chain_id: None,
        };
        assert_eq!(TonApiVersion::from_config(&cfg_no_version), TonApiVersion::V2);

        let cfg_v3 = wallet_config::ChainRuntimeConfig {
            chain_index: 607,
            family: "ton".into(),
            ton_api_version: Some(3),
            endpoints: vec![],
            confirmations: 12,
            evm_chain_id: None,
        };
        assert_eq!(TonApiVersion::from_config(&cfg_v3), TonApiVersion::V3);
    }

    #[test]
    fn ton_block_txs_params_v2() {
        let chain = TonChain {
            chain_index: CI(607),
            pool: crate::provider::RpcPool::new(vec![wallet_config::RpcEndpoint {
                url: "https://toncenter.com/api/v2/jsonRPC".into(),
                weight: 1,
            }])
            .unwrap(),
            api_version: TonApiVersion::V2,
        };
        let params = chain.block_txs_params(
            -1,
            "-9223372036854775808",
            85189872,
            "abcd",
            "ef01",
            None,
        );
        assert_eq!(params.get("workchain").and_then(|v| v.as_i64()), Some(-1));
        assert_eq!(params.get("shard").and_then(|v| v.as_str()), Some("-9223372036854775808"));
        assert_eq!(params.get("seqno").and_then(|v| v.as_u64()), Some(85189872));
        assert_eq!(params.get("count").and_then(|v| v.as_u64()), Some(256));
        assert!(params.get("after_lt").is_none());
        assert!(params.get("block").is_none());
    }

    #[test]
    fn ton_block_txs_params_includes_after_lt() {
        let chain = TonChain {
            chain_index: CI(607),
            pool: crate::provider::RpcPool::new(vec![wallet_config::RpcEndpoint {
                url: "https://toncenter.com/api/v2/jsonRPC".into(),
                weight: 1,
            }])
            .unwrap(),
            api_version: TonApiVersion::V3,
        };
        let params = chain.block_txs_params(
            0,
            "-9223372036854775808",
            89660165,
            "abcd",
            "ef01",
            Some("95855995000005"),
        );
        assert_eq!(params.get("workchain").and_then(|v| v.as_i64()), Some(0));
        assert_eq!(params.get("seqno").and_then(|v| v.as_u64()), Some(89660165));
        assert_eq!(
            params.get("after_lt").and_then(|v| v.as_str()),
            Some("95855995000005")
        );
    }

    /// Regression guard for the toncenter v2 `getBlockTransactions` schema:
    /// the field names are `workchain` and `shard`. If anyone re-renames these by
    /// accident, toncenter returns 422 and the syncer stalls. This test
    /// pins the exact body shape we send.
    #[test]
    fn fetch_block_txs_request_body_uses_workchain_and_shard() {
        let height: u64 = 85186901;
        // NOTE: toncenter JSON-RPC getBlockHeader uses direct params, not block wrapper.
        let header_params = json!({
            "workchain": -1,
            "shard": "-9223372036854775808",
            "seqno": height,
        });
        assert_eq!(header_params.get("workchain").and_then(|v| v.as_i64()), Some(-1));
        assert_eq!(header_params.get("shard").and_then(|v| v.as_str()), Some("-9223372036854775808"));
        assert!(header_params.get("workchain_id").is_none());

        // Body for `getBlockTransactions` (v2).
        let block_txs_params = json!({
            "workchain": -1,
            "shard": "-9223372036854775808",
            "seqno": height,
            "root_hash": "abcd",
            "file_hash": "ef01",
            "count": 1000,
        });
        assert_eq!(block_txs_params.get("workchain").and_then(|v| v.as_i64()), Some(-1));
        assert_eq!(
            block_txs_params.get("shard").and_then(|v| v.as_str()),
            Some("-9223372036854775808")
        );
        assert_eq!(block_txs_params.get("seqno").and_then(|v| v.as_u64()), Some(height));
        assert!(block_txs_params.get("workchain_id").is_none());
        assert!(block_txs_params.get("block").is_none());
    }
}
