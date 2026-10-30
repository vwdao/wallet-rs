use crate::provider::RpcPool;
use crate::traits::{
    BalanceReader, BlockSource, GasEstimator, NonceProvider, TokenBalance, TxBroadcaster,
};
use async_trait::async_trait;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use wallet_config::ChainRuntimeConfig;
use wallet_error::{AppError, AppResult};
use wallet_types::{
    Address, Amount, ChainIndex, GasEstimate, GasEstimateRequest, NormalizedTx, TxHash, TxStatus,
};

mod internal;
pub mod multicall;
pub use multicall::*;

#[derive(Debug)]
pub struct EvmChain {
    pub chain_index: ChainIndex,
    pub chain_id: u64,
    pub pool: RpcPool,
    pub confirmations: u64,
    pub decimals_cache: Arc<parking_lot::RwLock<HashMap<String, u32>>>,
}

impl EvmChain {
    pub fn new(cfg: &ChainRuntimeConfig) -> AppResult<Arc<Self>> {
        let chain_id = cfg.evm_chain_id.ok_or_else(|| {
            AppError::InvalidArgument(format!(
                "evm_chain_id required for chain {}",
                cfg.chain_index
            ))
        })?;
        Ok(Arc::new(Self {
            chain_index: ChainIndex(cfg.chain_index),
            chain_id,
            pool: RpcPool::new(cfg.endpoints.clone())?,
            confirmations: cfg.confirmations,
            decimals_cache: Arc::new(parking_lot::RwLock::new(HashMap::new())),
        }))
    }

    async fn rpc(&self, method: &str, params: Value) -> AppResult<Value> {
        let attempts = self.pool.len().max(1);
        let mut last_err = None;

        for _ in 0..attempts {
            let url = match self.pool.next_url() {
                Ok(u) => u.to_string(),
                Err(e) => return Err(e),
            };
            let body = json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": method,
                "params": params.clone(),
            });
            let resp = match self
                .pool
                .client()
                .post(&url)
                .json(&body)
                .send()
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    self.pool.mark_failure(&url);
                    last_err = Some(AppError::Unavailable(format!("{url}: {e}")));
                    continue;
                }
            };

            let status = resp.status();
            if !status.is_success() {
                let text = resp.text().await.unwrap_or_default();
                self.pool.mark_failure(&url);
                last_err = Some(AppError::Unavailable(format!(
                    "rpc {url} returned HTTP {status}: {text}"
                )));
                continue;
            }

            let text = match resp.text().await {
                Ok(t) => t,
                Err(e) => {
                    self.pool.mark_failure(&url);
                    last_err = Some(AppError::Unavailable(format!("read {url} body failed: {e}")));
                    continue;
                }
            };

            let v: Value = match serde_json::from_str(&text) {
                Ok(v) => v,
                Err(e) => {
                    self.pool.mark_failure(&url);
                    last_err = Some(AppError::Unavailable(format!(
                        "decode json from {url} failed: {e}; raw: {text}"
                    )));
                    continue;
                }
            };

            if let Some(err) = v.get("error").filter(|e| !e.is_null()) {
                self.pool.mark_failure(&url);
                last_err = Some(AppError::Unavailable(format!("{url} rpc error: {err}")));
                continue;
            }

            self.pool.mark_success(&url);
            return Ok(v.get("result").cloned().unwrap_or(Value::Null));
        }

        Err(last_err.unwrap_or_else(|| AppError::Unavailable("all rpc attempts failed".into())))
    }

    /// Best-effort RPC call that does not trip the circuit breaker. Used for
    /// optional calls (e.g. `debug_traceTransaction`) where an unsupported
    /// method is expected and must not poison endpoint health.
    async fn rpc_best_effort(&self, method: &str, params: Value) -> Option<Value> {
        let url = self.pool.next_url().ok()?.to_string();
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params,
        });
        let resp = self.pool.client().post(&url).json(&body).send().await.ok()?;
        let v: Value = resp.json().await.ok()?;
        match v.get("error").filter(|e| !e.is_null()) {
            Some(err) => {
                tracing::debug!(method, error = %err, "best-effort rpc failed");
                None
            }
            None => Some(v.get("result").cloned().unwrap_or(Value::Null)),
        }
    }

    /// Send batched JSON-RPC 2.0 requests in a single HTTP POST to minimize round trips.
    pub async fn batch_rpc(&self, calls: Vec<(&str, Value)>) -> AppResult<Vec<Value>> {
        if calls.is_empty() {
            return Ok(Vec::new());
        }
        let attempts = self.pool.len().max(1);
        let mut last_err = None;

        for _ in 0..attempts {
            let url = match self.pool.next_url() {
                Ok(u) => u.to_string(),
                Err(e) => return Err(e),
            };
            let body: Vec<Value> = calls
                .iter()
                .enumerate()
                .map(|(id, (method, params))| {
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "method": *method,
                        "params": params.clone(),
                    })
                })
                .collect();

            let resp = match self
                .pool
                .client()
                .post(&url)
                .json(&body)
                .send()
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    self.pool.mark_failure(&url);
                    last_err = Some(AppError::Unavailable(format!("{url}: {e}")));
                    continue;
                }
            };

            let status = resp.status();
            if !status.is_success() {
                let text = resp.text().await.unwrap_or_default();
                self.pool.mark_failure(&url);
                last_err = Some(AppError::Unavailable(format!(
                    "batch rpc {url} returned HTTP {status}: {text}"
                )));
                continue;
            }

            let text = match resp.text().await {
                Ok(t) => t,
                Err(e) => {
                    self.pool.mark_failure(&url);
                    last_err = Some(AppError::Unavailable(format!("read {url} body failed: {e}")));
                    continue;
                }
            };

            let array: Vec<Value> = match serde_json::from_str(&text) {
                Ok(a) => a,
                Err(e) => {
                    self.pool.mark_failure(&url);
                    last_err = Some(AppError::Unavailable(format!(
                        "decode batch json from {url} failed: {e}; raw: {text}"
                    )));
                    continue;
                }
            };

            self.pool.mark_success(&url);

            let mut results: Vec<Value> = vec![Value::Null; calls.len()];
            for item in array {
                if let Some(id) = item.get("id").and_then(|i| i.as_u64()) {
                    if (id as usize) < results.len() {
                        results[id as usize] = item.get("result").cloned().unwrap_or(Value::Null);
                    }
                }
            }
            return Ok(results);
        }

        Err(last_err.unwrap_or_else(|| AppError::Unavailable("all batch rpc attempts failed".into())))
    }

    /// Execute a Multicall3 batch query via on-chain contract call.
    pub async fn multicall3(&self, calls: &[Multicall3Call]) -> AppResult<Vec<Multicall3Result>> {
        if calls.is_empty() {
            return Ok(Vec::new());
        }
        let calldata = encode_aggregate3(calls);
        let calldata_hex = format!("0x{}", hex::encode(calldata));
        let result = self
            .rpc(
                "eth_call",
                json!([
                    {
                        "to": MULTICALL3_ADDRESS,
                        "data": calldata_hex,
                    },
                    "latest"
                ]),
            )
            .await?;

        let hex_str = result.as_str().unwrap_or("0x").trim_start_matches("0x");
        let bytes = hex::decode(hex_str).map_err(|e| AppError::internal(e.to_string()))?;
        decode_aggregate3(&bytes).ok_or_else(|| AppError::internal("failed to decode multicall3 return data"))
    }
}

#[async_trait]
impl BalanceReader for EvmChain {
    async fn native_balance(&self, addr: &Address) -> AppResult<Amount> {
        let result = self
            .rpc("eth_getBalance", json!([addr.as_str(), "latest"]))
            .await?;
        let hex = result.as_str().unwrap_or("0x0");
        let raw = u128::from_str_radix(hex.trim_start_matches("0x"), 16)
            .map_err(|e| AppError::internal(e.to_string()))?;
        Ok(Amount::new(wallet_types::decimal_from_u128(raw), 18))
    }
}

#[async_trait]
impl TokenBalance for EvmChain {
    async fn token_balance(&self, wallet: &Address, token: &Address) -> AppResult<Amount> {
        let cached_decimals = self.decimals_cache.read().get(token.as_str()).copied();
        if let Some(dec) = cached_decimals {
            // Decimals is already known from cache, only 1 balanceOf eth_call needed!
            let mut data = String::from("0x70a08231");
            let addr = wallet.as_str().trim_start_matches("0x");
            data.push_str(&format!("{:0>64}", addr));
            let result = self
                .rpc(
                    "eth_call",
                    json!([{ "to": token.as_str(), "data": data }, "latest"]),
                )
                .await?;
            let hex = result.as_str().unwrap_or("0x0");
            let raw = u128::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap_or(0);
            return Ok(Amount::new(wallet_types::decimal_from_u128(raw), dec));
        }

        // Decimals not cached: use Multicall3 to fetch decimals() AND balanceOf() in ONE SINGLE eth_call!
        let calls = vec![
            build_decimals_call(token),
            build_balance_of_call(token, wallet),
        ];

        if let Ok(results) = self.multicall3(&calls).await {
            if results.len() == 2 && results[0].success && results[1].success {
                let dec = parse_decimals_from_bytes(&results[0].return_data);
                self.decimals_cache.write().insert(token.as_str().to_string(), dec);
                let bal_dec = parse_u256_to_decimal(&results[1].return_data);
                return Ok(Amount::new(bal_dec, dec));
            }
        }

        // Fallback if Multicall3 is not deployed / reverts on target network:
        let decimals_result = self
            .rpc(
                "eth_call",
                json!([{ "to": token.as_str(), "data": "0x313ce567" }, "latest"]),
            )
            .await;
        let decimals = decimals_result
            .ok()
            .and_then(|r| r.as_str().map(|s| s.to_string()))
            .and_then(|hex| u32::from_str_radix(hex.trim_start_matches("0x"), 16).ok())
            .unwrap_or(18);
        self.decimals_cache.write().insert(token.as_str().to_string(), decimals);

        let mut data = String::from("0x70a08231");
        let addr = wallet.as_str().trim_start_matches("0x");
        data.push_str(&format!("{:0>64}", addr));
        let result = self
            .rpc(
                "eth_call",
                json!([{ "to": token.as_str(), "data": data }, "latest"]),
            )
            .await?;
        let hex = result.as_str().unwrap_or("0x0");
        let raw = u128::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap_or(0);
        Ok(Amount::new(wallet_types::decimal_from_u128(raw), decimals))
    }

    async fn token_balances_batch(
        &self,
        wallet: &Address,
        tokens: &[Address],
    ) -> AppResult<Vec<(Address, Amount)>> {
        if tokens.is_empty() {
            return Ok(Vec::new());
        }

        // Strategy 1: Multicall3 (1 single on-chain eth_call for all decimals + all balances)
        let mut mc_calls = Vec::with_capacity(tokens.len() * 2);
        for t in tokens {
            if !self.decimals_cache.read().contains_key(t.as_str()) {
                mc_calls.push(build_decimals_call(t));
            }
            mc_calls.push(build_balance_of_call(t, wallet));
        }

        if let Ok(results) = self.multicall3(&mc_calls).await {
            let mut res_idx = 0;
            let mut out = Vec::with_capacity(tokens.len());
            for t in tokens {
                let dec = if let Some(&d) = self.decimals_cache.read().get(t.as_str()) {
                    d
                } else if res_idx < results.len() {
                    let d_res = &results[res_idx];
                    res_idx += 1;
                    let d = if d_res.success {
                        parse_decimals_from_bytes(&d_res.return_data)
                    } else {
                        18
                    };
                    self.decimals_cache.write().insert(t.as_str().to_string(), d);
                    d
                } else {
                    18
                };

                let bal_dec = if res_idx < results.len() {
                    let bal_res = &results[res_idx];
                    res_idx += 1;
                    if bal_res.success {
                        parse_u256_to_decimal(&bal_res.return_data)
                    } else {
                        Decimal::ZERO
                    }
                } else {
                    Decimal::ZERO
                };
                out.push((t.clone(), Amount::new(bal_dec, dec)));
            }
            return Ok(out);
        }

        // Strategy 2 (Fallback): JSON-RPC 2.0 array batching if Multicall3 is unavailable
        let mut missing_decimals = Vec::new();
        for t in tokens {
            if !self.decimals_cache.read().contains_key(t.as_str()) {
                missing_decimals.push(t);
            }
        }
        if !missing_decimals.is_empty() {
            let dec_calls: Vec<(&str, Value)> = missing_decimals
                .iter()
                .map(|t| {
                    (
                        "eth_call",
                        json!([{ "to": t.as_str(), "data": "0x313ce567" }, "latest"]),
                    )
                })
                .collect();
            if let Ok(results) = self.batch_rpc(dec_calls).await {
                let mut cache = self.decimals_cache.write();
                for (t, res) in missing_decimals.into_iter().zip(results) {
                    let d = res
                        .as_str()
                        .and_then(|hex| u32::from_str_radix(hex.trim_start_matches("0x"), 16).ok())
                        .unwrap_or(18);
                    cache.insert(t.as_str().to_string(), d);
                }
            }
        }

        let addr = wallet.as_str().trim_start_matches("0x");
        let calldata = format!("0x70a08231{:0>64}", addr);
        let bal_calls: Vec<(&str, Value)> = tokens
            .iter()
            .map(|t| {
                (
                    "eth_call",
                    json!([{ "to": t.as_str(), "data": &calldata }, "latest"]),
                )
            })
            .collect();

        let results = self.batch_rpc(bal_calls).await?;
        let cache = self.decimals_cache.read();
        let mut out = Vec::with_capacity(tokens.len());
        for (t, res) in tokens.iter().zip(results) {
            let hex = res.as_str().unwrap_or("0x0");
            let raw = u128::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap_or(0);
            let dec = cache.get(t.as_str()).copied().unwrap_or(18);
            out.push((t.clone(), Amount::new(wallet_types::decimal_from_u128(raw), dec)));
        }
        Ok(out)
    }
}

#[async_trait]
impl TxBroadcaster for EvmChain {
    async fn send_raw(&self, raw: &[u8]) -> AppResult<TxHash> {
        let hex_tx = format!("0x{}", hex::encode(raw));
        let result = self.rpc("eth_sendRawTransaction", json!([hex_tx])).await?;
        let hash = result
            .as_str()
            .ok_or_else(|| AppError::internal("missing tx hash"))?;
        Ok(TxHash::new(hash))
    }
}

#[async_trait]
impl BlockSource for EvmChain {
    async fn tip(&self) -> AppResult<u64> {
        let result = self.rpc("eth_blockNumber", json!([])).await?;
        let hex = result
            .as_str()
            .ok_or_else(|| AppError::Unavailable("eth_blockNumber returned non-string result".into()))?;
        let trimmed = hex.trim_start_matches("0x");
        if trimmed.is_empty() || trimmed == "0" {
            return Err(AppError::Unavailable("eth_blockNumber returned 0x0 or empty".into()));
        }
        u64::from_str_radix(trimmed, 16)
            .map_err(|e| AppError::internal(e.to_string()))
    }

    async fn fetch_block_txs(&self, height: u64) -> AppResult<Vec<NormalizedTx>> {
        let block_hex = format!("0x{height:x}");
        // A gateway that is failing over, or that has a lagging upstream in
        // rotation, can transiently return `null` for a block that certainly
        // exists (the tip has long since passed it). Retry with a short
        // backoff before surfacing the error so a single blip doesn't stall
        // the whole catch-up sweep.
        const FETCH_ATTEMPTS: usize = 3;
        let mut last_err = None;
        for attempt in 1..=FETCH_ATTEMPTS {
            match self.fetch_block_once(&block_hex, height).await {
                Ok(txs) => return Ok(txs),
                Err(e) if attempt < FETCH_ATTEMPTS => {
                    tracing::debug!(
                        height,
                        attempt,
                        error = %e,
                        "transient block fetch failure, retrying"
                    );
                    tokio::time::sleep(Duration::from_millis(200 * attempt as u64)).await;
                }
                Err(e) => last_err = Some(e),
            }
        }
        Err(last_err.unwrap_or_else(|| {
            AppError::Unavailable(format!("failed to fetch block at height {height}"))
        }))
    }
}

impl EvmChain {
    /// Single attempt at fetching a block's normalized transactions.
    async fn fetch_block_once(
        &self,
        block_hex: &str,
        height: u64,
    ) -> AppResult<Vec<NormalizedTx>> {
        let result = self
            .rpc("eth_getBlockByNumber", json!([block_hex, true]))
            .await?;
        // A `null` result (or a payload without a `hash`) means the gateway
        // could not serve the block at this height — a transient flake, not a
        // legitimate empty block. Surface it as an error so the caller can
        // retry instead of skipping the height.
        if result.is_null() || result.get("hash").and_then(Value::as_str).is_none() {
            return Err(AppError::Unavailable(format!(
                "eth_getBlockByNumber returned no block at height {height}"
            )));
        }
        let txs = result
            .get("transactions")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default();
        let mut out = Vec::with_capacity(txs.len());
        let mut fee_by_hash: HashMap<String, Amount> = HashMap::new();
        // EIP-7702 txs (type 0x4) can move native tokens via authorized calls;
        // they are traced for internal transfers regardless of Swap events.
        let mut authorized: HashSet<String> = HashSet::new();
        for tx in txs {
            let hash = tx.get("hash").and_then(|v| v.as_str()).unwrap_or_default();
            if internal::is_eip7702(&tx) {
                authorized.insert(hash.to_string());
            }
            let from = tx.get("from").and_then(|v| v.as_str()).map(Address::new);
            let to = tx.get("to").and_then(|v| v.as_str()).map(Address::new);
            let value_hex = tx.get("value").and_then(|v| v.as_str()).unwrap_or("0x0");
            let value = u128::from_str_radix(value_hex.trim_start_matches("0x"), 16).unwrap_or(0);
            let value = wallet_types::decimal_from_u128(value);
            // Skip native rows that move zero value (pure contract interactions /
            // 0-value calls); actual token movement is captured via log parsing.
            if value.is_zero() {
                continue;
            }
            out.push(NormalizedTx {
                hash: TxHash::new(hash),
                from,
                to,
                value: Amount::new(value, 18),
                gas_fee: None,
                block_number: height,
                status: TxStatus::Success,
                raw: tx,
                contract_address: None,
                log_index: None,
                // A native value transfer row; the calldata method (e.g. an
                // opaque aggregator selector) belongs to the tx, not the row.
                method: Some("transfer".to_string()),
            });
        }
        // ERC20 transfers: parse Transfer(address,address,uint256) logs from receipts.
        let receipts = self.fetch_block_receipts(&block_hex).await.unwrap_or_default();
        // Only swap-emitting or EIP-7702 txs get `debug_traceTransaction`, to
        // keep expensive trace calls to a minimum.
        let mut trace_targets: Vec<(String, TxStatus)> = Vec::new();
        for (i, receipt) in receipts.into_iter().enumerate() {
            let tx_hash = receipt
                .get("transactionHash")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let status = match receipt.get("status").and_then(|v| v.as_str()) {
                Some(s) if s.trim_start_matches("0x") == "0" => TxStatus::Failed,
                _ => TxStatus::Success,
            };
            if let Some(fee) = receipt_gas_fee(&receipt) {
                fee_by_hash.insert(tx_hash.to_string(), fee);
            }
            let logs = receipt.get("logs").and_then(|l| l.as_array());
            if let Some(logs) = logs {
                for log in logs {
                    if let Some(tx) = erc20_transfer_tx(&tx_hash, status, height, i, log) {
                        out.push(tx);
                    }
                }
            }
            if status == TxStatus::Success
                && (authorized.contains(tx_hash)
                    || logs.map(|l| internal::has_swap_event(l.as_slice())).unwrap_or(false))
            {
                trace_targets.push((tx_hash.to_string(), status));
            }
        }
        // Internal native transfers (bounded concurrency to limit debug calls).
        let max_inflight = 8;
        let mut pending: FuturesUnordered<_> = FuturesUnordered::new();
        for (hash, status) in trace_targets {
            let h = hash.clone();
            pending.push(async move {
                let rows = self.fetch_internal_transfers(&h, height, status).await;
                (h, rows)
            });
            if pending.len() >= max_inflight {
                if let Some((_, rows)) = pending.next().await {
                    out.extend(rows);
                }
            }
        }
        while let Some((_, rows)) = pending.next().await {
            out.extend(rows);
        }
        // Backfill the per-tx gas fee onto every row sharing that tx hash
        // (native, ERC20, and internal transfer rows all paid the same fee).
        for row in &mut out {
            if let Some(fee) = fee_by_hash.get(row.hash.as_str()) {
                row.gas_fee = Some(fee.clone());
            }
        }
        Ok(out)
    }

    /// Fetch all receipts for a block. Prefers the batched `eth_getBlockReceipts`
    /// (geth) and falls back to per-transaction receipts on providers without it.
    async fn fetch_block_receipts(&self, block_hex: &str) -> AppResult<Vec<Value>> {
        if let Ok(v) = self.rpc("eth_getBlockReceipts", json!([block_hex])).await {
            if let Some(arr) = v.as_array() {
                return Ok(arr.clone());
            }
        }
        let result = self
            .rpc("eth_getBlockByNumber", json!([block_hex, false]))
            .await?;
        let hashes = result
            .get("transactions")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default();
        let mut receipts = Vec::with_capacity(hashes.len());
        for h in hashes {
            if let Some(h) = h.as_str() {
                if let Ok(r) = self.rpc("eth_getTransactionReceipt", json!([h])).await {
                    if !r.is_null() {
                        receipts.push(r);
                    }
                }
            }
        }
        Ok(receipts)
    }
}

#[async_trait]
impl GasEstimator for EvmChain {
    async fn estimate_gas(&self, tx: &GasEstimateRequest) -> AppResult<GasEstimate> {
        let mut obj = json!({ "from": tx.from.as_str() });
        if let Some(to) = &tx.to {
            obj["to"] = json!(to.as_str());
        }
        if let Some(data) = &tx.data {
            obj["data"] = json!(format!("0x{}", hex::encode(data)));
        }
        let result = self.rpc("eth_estimateGas", json!([obj])).await?;
        let hex = result.as_str().unwrap_or("0x5208");
        let gas_limit = u64::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap_or(21_000);

        // Fetch EIP-1559 fee data
        let fee_result = self
            .rpc("eth_feeHistory", json!([3, "latest", [25, 50, 75]]))
            .await;
        let (max_fee, max_priority) = match fee_result {
            Ok(fee) => {
                let base_fee = fee
                    .pointer("/baseFeePerGas")
                    .and_then(|b| b.as_array())
                    .and_then(|arr| arr.last())
                    .and_then(|v| v.as_str())
                    .and_then(|h| u128::from_str_radix(h.trim_start_matches("0x"), 16).ok())
                    .unwrap_or(1_000_000_000); // 1 gwei fallback

                let priority_fee = fee
                    .pointer("/reward")
                    .and_then(|r| r.as_array())
                    .and_then(|arr| arr.first())
                    .and_then(|r| r.as_array())
                    .and_then(|arr| arr.get(1))
                    .and_then(|v| v.as_str())
                    .and_then(|h| u128::from_str_radix(h.trim_start_matches("0x"), 16).ok())
                    .unwrap_or(1_500_000_000); // 1.5 gwei fallback

                let max_fee = base_fee * 2 + priority_fee;
                (Some(max_fee), Some(priority_fee))
            }
            Err(_) => (None, None),
        };

        Ok(GasEstimate {
            gas_limit,
            max_fee_per_gas: max_fee,
            max_priority_fee_per_gas: max_priority,
            ..GasEstimate::default()
        })
    }
}

#[async_trait]
impl NonceProvider for EvmChain {
    async fn nonce(&self, addr: &Address) -> AppResult<u64> {
        let result = self
            .rpc("eth_getTransactionCount", json!([addr.as_str(), "pending"]))
            .await?;
        let hex = result.as_str().unwrap_or("0x0");
        u64::from_str_radix(hex.trim_start_matches("0x"), 16)
            .map_err(|e| AppError::internal(e.to_string()))
    }
}

/// `keccak256("Transfer(address,address,uint256)")`
const TRANSFER_TOPIC: &str =
    "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";

/// Convert an ERC20 Transfer log into a `NormalizedTx` (token transfer).
fn erc20_transfer_tx(
    tx_hash: &str,
    status: TxStatus,
    block_number: u64,
    tx_index: usize,
    log: &Value,
) -> Option<NormalizedTx> {
    let contract = log.get("address").and_then(|v| v.as_str())?;
    let topics = log.get("topics")?.as_array()?;
    if topics.len() < 3 || topics[0].as_str()? != TRANSFER_TOPIC {
        return None;
    }
    let from = topic_address(topics[1].as_str()?);
    let to = topic_address(topics[2].as_str()?);
    let amount = hex_u256_decimal(log.get("data").and_then(|v| v.as_str())?);
    let log_index = log
        .get("logIndex")
        .and_then(|v| v.as_str())
        .and_then(|h| u64::from_str_radix(h.trim_start_matches("0x"), 16).ok());
    Some(NormalizedTx {
        hash: TxHash::new(tx_hash),
        from,
        to,
        value: Amount::new(amount, 18),
        gas_fee: None,
        block_number,
        status,
        raw: json!({ "log": log, "txIndex": tx_index }),
        contract_address: Some(Address::new(contract.to_string())),
        log_index: log_index.map(|i| i as i64),
        // Rows derived from an ERC20 `Transfer(address,address,uint256)` event;
        // the method is the event itself regardless of the tx calldata.
        method: Some("transfer".to_string()),
    })
}

/// Extract the 20-byte address from a 32-byte (right-padded) log topic.
fn topic_address(topic: &str) -> Option<Address> {
    let hex = topic.strip_prefix("0x").unwrap_or(topic);
    if hex.len() < 40 {
        return None;
    }
    Some(Address::new(format!("0x{}", &hex[hex.len() - 40..])))
}

/// Parse a 32-byte uint256 hex string into a `Decimal`. Overflowing values
/// saturate at `Decimal::MAX` instead of panicking on `Decimal::from(u128)`.
fn hex_u256_decimal(hex: &str) -> Decimal {
    let hex = hex.trim_start_matches("0x");
    let trimmed = hex.trim_start_matches('0');
    match u128::from_str_radix(if trimmed.is_empty() { "0" } else { trimmed }, 16) {
        Ok(v) => wallet_types::decimal_from_u128(v),
        Err(_) => Decimal::ZERO,
    }
}

/// Compute the native gas fee paid from a receipt. OP Stack receipts expose
/// the data-availability charge separately as `l1Fee`; Nitro includes it in
/// `gasUsed` and normally does not expose a separate fee.
fn receipt_gas_fee(receipt: &Value) -> Option<Amount> {
    let gas_used = receipt
        .get("gasUsed")
        .and_then(|v| v.as_str())
        .and_then(|h| u64::from_str_radix(h.trim_start_matches("0x"), 16).ok())?;
    let gas_price = receipt
        .get("effectiveGasPrice")
        .or_else(|| receipt.get("gasPrice"))
        .and_then(|v| v.as_str())
        .and_then(|h| u128::from_str_radix(h.trim_start_matches("0x"), 16).ok())?;
    let l2_fee = (gas_used as u128).checked_mul(gas_price)?;
    let l1_fee = match receipt.get("l1Fee") {
        Some(value) if !value.is_null() => {
            u128::from_str_radix(value.as_str()?.trim_start_matches("0x"), 16).ok()?
        }
        _ => 0,
    };
    Some(Amount::new(
        wallet_types::decimal_from_u128(l2_fee)
            .checked_add(wallet_types::decimal_from_u128(l1_fee))?,
        18,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn topic(address: &str) -> String {
        let addr = address.strip_prefix("0x").unwrap_or(address);
        format!("0x{}", "0".repeat(24) + addr)
    }

    #[test]
    fn test_erc20_transfer_parse() {
        let log = json!({
            "address": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
            "topics": [
                "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
                topic("0x1111111111111111111111111111111111111111"),
                topic("0x2222222222222222222222222222222222222222"),
            ],
            "data": "0x0000000000000000000000000000000000000000000000000000000000000064",
            "logIndex": "0x3",
        });
        let tx = erc20_transfer_tx("0xabc", TxStatus::Success, 42, 0, &log).unwrap();
        assert_eq!(tx.contract_address.unwrap().as_str(), "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");
        assert_eq!(tx.from.unwrap().as_str(), "0x1111111111111111111111111111111111111111");
        assert_eq!(tx.to.unwrap().as_str(), "0x2222222222222222222222222222222222222222");
        assert_eq!(tx.value.raw, Decimal::from(100));
        assert_eq!(tx.block_number, 42);
        assert_eq!(tx.log_index, Some(3));
        assert_eq!(tx.method.as_deref(), Some("transfer"));
    }

    #[test]
    fn test_erc20_ignores_non_transfer_topic() {
        let log = json!({
            "address": "0xabc",
            "topics": ["0x0000000000000000000000000000000000000000000000000000000000000000",
                       topic("0x1111111111111111111111111111111111111111"),
                       topic("0x2222222222222222222222222222222222222222")],
            "data": "0x64",
        });
        assert!(erc20_transfer_tx("0x1", TxStatus::Success, 1, 0, &log).is_none());
    }

    #[test]
    fn test_hex_u256_decimal() {
        assert_eq!(hex_u256_decimal("0x0"), Decimal::ZERO);
        assert_eq!(hex_u256_decimal("0x64"), Decimal::from(100));
        assert_eq!(hex_u256_decimal("0x000000000000000000000000000000000000000000000000000000000000000f"), Decimal::from(15));
    }

    #[test]
    fn test_receipt_gas_fee_effective_price() {
        let receipt = json!({
            "gasUsed": "0x5208",
            "effectiveGasPrice": "0x3b9aca00",
        });
        let fee = receipt_gas_fee(&receipt).unwrap();
        assert_eq!(fee.decimals, 18);
        assert_eq!(fee.raw, Decimal::from(21_000u64 * 1_000_000_000u64));
    }

    #[test]
    fn test_receipt_gas_fee_falls_back_to_gas_price() {
        let receipt = json!({
            "gasUsed": "0x5208",
            "gasPrice": "0x3b9aca00",
        });
        let fee = receipt_gas_fee(&receipt).unwrap();
        assert_eq!(fee.raw, Decimal::from(21_000u64 * 1_000_000_000u64));
    }

    #[test]
    fn test_receipt_gas_fee_missing_fields() {
        assert!(receipt_gas_fee(&json!({})).is_none());
        assert!(receipt_gas_fee(&json!({ "gasUsed": "0x5208" })).is_none());
    }

    #[test]
    fn test_receipt_gas_fee_includes_separate_l1_fee() {
        let receipt = json!({
            "gasUsed": "0x5208",
            "effectiveGasPrice": "0x3b9aca00",
            "l1Fee": "0x64",
        });
        assert_eq!(
            receipt_gas_fee(&receipt).unwrap().raw,
            Decimal::from(21_000u64 * 1_000_000_000u64 + 100)
        );
    }

    #[test]
    fn test_receipt_gas_fee_rejects_invalid_l1_fee() {
        let receipt = json!({
            "gasUsed": "0x5208",
            "effectiveGasPrice": "0x1",
            "l1Fee": "invalid",
        });
        assert!(receipt_gas_fee(&receipt).is_none());
    }
}
