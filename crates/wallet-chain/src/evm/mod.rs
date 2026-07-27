use crate::provider::RpcPool;
use crate::traits::{
    BalanceReader, BlockSource, GasEstimator, NonceProvider, TokenBalance, TxBroadcaster,
};
use async_trait::async_trait;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::sync::Arc;
use wallet_config::ChainRuntimeConfig;
use wallet_error::{AppError, AppResult};
use wallet_types::{
    Address, Amount, GasEstimate, GasEstimateRequest, NormalizedTx, TxHash, TxStatus, ChainIndex,
};

#[derive(Debug)]
pub struct EvmChain {
    pub chain_index: ChainIndex,
    pub chain_id: u64,
    pub pool: RpcPool,
    pub confirmations: u64,
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
        }))
    }

    async fn rpc(&self, method: &str, params: Value) -> AppResult<Value> {
        let url = self.pool.next_url()?.to_string();
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params,
        });
        let resp = self
            .pool
            .client()
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                self.pool.mark_failure(&url);
                AppError::Unavailable(e.to_string())
            })?;
        let v: Value = resp.json().await.map_err(|e| {
            self.pool.mark_failure(&url);
            AppError::Unavailable(e.to_string())
        })?;
        if let Some(err) = v.get("error") {
            self.pool.mark_failure(&url);
            return Err(AppError::Unavailable(err.to_string()));
        }
        self.pool.mark_success(&url);
        Ok(v.get("result").cloned().unwrap_or(Value::Null))
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
        Ok(Amount::new(Decimal::from(raw), 18))
    }
}

#[async_trait]
impl TokenBalance for EvmChain {
    async fn token_balance(&self, wallet: &Address, token: &Address) -> AppResult<Amount> {
        // First, fetch decimals from the token contract: decimals() = 0x313ce567
        let decimals_result = self
            .rpc(
                "eth_call",
                json!([{ "to": token.as_str(), "data": "0x313ce567" }, "latest"]),
            )
            .await;
        let decimals = decimals_result
            .ok()
            .and_then(|r| r.as_str().map(|s| s.to_string()))
            .and_then(|hex| {
                u32::from_str_radix(hex.trim_start_matches("0x"), 16).ok()
            })
            .unwrap_or(18);

        // balanceOf(address) selector 0x70a08231
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
        Ok(Amount::new(Decimal::from(raw), decimals))
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
        let hex = result.as_str().unwrap_or("0x0");
        u64::from_str_radix(hex.trim_start_matches("0x"), 16)
            .map_err(|e| AppError::internal(e.to_string()))
    }

    async fn fetch_block_txs(&self, height: u64) -> AppResult<Vec<NormalizedTx>> {
        let block_hex = format!("0x{height:x}");
        let result = self
            .rpc("eth_getBlockByNumber", json!([block_hex, true]))
            .await?;
        let txs = result
            .get("transactions")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default();
        let mut out = Vec::with_capacity(txs.len());
        for tx in txs {
            let hash = tx
                .get("hash")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let from = tx
                .get("from")
                .and_then(|v| v.as_str())
                .map(Address::new);
            let to = tx.get("to").and_then(|v| v.as_str()).map(Address::new);
            let value_hex = tx.get("value").and_then(|v| v.as_str()).unwrap_or("0x0");
            let value = u128::from_str_radix(value_hex.trim_start_matches("0x"), 16).unwrap_or(0);
            out.push(NormalizedTx {
                hash: TxHash::new(hash),
                from,
                to,
                value: Amount::new(Decimal::from(value), 18),
                block_number: height,
                status: TxStatus::Success,
                raw: tx,
            });
        }
        Ok(out)
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
        let fee_result = self.rpc("eth_feeHistory", json!([3, "latest", [25, 50, 75]])).await;
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
        })
    }
}

#[async_trait]
impl NonceProvider for EvmChain {
    async fn nonce(&self, addr: &Address) -> AppResult<u64> {
        let result = self
            .rpc(
                "eth_getTransactionCount",
                json!([addr.as_str(), "pending"]),
            )
            .await?;
        let hex = result.as_str().unwrap_or("0x0");
        u64::from_str_radix(hex.trim_start_matches("0x"), 16)
            .map_err(|e| AppError::internal(e.to_string()))
    }
}
