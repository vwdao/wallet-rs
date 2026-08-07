use crate::provider::RpcPool;
use crate::traits::{BalanceReader, BlockSource, GasEstimator, TokenBalance, TxBroadcaster};
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::sync::Arc;
use wallet_config::ChainRuntimeConfig;
use wallet_error::{AppError, AppResult};
use wallet_types::{
    Address, Amount, ChainIndex, GasEstimate, GasEstimateRequest, NormalizedTx, TxHash, TxStatus,
};

#[derive(Debug)]
pub struct SolanaChain {
    pub chain_index: ChainIndex,
    pub pool: RpcPool,
}

impl SolanaChain {
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
        if let Some(err) = v.get("error").filter(|e| !e.is_null()) {
            self.pool.mark_failure(&url);
            return Err(AppError::Unavailable(err.to_string()));
        }
        self.pool.mark_success(&url);
        Ok(v.get("result").cloned().unwrap_or(Value::Null))
    }

    /// Derive SPL Token Associated Token Account (ATA) address via RPC lookup.
    /// Uses getTokenAccountsByOwner to find existing ATA for a given mint.
    /// Returns (address, exists).
    pub async fn ata_address_rpc(
        &self,
        owner: &Address,
        mint: &Address,
    ) -> AppResult<(Address, bool)> {
        let result = self
            .rpc(
                "getTokenAccountsByOwner",
                json!([
                    owner.as_str(),
                    { "mint": mint.as_str() },
                    { "encoding": "jsonParsed" }
                ]),
            )
            .await?;
        let accounts = result
            .get("value")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        if let Some(first) = accounts.first() {
            let pubkey = first.get("pubkey").and_then(|p| p.as_str()).unwrap_or("");
            return Ok((Address::new(pubkey), true));
        }
        let ata = Address::new(format!("ata:{}:{}", owner.as_str(), mint.as_str()));
        Ok((ata, false))
    }

    /// Derive SPL Token Associated Token Account (ATA) address (sync fallback).
    /// Falls back to placeholder when RPC lookup is not available.
    pub fn ata_address(owner: &Address, mint: &Address) -> Address {
        Address::new(format!("ata:{}:{}", owner.as_str(), mint.as_str()))
    }
}

#[async_trait]
impl BalanceReader for SolanaChain {
    async fn native_balance(&self, addr: &Address) -> AppResult<Amount> {
        let result = self.rpc("getBalance", json!([addr.as_str()])).await?;
        let lamports = result.get("value").and_then(|v| v.as_u64()).unwrap_or(0);
        Ok(Amount::new(Decimal::from(lamports), 9))
    }
}

#[async_trait]
impl TokenBalance for SolanaChain {
    async fn token_balance(&self, wallet: &Address, token: &Address) -> AppResult<Amount> {
        // Query token accounts by owner, filtered by mint
        let result = self
            .rpc(
                "getTokenAccountsByOwner",
                json!([
                    wallet.as_str(),
                    { "mint": token.as_str() },
                    { "encoding": "jsonParsed" }
                ]),
            )
            .await?;
        let accounts = result
            .get("value")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        if let Some(first) = accounts.first() {
            let amount = first
                .pointer("/account/data/parsed/info/tokenAmount/amount")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
            let decimals = first
                .pointer("/account/data/parsed/info/tokenAmount/decimals")
                .and_then(|v| v.as_u64())
                .unwrap_or(6) as u32;
            Ok(Amount::new(Decimal::from(amount), decimals))
        } else {
            Ok(Amount::zero(6))
        }
    }
}

#[async_trait]
impl TxBroadcaster for SolanaChain {
    async fn send_raw(&self, raw: &[u8]) -> AppResult<TxHash> {
        let b64 = STANDARD.encode(raw);
        let result = self
            .rpc("sendTransaction", json!([b64, { "encoding": "base64" }]))
            .await?;
        let hash = result
            .as_str()
            .ok_or_else(|| AppError::internal("missing solana sig"))?;
        Ok(TxHash::new(hash))
    }
}

#[async_trait]
impl BlockSource for SolanaChain {
    async fn tip(&self) -> AppResult<u64> {
        let result = self.rpc("getSlot", json!([])).await?;
        Ok(result.as_u64().unwrap_or(0))
    }

    /// Fetch block transactions using `transactionDetails: "accounts"` mode.
    /// Note: the `value` field in NormalizedTx is not available in this mode
    /// and defaults to zero. Use `getBalance` RPC for actual balance queries.
    async fn fetch_block_txs(&self, height: u64) -> AppResult<Vec<NormalizedTx>> {
        let result = self
            .rpc(
                "getBlock",
                json!([
                    height,
                    {
                        "encoding": "json",
                        "transactionDetails": "accounts",
                        "rewards": false,
                        "maxSupportedTransactionVersion": 0
                    }
                ]),
            )
            .await?;
        let txs = result
            .get("transactions")
            .and_then(|s| s.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(txs
            .into_iter()
            .map(|tx| {
                let sig = tx
                    .get("transaction")
                    .and_then(|t| t.get("signatures"))
                    .and_then(|s| s.as_array())
                    .and_then(|s| s.first())
                    .and_then(|s| s.as_str())
                    .unwrap_or("unknown");
                let account_keys = tx
                    .get("transaction")
                    .and_then(|t| t.get("accountKeys"))
                    .and_then(|a| a.as_array())
                    .cloned()
                    .unwrap_or_default();
                let fee_payer = account_keys.first().and_then(|k| {
                    if let Some(s) = k.as_str() {
                        Some(Address::new(s.to_string()))
                    } else {
                        k.get("pubkey")
                            .and_then(|p| p.as_str())
                            .map(|s| Address::new(s.to_string()))
                    }
                });
                let from = fee_payer;
                let to = account_keys.get(1).and_then(|k| {
                    if let Some(s) = k.as_str() {
                        Some(Address::new(s.to_string()))
                    } else {
                        k.get("pubkey")
                            .and_then(|p| p.as_str())
                            .map(|s| Address::new(s.to_string()))
                    }
                });
                NormalizedTx {
                    hash: TxHash::new(sig),
                    from,
                    to,
                    value: Amount::zero(9),
                    block_number: height,
                    status: TxStatus::Success,
                    raw: json!({ "signature": sig }),
                }
            })
            .collect())
    }
}

#[async_trait]
impl GasEstimator for SolanaChain {
    async fn estimate_gas(&self, _tx: &GasEstimateRequest) -> AppResult<GasEstimate> {
        let result = self.rpc("getRecentPrioritizationFees", json!([])).await?;
        let fees = result.as_array().cloned().unwrap_or_default();
        let avg_priority: u64 = if fees.is_empty() {
            5000
        } else {
            let sum: u64 = fees
                .iter()
                .filter_map(|f| f.get("prioritizationFee").and_then(|v| v.as_u64()))
                .sum();
            sum / fees.len() as u64
        };
        Ok(GasEstimate {
            gas_limit: 200_000,
            max_fee_per_gas: Some((5000 + avg_priority) as u128),
            max_priority_fee_per_gas: Some(avg_priority as u128),
        })
    }
}
