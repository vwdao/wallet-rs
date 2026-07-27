use crate::provider::RpcPool;
use crate::traits::{BalanceReader, BlockSource, GasEstimator, TokenBalance, TxBroadcaster};
use async_trait::async_trait;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::sync::Arc;
use wallet_config::ChainRuntimeConfig;
use wallet_error::{AppError, AppResult};
use wallet_types::{Address, Amount, GasEstimate, GasEstimateRequest, NormalizedTx, TxHash, TxStatus, ChainIndex};

#[derive(Debug)]
pub struct TronChain {
    pub chain_index: ChainIndex,
    pub pool: RpcPool,
}

impl TronChain {
    pub fn new(cfg: &ChainRuntimeConfig) -> AppResult<Arc<Self>> {
        Ok(Arc::new(Self {
            chain_index: ChainIndex(cfg.chain_index),
            pool: RpcPool::new(cfg.endpoints.clone())?,
        }))
    }

    async fn post(&self, path: &str, body: Value) -> AppResult<Value> {
        let base = self.pool.next_url()?.to_string();
        let url = format!("{}{}", base.trim_end_matches('/'), path);
        let resp = self
            .pool
            .client()
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                self.pool.mark_failure(&base);
                AppError::Unavailable(e.to_string())
            })?;
        let v: Value = resp.json().await.map_err(|e| {
            self.pool.mark_failure(&base);
            AppError::Unavailable(e.to_string())
        })?;
        self.pool.mark_success(&base);
        Ok(v)
    }
}

#[async_trait]
impl BalanceReader for TronChain {
    async fn native_balance(&self, addr: &Address) -> AppResult<Amount> {
        let v = self
            .post("/wallet/getaccount", json!({ "address": addr.as_str(), "visible": true }))
            .await?;
        let balance = v.get("balance").and_then(|b| b.as_i64()).unwrap_or(0);
        Ok(Amount::new(Decimal::from(balance), 6))
    }
}

#[async_trait]
impl TxBroadcaster for TronChain {
    async fn send_raw(&self, raw: &[u8]) -> AppResult<TxHash> {
        let v = self
            .post(
                "/wallet/broadcasthex",
                json!({ "transaction": hex::encode(raw) }),
            )
            .await?;
        let hash = v
            .get("txid")
            .and_then(|t| t.as_str())
            .unwrap_or_default();
        Ok(TxHash::new(hash))
    }
}

#[async_trait]
impl BlockSource for TronChain {
    async fn tip(&self) -> AppResult<u64> {
        let v = self.post("/wallet/getnowblock", json!({})).await?;
        Ok(v.pointer("/block_header/raw_data/number")
            .and_then(|n| n.as_u64())
            .unwrap_or(0))
    }

    async fn fetch_block_txs(&self, height: u64) -> AppResult<Vec<NormalizedTx>> {
        let v = self
            .post("/wallet/getblockbynum", json!({ "num": height }))
            .await?;
        let txs = v
            .get("transactions")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(txs
            .into_iter()
            .map(|tx| {
                let hash = tx
                    .get("txID")
                    .and_then(|h| h.as_str())
                    .unwrap_or_default();
                let raw_data = tx.get("raw_data");
                let from = raw_data
                    .and_then(|r| r.get("contract"))
                    .and_then(|c| c.as_array())
                    .and_then(|arr| arr.first())
                    .and_then(|c| c.get("parameter"))
                    .and_then(|p| p.get("value"))
                    .and_then(|v| v.get("owner_address"))
                    .and_then(|v| v.as_str())
                    .map(Address::new);
                let to = raw_data
                    .and_then(|r| r.get("contract"))
                    .and_then(|c| c.as_array())
                    .and_then(|arr| arr.first())
                    .and_then(|c| c.get("parameter"))
                    .and_then(|p| p.get("value"))
                    .and_then(|v| v.get("to_address"))
                    .and_then(|v| v.as_str())
                    .map(Address::new);
                let value = raw_data
                    .and_then(|r| r.get("contract"))
                    .and_then(|c| c.as_array())
                    .and_then(|arr| arr.first())
                    .and_then(|c| c.get("parameter"))
                    .and_then(|p| p.get("value"))
                    .and_then(|v| v.get("amount"))
                    .and_then(|v| v.as_i64())
                    .map(|a| Amount::new(Decimal::from(a), 6))
                    .unwrap_or_else(|| Amount::zero(6));
                NormalizedTx {
                    hash: TxHash::new(hash),
                    from,
                    to,
                    value,
                    block_number: height,
                    status: TxStatus::Success,
                    raw: tx,
                }
            })
            .collect())
    }
}

#[async_trait]
impl GasEstimator for TronChain {
    async fn estimate_gas(&self, tx: &GasEstimateRequest) -> AppResult<GasEstimate> {
        if let Some(to_addr) = &tx.to {
            let data_hex = tx
                .data
                .as_ref()
                .map(|d| format!("0x{}", hex::encode(d)))
                .unwrap_or_else(|| "0x06feacc0".to_string());
            let _selector = if data_hex.len() >= 10 {
                &data_hex[2..10]
            } else {
                ""
            };
            let params: Vec<String> = if let Some(raw) = tx.data.as_ref() {
                if raw.len() > 4 {
                    raw.chunks(32)
                        .map(hex::encode)
                        .collect()
                } else {
                    vec![]
                }
            } else {
                vec![]
            };
            let mut body = json!({
                "owner_address": tx.from.as_str(),
                "contract_address": to_addr.as_str(),
                "function_selector": "estimateEnergy(address)",
                "parameter": params,
                "visible": true,
            });
            if let Some(val) = &tx.value {
                body["call_value"] = json!(val.raw.to_string());
            }
            let v = self.post("/wallet/triggerconstantcontract", body).await?;
            if let Some(constant_result) = v.get("constant_result").and_then(|r| r.as_array()) {
                if let Some(first) = constant_result.first() {
                    let hex_str = first.as_str().unwrap_or("0");
                    let energy = u64::from_str_radix(hex_str, 16).unwrap_or(65_000);
                    return Ok(GasEstimate {
                        gas_limit: energy.max(65_000),
                        max_fee_per_gas: None,
                        max_priority_fee_per_gas: None,
                    });
                }
            }
        }
        Ok(GasEstimate {
            gas_limit: 65_000,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        })
    }
}

#[async_trait]
impl TokenBalance for TronChain {
    async fn token_balance(&self, wallet: &Address, token: &Address) -> AppResult<Amount> {
        // Use TRON RPC to query TRC20 balance
        let v = self
            .post(
                "/wallet/triggerconstantcontract",
                json!({
                    "owner_address": wallet.as_str(),
                    "contract_address": token.as_str(),
                    "function_selector": "balanceOf(address)",
                    "parameter": [wallet.as_str()],
                    "visible": true,
                }),
            )
            .await?;
        if let Some(constant_result) = v.get("constant_result").and_then(|r| r.as_array()) {
            if let Some(first) = constant_result.first() {
                let hex_str = first.as_str().unwrap_or("0");
                let balance = u64::from_str_radix(hex_str, 16).unwrap_or(0);
                return Ok(Amount::new(Decimal::from(balance), 6));
            }
        }
        Ok(Amount::zero(6))
    }
}
