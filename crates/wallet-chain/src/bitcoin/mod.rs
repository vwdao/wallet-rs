use crate::provider::RpcPool;
use crate::traits::{BalanceReader, BlockSource, GasEstimator, TokenBalance, TxBroadcaster};
use wallet_types::{GasEstimate, GasEstimateRequest};
use async_trait::async_trait;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::sync::Arc;
use wallet_config::ChainRuntimeConfig;
use wallet_error::{AppError, AppResult};
use wallet_types::{Address, Amount, NormalizedTx, TxHash, TxStatus, ChainIndex};

#[derive(Debug)]
pub struct BitcoinChain {
    pub chain_index: ChainIndex,
    pub pool: RpcPool,
}

impl BitcoinChain {
    pub fn new(cfg: &ChainRuntimeConfig) -> AppResult<Arc<Self>> {
        Ok(Arc::new(Self {
            chain_index: ChainIndex(cfg.chain_index),
            pool: RpcPool::new(cfg.endpoints.clone())?,
        }))
    }

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
impl BalanceReader for BitcoinChain {
    async fn native_balance(&self, addr: &Address) -> AppResult<Amount> {
        let v = self
            .rpc(
                "getreceivedbyaddress",
                json!([addr.as_str(), 0]),
            )
            .await?;
        let btc = v.as_f64().unwrap_or(0.0);
        let sats = (btc * 1e8) as u64;
        Ok(Amount::new(Decimal::from(sats), 8))
    }
}

#[async_trait]
impl TokenBalance for BitcoinChain {
    async fn token_balance(&self, _wallet: &Address, _token: &Address) -> AppResult<Amount> {
        // No SPL-like tokens on Bitcoin mainnet
        Ok(Amount::zero(8))
    }
}

#[async_trait]
impl TxBroadcaster for BitcoinChain {
    async fn send_raw(&self, raw: &[u8]) -> AppResult<TxHash> {
        let hex_str = hex::encode(raw);
        let result = self
            .rpc("sendrawtransaction", json!([hex_str]))
            .await?;
        let hash = result
            .as_str()
            .ok_or_else(|| AppError::internal("missing txid"))?;
        Ok(TxHash::new(hash))
    }
}

#[async_trait]
impl BlockSource for BitcoinChain {
    async fn tip(&self) -> AppResult<u64> {
        let result = self.rpc("getblockcount", json!([])).await?;
        Ok(result.as_u64().unwrap_or(0))
    }

    async fn fetch_block_txs(&self, height: u64) -> AppResult<Vec<NormalizedTx>> {
        let hash: String = self
            .rpc("getblockhash", json!([height]))
            .await?
            .as_str()
            .unwrap_or("")
            .to_string();
        let block = self
            .rpc(
                "getblock",
                json!([hash, 2]),
            )
            .await?;
        let txs = block
            .get("tx")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default();
        let mut result = Vec::with_capacity(txs.len());
        for tx in &txs {
            let txid = tx.get("txid").and_then(|v| v.as_str()).unwrap_or("");
            let total_out: u64 = tx
                .get("vout")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|o| o.get("value").and_then(|v| v.as_f64()))
                        .map(|f| (f * 1e8) as u64)
                        .sum()
                })
                .unwrap_or(0);
            let first_input_addr = tx
                .get("vin")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.first())
                .and_then(|i| i.get("address").and_then(|a| a.as_str()))
                .map(|s| Address::new(s.to_string()));
            let first_output_addr = tx
                .get("vout")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.first())
                .and_then(|o| o.get("address").and_then(|a| a.as_str()))
                .map(|s| Address::new(s.to_string()));
            result.push(NormalizedTx {
                hash: TxHash::new(txid),
                from: first_input_addr,
                to: first_output_addr,
                value: Amount::new(Decimal::from(total_out), 8),
                block_number: height,
                status: TxStatus::Success,
                raw: json!({ "txid": txid }),
            });
        }
        Ok(result)
    }
}

#[async_trait]
impl GasEstimator for BitcoinChain {
    async fn estimate_gas(&self, _tx: &GasEstimateRequest) -> AppResult<GasEstimate> {
        let v = self
            .rpc("estimatesmartfee", json!([6]))
            .await?;
        let feerate = v
            .get("feerate")
            .and_then(|f| f.as_f64())
            .unwrap_or(0.0001);
        // Convert BTC/kB to sat/vB (divide by 100_000_000/1000 = 100_000)
        let sat_per_vb = (feerate * 1e8 / 1000.0) as u128;
        Ok(GasEstimate {
            gas_limit: 250,
            max_fee_per_gas: Some(sat_per_vb),
            max_priority_fee_per_gas: None,
        })
    }
}
