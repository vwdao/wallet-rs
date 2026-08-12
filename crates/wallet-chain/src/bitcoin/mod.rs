use crate::provider::RpcPool;
use crate::traits::{BalanceReader, BlockSource, GasEstimator, TokenBalance, TxBroadcaster};
use async_trait::async_trait;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use wallet_config::ChainRuntimeConfig;
use wallet_error::{AppError, AppResult};
use wallet_types::{Address, Amount, ChainIndex, NormalizedTx, TxHash, TxStatus};
use wallet_types::{GasEstimate, GasEstimateRequest};

/// How many `getrawtransaction` prevout lookups to run in parallel per chunk.
const PREVOUT_FETCH_CONCURRENCY: usize = 8;

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
        // Verbosity 3 includes vin[].prevout (address + value) on Bitcoin, but
        // dogecoin-core omits it, so input addresses/values are resolved via
        // `getrawtransaction` below when a vin entry has no embedded prevout.
        let block = self.rpc("getblock", json!([hash, 3])).await?;
        let raw_txs = block
            .get("tx")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default();

        // Some public dogecoin gateways ignore the verbosity flag and return
        // verbosity-1 shaped blocks (`tx` = txid strings). Every DOGE/BTC
        // block carries at least the coinbase tx, so an empty `tx` list or a
        // string-typed one means the response is unusable — silently treating
        // it as a 0-tx block would drop real transactions. Reconstruct txid
        // lists via `getrawtransaction`; fail loudly on anything else so the
        // cursor holds and the next tick retries.
        let txs: Vec<Value> = match classify_block_tx_shape(&raw_txs) {
            BlockTxShape::Objects => raw_txs,
            BlockTxShape::Txids => {
                tracing::warn!(
                    chain = %self.chain_index,
                    height,
                    txids = raw_txs.len(),
                    "getblock verbosity 3 returned a txid list; reconstructing via getrawtransaction"
                );
                reconstruct_block_txs(self, &raw_txs).await
            }
            BlockTxShape::EmptyOrMalformed => {
                return Err(AppError::Unavailable(format!(
                    "getblock verbosity 3 returned no usable tx data at height {height}"
                )));
            }
        };

        let mut prev_txids = HashSet::new();
        for tx in &txs {
            let Some(vin) = tx.get("vin").and_then(|v| v.as_array()) else {
                continue;
            };
            if vin
                .first()
                .and_then(|v| v.get("coinbase"))
                .is_some()
            {
                continue;
            }
            for i in vin {
                if i.get("prevout").is_none() {
                    if let Some(t) = i.get("txid").and_then(|v| v.as_str()) {
                        prev_txids.insert(t.to_string());
                    }
                }
            }
        }
        let prevouts = fetch_prevouts(self, &prev_txids).await;
        let mut result = Vec::new();
        for tx in &txs {
            result.extend(expand_bitcoin_tx(tx, height, &prevouts));
        }
        Ok(result)
    }
}

/// The shape of a `getblock` `tx` array.
enum BlockTxShape {
    /// Full tx objects (verbosity 2/3).
    Objects,
    /// Plain txid strings (verbosity 1) — some gateways ignore the verbosity.
    Txids,
    /// Empty or mixed — no usable tx data.
    EmptyOrMalformed,
}

fn classify_block_tx_shape(txs: &[Value]) -> BlockTxShape {
    if txs.is_empty() {
        return BlockTxShape::EmptyOrMalformed;
    }
    if txs.iter().all(|t| t.is_object()) {
        return BlockTxShape::Objects;
    }
    if txs.iter().all(|t| t.is_string()) {
        return BlockTxShape::Txids;
    }
    BlockTxShape::EmptyOrMalformed
}

/// Fetch a raw transaction's JSON (verbosity 1) on a best-effort basis.
///
/// Deliberately does **not** mark the endpoint as failed on a JSON-RPC error:
/// a `-5` ("No such mempool or blockchain transaction") simply means the
/// caller degrades (drops a spend, skips a reconstruction) rather than
/// failing the whole block.
async fn getrawtransaction_value(chain: &BitcoinChain, txid: &str) -> Option<Value> {
    let url = chain.pool.next_url().ok()?.to_string();
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getrawtransaction",
        "params": [txid, 1]
    });
    let resp = chain
        .pool
        .client()
        .post(&url)
        .json(&body)
        .send()
        .await
        .ok()?;
    let v: Value = resp.json().await.ok()?;
    match v.get("error") {
        Some(e) if !e.is_null() => {
            tracing::debug!(
                txid,
                error = %e,
                "getrawtransaction lookup failed, row data stays partial"
            );
            None
        }
        _ => {
            chain.pool.mark_success(&url);
            v.get("result").cloned()
        }
    }
}

/// Rebuild full tx objects from a verbosity-1 `tx` (txid list) block, so an
/// endpoint that ignores the verbosity flag still yields complete rows.
/// Lookups that fail are skipped; if nothing can be reconstructed the block
/// fetch reports `Ok(vec![])` and the shape check above is the safety net
/// that prevents a partially-served block from being counted as complete.
async fn reconstruct_block_txs(chain: &BitcoinChain, txids: &[Value]) -> Vec<Value> {
    let mut txs = Vec::with_capacity(txids.len());
    for chunk in txids.chunks(PREVOUT_FETCH_CONCURRENCY) {
        let ids: Vec<String> = chunk
            .iter()
            .filter_map(|t| t.as_str().map(|s| s.to_string()))
            .collect();
        let batch: Vec<_> = ids
            .iter()
            .map(|txid| getrawtransaction_value(chain, txid))
            .collect();
        for tx in futures::future::join_all(batch).await.into_iter().flatten() {
            if !tx.is_null() {
                txs.push(tx);
            }
        }
    }
    txs
}

/// A resolved previous output (`vin[].prevout`), fetched out-of-band because
/// dogecoin-core verbosity-3 blocks do not embed it.
#[derive(Debug, Clone, Default)]
pub(crate) struct PrevOut {
    pub address: Option<String>,
    pub sats: u64,
}

/// Best-effort lookup of a previous transaction's outputs.
///
/// Reuses [`getrawtransaction_value`]; a failed lookup simply means the row
/// keeps `from`/`gas_fee` as `None`, matching the pre-enrichment behaviour.
async fn rpc_prevout(chain: &BitcoinChain, txid: &str) -> Option<Value> {
    getrawtransaction_value(chain, txid).await
}

/// Fetch the previous outputs referenced by a block's inputs, deduped by
/// `txid` and capped in concurrency. Lookups that fail are skipped — callers
/// degrade to `from`/`gas_fee` of `None` rather than failing the whole block.
async fn fetch_prevouts(
    chain: &BitcoinChain,
    txids: &HashSet<String>,
) -> HashMap<(String, u64), PrevOut> {
    let txid_vec: Vec<&String> = txids.iter().collect();
    let mut results = Vec::new();
    for chunk in txid_vec.chunks(PREVOUT_FETCH_CONCURRENCY) {
        let mut batch = Vec::with_capacity(chunk.len());
        for txid in chunk {
            batch.push(rpc_prevout(chain, txid));
        }
        results.extend(futures::future::join_all(batch).await);
    }
    let mut map = HashMap::new();
    for (txid, tx) in txid_vec.iter().zip(results) {
        let Some(tx) = tx else { continue };
        let Some(vout) = tx.get("vout").and_then(|v| v.as_array()) else {
            continue;
        };
        for o in vout {
            let n = o.get("n").and_then(|v| v.as_u64()).unwrap_or(0);
            let addr = o.get("scriptPubKey").and_then(script_address);
            map.insert(((*txid).clone(), n), PrevOut {
                address: addr,
                sats: btc_to_sats(o.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0)),
            });
        }
    }
    map
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

/// Expand one getblock verbosity-3 tx into spend + payment rows.
///
/// A many-to-many tx cannot be collapsed into a single `from`/`to` pair, so
/// each tx is decomposed into two record sets:
///
/// - **Spend records** — one per resolved input: `from` = that input's
///   address, `to` = `None`, `value` = the input's value, `log_index` =
///   negative (`-(vin_pos + 2)`, keeping input records disjoint from output
///   records under the `(chain_index, hash, contract_address, log_index)`
///   unique key). Unresolved inputs (no embedded prevout and no out-of-band
///   lookup) are skipped.
/// - **Payment records** — one per non-change output: `from` = the tx's input
///   address, `to` = the output address, `value` = the output's value,
///   `log_index` = `vout.n`.
///
/// Coinbase txs yield no records. `gas_fee` is the tx-level fee (Σin − Σout)
/// shared by every record when all inputs were resolved.
pub(crate) fn expand_bitcoin_tx(
    tx: &Value,
    height: u64,
    prevouts: &HashMap<(String, u64), PrevOut>,
) -> Vec<NormalizedTx> {
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

    // Resolve each input's address + value, preferring the embedded prevout
    // (Bitcoin verbosity 3) and falling back to the fetched prevout map.
    let mut spends: Vec<(String, u64)> = Vec::new();
    let mut input_addr_set = std::collections::HashSet::new();
    let mut total_in: u64 = 0;
    let mut resolved_inputs = 0usize;
    for i in &vin {
        let resolved: Option<(Option<String>, u64)> = match i.get("prevout") {
            Some(p) => Some((
                p.get("scriptPubKey").and_then(script_address),
                btc_to_sats(p.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0)),
            )),
            None => {
                let (Some(prev_txid), Some(prev_vout)) = (
                    i.get("txid").and_then(|v| v.as_str()),
                    i.get("vout").and_then(|v| v.as_u64()),
                ) else {
                    continue;
                };
                prevouts
                    .get(&(prev_txid.to_string(), prev_vout))
                    .map(|po| (po.address.clone(), po.sats))
            }
        };
        let Some((addr, sats)) = resolved else { continue };
        resolved_inputs += 1;
        total_in += sats;
        if let Some(a) = addr {
            input_addr_set.insert(a.clone());
            spends.push((a, sats));
        }
    }
    // Fee is only computable when every input's previous output was resolved.
    let prevouts_complete = !vin.is_empty() && resolved_inputs == vin.len();

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

    let from = spends
        .iter()
        .find(|(a, _)| !output_addr_set.contains(a))
        .map(|(a, _)| a.clone())
        .or_else(|| spends.first().map(|(a, _)| a.clone()));

    let gas_fee = if prevouts_complete && total_in > total_out {
        Some(Amount::new(Decimal::from(total_in - total_out), 8))
    } else {
        None
    };

    let mut rows = Vec::new();
    // Spend records: one per resolved input, `from`-only.
    for (pos, (addr, sats)) in spends.iter().enumerate() {
        rows.push(NormalizedTx {
            hash: TxHash::new(txid),
            from: Some(Address::new(addr.clone())),
            to: None,
            value: Amount::new(Decimal::from(*sats), 8),
            gas_fee: gas_fee.clone(),
            block_number: height,
            status: TxStatus::Success,
            raw: json!({ "txid": txid, "vin": pos }),
            contract_address: None,
            // Negative and offset by 2: `-(pos+2)` keeps input records out of
            // the `vout.n` space and out of the NULL (→ -1) conflict slot.
            log_index: Some(-((pos as i64) + 2)),
            method: Some("native_spend".into()),
        });
    }
    // Payment records: one per non-change output, `to`-side.
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
            log_index: Some(n as i64),
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
            ..GasEstimate::default()
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

    fn empty_prevouts() -> HashMap<(String, u64), PrevOut> {
        HashMap::new()
    }

    #[test]
    fn skips_change_keeps_payment() {
        let rows = expand_bitcoin_tx(&payment_tx(), 100, &empty_prevouts());
        assert_eq!(rows.len(), 2);
        // Spend record first: from-only, negative log_index.
        assert_eq!(rows[0].from.as_ref().unwrap().as_str(), "bc1from");
        assert!(rows[0].to.is_none());
        assert_eq!(rows[0].log_index, Some(-2));
        assert_eq!(rows[0].method.as_deref(), Some("native_spend"));
        assert_eq!(rows[0].value.raw, Decimal::from(100_000_000u64));
        assert!(rows[0].gas_fee.is_some());
        // Payment record: from + to, change output skipped.
        assert_eq!(rows[1].to.as_ref().unwrap().as_str(), "bc1to");
        assert_eq!(rows[1].from.as_ref().unwrap().as_str(), "bc1from");
        assert_eq!(rows[1].log_index, Some(0));
        assert_eq!(rows[1].method.as_deref(), Some("native_transfer"));
        assert!(rows[1].gas_fee.is_some());
    }

    #[test]
    fn skips_coinbase() {
        let tx = json!({
            "txid": "coin",
            "vin": [{ "coinbase": "04..." }],
            "vout": [{ "n": 0, "value": 6.25, "scriptPubKey": { "address": "bc1miner" } }]
        });
        assert!(expand_bitcoin_tx(&tx, 1, &empty_prevouts()).is_empty());
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
        let rows = expand_bitcoin_tx(&tx, 2, &empty_prevouts());
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].method.as_deref(), Some("native_spend"));
        assert_eq!(rows[0].log_index, Some(-2));
        assert_eq!(rows[1].log_index, Some(1));
        assert_eq!(rows[1].to.as_ref().unwrap().as_str(), "bc1b");
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
        // No embedded prevout and no out-of-band map → spend is dropped, the
        // two outputs still split and keep `from`/`gas_fee` as None.
        let rows = expand_bitcoin_tx(&tx, 3, &empty_prevouts());
        assert_eq!(rows.len(), 2);
        assert!(rows[0].from.is_none());
        assert!(rows[0].gas_fee.is_none());
        assert_eq!(rows[0].log_index, Some(0));
        assert_eq!(rows[1].log_index, Some(1));
        assert!(rows[1].gas_fee.is_none());
    }

    #[test]
    fn doge_style_prevout_map_yields_spend() {
        let tx = json!({
            "txid": "d",
            "vin": [{ "txid": "prev", "vout": 1 }],
            "vout": [
                { "n": 0, "value": 0.6, "scriptPubKey": { "address": "Db1to" } },
                { "n": 1, "value": 0.3999, "scriptPubKey": { "address": "Db1from" } }
            ]
        });
        // dogecoin-core omits vin[].prevout; the map fills the gap.
        let mut prevouts = HashMap::new();
        prevouts.insert(
            ("prev".to_string(), 1),
            PrevOut { address: Some("Db1from".into()), sats: 100_000_000 },
        );
        let rows = expand_bitcoin_tx(&tx, 6, &prevouts);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].method.as_deref(), Some("native_spend"));
        assert_eq!(rows[0].from.as_ref().unwrap().as_str(), "Db1from");
        assert_eq!(rows[0].log_index, Some(-2));
        assert_eq!(rows[0].value.raw, Decimal::from(100_000_000u64));
        // Change output (pays back to the input address) is skipped.
        assert_eq!(rows[1].to.as_ref().unwrap().as_str(), "Db1to");
        assert_eq!(rows[1].from.as_ref().unwrap().as_str(), "Db1from");
        assert_eq!(rows[1].gas_fee.as_ref().unwrap().raw, Decimal::from(10_000u64));
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
        let rows = expand_bitcoin_tx(&tx, 4, &empty_prevouts());
        assert_eq!(rows.len(), 3);
        let fee0 = rows[0].gas_fee.as_ref().expect("fee on first");
        let fee1 = rows[1].gas_fee.as_ref().expect("fee on second");
        let fee2 = rows[2].gas_fee.as_ref().expect("fee on third");
        assert_eq!(fee0.raw, fee1.raw);
        assert_eq!(fee0.raw, fee2.raw);
        assert_eq!(fee0.raw, Decimal::from(100_000u64)); // 1.0 - 0.999 BTC
    }

    #[test]
    fn tx_shape_classification() {
        use BlockTxShape::*;
        assert!(matches!(classify_block_tx_shape(&[]), EmptyOrMalformed));
        assert!(matches!(
            classify_block_tx_shape(&[json!("aa"), json!("bb")]),
            Txids
        ));
        assert!(matches!(
            classify_block_tx_shape(&[json!({ "txid": "aa" })]),
            Objects
        ));
        assert!(matches!(
            classify_block_tx_shape(&[json!("aa"), json!({ "txid": "bb" })]),
            EmptyOrMalformed
        ));
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
        let rows = expand_bitcoin_tx(&tx, 5, &empty_prevouts());
        assert_eq!(rows.len(), 3);
        // Two spend records (negative log_index), then the payment record.
        assert_eq!(rows[0].log_index, Some(-2));
        assert_eq!(rows[0].from.as_ref().unwrap().as_str(), "bc1change");
        assert_eq!(rows[1].log_index, Some(-3));
        assert_eq!(rows[1].from.as_ref().unwrap().as_str(), "bc1real");
        assert_eq!(rows[2].from.as_ref().unwrap().as_str(), "bc1real");
        assert_eq!(rows[2].to.as_ref().unwrap().as_str(), "bc1pay");
    }
}
