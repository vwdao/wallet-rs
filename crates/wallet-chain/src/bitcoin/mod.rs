use crate::provider::RpcPool;
use crate::traits::{BalanceReader, BlockSource, GasEstimator, TokenBalance, TxBroadcaster};
use async_trait::async_trait;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::sync::Arc;
use wallet_config::ChainRuntimeConfig;
use wallet_error::{AppError, AppResult};
use wallet_types::{Address, Amount, ChainIndex, NormalizedTx, TxHash, TxStatus};
use wallet_types::{GasEstimate, GasEstimateRequest};

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
        if let Some(err) = v.get("error").filter(|e| !e.is_null()) {
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
            .rpc("getreceivedbyaddress", json!([addr.as_str(), 0]))
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
        let result = self.rpc("sendrawtransaction", json!([hex_str])).await?;
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
        // Verbosity 3 includes vin[].prevout (address + value) for fee/from.
        let block = self.rpc("getblock", json!([hash, 3])).await?;
        let txs = block
            .get("tx")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default();
        let mut result = Vec::new();
        for tx in &txs {
            result.extend(expand_bitcoin_tx(tx, height));
        }
        Ok(result)
    }
}

fn btc_to_sats(v: f64) -> u64 {
    (v * 1e8).round() as u64
}

fn script_address(script: &Value) -> Option<String> {
    if let Some(a) = script.get("address").and_then(|v| v.as_str()) {
        return Some(a.to_string());
    }
    script
        .get("addresses")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Expand one getblock verbosity-3 tx into payment rows (one per non-change output).
pub(crate) fn expand_bitcoin_tx(tx: &Value, height: u64) -> Vec<NormalizedTx> {
    let vin = tx
        .get("vin")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    if vin.first().and_then(|v| v.get("coinbase")).is_some() {
        return Vec::new();
    }
    let txid = tx.get("txid").and_then(|v| v.as_str()).unwrap_or("");
    let vout = tx
        .get("vout")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let mut input_addrs: Vec<String> = Vec::new();
    let mut input_addr_set = std::collections::HashSet::new();
    let mut total_in: u64 = 0;
    let mut prevouts_complete = !vin.is_empty();
    for i in &vin {
        match i.get("prevout") {
            Some(p) => {
                if let Some(v) = p.get("value").and_then(|v| v.as_f64()) {
                    total_in += btc_to_sats(v);
                } else {
                    prevouts_complete = false;
                }
                if let Some(spk) = p.get("scriptPubKey") {
                    if let Some(a) = script_address(spk) {
                        input_addr_set.insert(a.clone());
                        input_addrs.push(a);
                    }
                }
            }
            None => prevouts_complete = false,
        }
    }

    let mut outputs: Vec<(u64, String, u64)> = Vec::new();
    let mut output_addr_set = std::collections::HashSet::new();
    let mut total_out: u64 = 0;
    for o in &vout {
        let n = o.get("n").and_then(|v| v.as_u64()).unwrap_or(0);
        let sats = o
            .get("value")
            .and_then(|v| v.as_f64())
            .map(btc_to_sats)
            .unwrap_or(0);
        total_out += sats;
        if let Some(a) = o.get("scriptPubKey").and_then(script_address) {
            output_addr_set.insert(a.clone());
            outputs.push((n, a, sats));
        }
    }

    let from = input_addrs
        .iter()
        .find(|a| !output_addr_set.contains(*a))
        .cloned()
        .or_else(|| input_addrs.first().cloned());
    if from.is_none() && !vin.is_empty() {
        tracing::warn!(txid, "bitcoin tx missing input addresses (no prevout?)");
    }

    let gas_fee = if prevouts_complete && total_in > total_out {
        Some(Amount::new(Decimal::from(total_in - total_out), 8))
    } else {
        None
    };

    let mut rows = Vec::new();
    for (n, to_addr, sats) in outputs {
        // Change: any output that pays back to an input address.
        if input_addr_set.contains(&to_addr) {
            continue;
        }
        rows.push(NormalizedTx {
            hash: TxHash::new(txid),
            from: from.as_ref().map(|a| Address::new(a.clone())),
            to: Some(Address::new(to_addr)),
            value: Amount::new(Decimal::from(sats), 8),
            gas_fee: gas_fee.clone(),
            block_number: height,
            status: TxStatus::Success,
            raw: json!({ "txid": txid, "vout": n }),
            contract_address: None,
            log_index: Some(n),
            method: Some("native_transfer".into()),
        });
    }
    rows
}

#[async_trait]
impl GasEstimator for BitcoinChain {
    async fn estimate_gas(&self, _tx: &GasEstimateRequest) -> AppResult<GasEstimate> {
        let v = self.rpc("estimatesmartfee", json!([6])).await?;
        let feerate = v.get("feerate").and_then(|f| f.as_f64()).unwrap_or(0.0001);
        // Convert BTC/kB to sat/vB (divide by 100_000_000/1000 = 100_000)
        let sat_per_vb = (feerate * 1e8 / 1000.0) as u128;
        Ok(GasEstimate {
            gas_limit: 250,
            max_fee_per_gas: Some(sat_per_vb),
            max_priority_fee_per_gas: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn payment_tx() -> Value {
        json!({
            "txid": "abc",
            "vin": [{
                "txid": "prev",
                "vout": 0,
                "prevout": {
                    "value": 1.0,
                    "scriptPubKey": { "address": "bc1from" }
                }
            }],
            "vout": [
                { "n": 0, "value": 0.4, "scriptPubKey": { "address": "bc1to" } },
                { "n": 1, "value": 0.5999, "scriptPubKey": { "address": "bc1from" } }
            ]
        })
    }

    #[test]
    fn skips_change_keeps_payment() {
        let rows = expand_bitcoin_tx(&payment_tx(), 100);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].to.as_ref().unwrap().as_str(), "bc1to");
        assert_eq!(rows[0].from.as_ref().unwrap().as_str(), "bc1from");
        assert_eq!(rows[0].log_index, Some(0));
        assert_eq!(rows[0].method.as_deref(), Some("native_transfer"));
        assert!(rows[0].gas_fee.is_some());
    }

    #[test]
    fn skips_coinbase() {
        let tx = json!({
            "txid": "coin",
            "vin": [{ "coinbase": "04..." }],
            "vout": [{ "n": 0, "value": 6.25, "scriptPubKey": { "address": "bc1miner" } }]
        });
        assert!(expand_bitcoin_tx(&tx, 1).is_empty());
    }

    #[test]
    fn skips_op_return() {
        let tx = json!({
            "txid": "x",
            "vin": [{
                "prevout": { "value": 0.1, "scriptPubKey": { "address": "bc1a" } }
            }],
            "vout": [
                { "n": 0, "value": 0.0, "scriptPubKey": { "type": "nulldata" } },
                { "n": 1, "value": 0.09, "scriptPubKey": { "address": "bc1b" } }
            ]
        });
        let rows = expand_bitcoin_tx(&tx, 2);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].log_index, Some(1));
        assert_eq!(rows[0].to.as_ref().unwrap().as_str(), "bc1b");
    }

    #[test]
    fn no_prevout_still_splits() {
        let tx = json!({
            "txid": "y",
            "vin": [{ "txid": "p", "vout": 0 }],
            "vout": [
                { "n": 0, "value": 0.01, "scriptPubKey": { "address": "bc1c" } },
                { "n": 1, "value": 0.02, "scriptPubKey": { "address": "bc1d" } }
            ]
        });
        let rows = expand_bitcoin_tx(&tx, 3);
        assert_eq!(rows.len(), 2);
        assert!(rows[0].from.is_none());
        assert!(rows[0].gas_fee.is_none());
        assert_eq!(rows[0].log_index, Some(0));
        assert_eq!(rows[1].log_index, Some(1));
        assert!(rows[1].gas_fee.is_none());
    }

    #[test]
    fn multi_payment_same_fee_on_all_rows() {
        let tx = json!({
            "txid": "z",
            "vin": [{
                "prevout": { "value": 1.0, "scriptPubKey": { "address": "bc1s" } }
            }],
            "vout": [
                { "n": 0, "value": 0.3, "scriptPubKey": { "address": "bc1p1" } },
                { "n": 1, "value": 0.3, "scriptPubKey": { "address": "bc1p2" } },
                { "n": 2, "value": 0.399, "scriptPubKey": { "address": "bc1s" } }
            ]
        });
        let rows = expand_bitcoin_tx(&tx, 4);
        assert_eq!(rows.len(), 2);
        let fee0 = rows[0].gas_fee.as_ref().expect("fee on first");
        let fee1 = rows[1].gas_fee.as_ref().expect("fee on second");
        assert_eq!(fee0.raw, fee1.raw);
        assert_eq!(fee0.raw, Decimal::from(100_000u64)); // 1.0 - 0.999 BTC
    }

    #[test]
    fn from_prefers_non_change_input() {
        let tx = json!({
            "txid": "w",
            "vin": [
                { "prevout": { "value": 0.5, "scriptPubKey": { "address": "bc1change" } } },
                { "prevout": { "value": 0.5, "scriptPubKey": { "address": "bc1real" } } }
            ],
            "vout": [
                { "n": 0, "value": 0.7, "scriptPubKey": { "address": "bc1pay" } },
                { "n": 1, "value": 0.299, "scriptPubKey": { "address": "bc1change" } }
            ]
        });
        let rows = expand_bitcoin_tx(&tx, 5);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].from.as_ref().unwrap().as_str(), "bc1real");
        assert_eq!(rows[0].to.as_ref().unwrap().as_str(), "bc1pay");
    }
}
