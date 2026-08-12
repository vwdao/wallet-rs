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

/// How many `getrawtransaction` prevout lookups run in parallel while
/// enriching one block. Zcash emits a block every ~75s with few transparent
/// txs, so a small cap keeps third-party RPCs comfortable.
const PREVOUT_FETCH_CONCURRENCY: usize = 8;

/// Zcash (`zcashd`/`zebra`) JSON-RPC chain.
///
/// The RPC surface is Bitcoin-compatible, but there are two important
/// differences that force a dedicated implementation rather than reusing
/// `BitcoinChain`:
///
/// - `getblock` supports verbosity 0..=2 only — there is no verbosity-3
///   `vin[].prevout`. Input addresses and fees are instead resolved
///   best-effort from the previous transactions via `getrawtransaction`
///   (deduped + concurrent). When the node cannot serve a previous tx (e.g.
///   no `-txindex`, mempool-only lookup), `from`/`gas_fee` are left `None`
///   for the affected rows.
/// - Blocks carry shielded transactions (`vShieldedSpend`, `vShieldedOutput`,
///   `vJoinSplit`, `vOrchardActions`) whose recipients are unlinkable.
///   Transparent (t-addr) transfers are indexed per addressed output; a
///   transparent→shielded transfer is surfaced as a `from`-only row
///   (`value` = net shielded inflow), and a shielded→transparent transfer as
///   a `to`-only row.
#[derive(Debug)]
pub struct ZcashChain {
    pub chain_index: ChainIndex,
    pub pool: RpcPool,
}

impl ZcashChain {
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
impl BalanceReader for ZcashChain {
    async fn native_balance(&self, addr: &Address) -> AppResult<Amount> {
        // Transparent (t-addr) balance. `getreceivedbyaddress` only counts
        // transparent received value; shielded (z-addr) balances are not
        // exposed by the RPC without a wallet.
        let v = self
            .rpc("getreceivedbyaddress", json!([addr.as_str(), 0]))
            .await?;
        let zec = v.as_f64().unwrap_or(0.0);
        let zats = (zec * 1e8) as u64;
        Ok(Amount::new(Decimal::from(zats), 8))
    }
}

#[async_trait]
impl TokenBalance for ZcashChain {
    async fn token_balance(&self, _wallet: &Address, _token: &Address) -> AppResult<Amount> {
        // No tokens on Zcash transparent mainnet.
        Ok(Amount::zero(8))
    }
}

#[async_trait]
impl TxBroadcaster for ZcashChain {
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
impl BlockSource for ZcashChain {
    async fn tip(&self) -> AppResult<u64> {
        let result = self.rpc("getblockcount", json!([])).await?;
        result.as_u64().ok_or_else(|| {
            AppError::Unavailable(
                "getblockcount returned no height (endpoint rate-limited or down)".into(),
            )
        })
    }

    async fn fetch_block_txs(&self, height: u64) -> AppResult<Vec<NormalizedTx>> {
        let hash: String = self
            .rpc("getblockhash", json!([height]))
            .await?
            .as_str()
            .unwrap_or("")
            .to_string();
        // zcashd supports verbosity 0..=2 only. Verbosity 2 returns full tx
        // objects with `vout[].scriptPubKey` addresses but no `vin[].prevout`;
        // input addresses/values are resolved via `getrawtransaction` below.
        let block = self.rpc("getblock", json!([hash, 2])).await?;
        let txs = block
            .get("tx")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default();
        let mut prev_txids = HashSet::new();
        for tx in &txs {
            if tx
                .get("vin")
                .and_then(|v| v.as_array())
                .and_then(|a| a.first())
                .and_then(|v| v.get("coinbase"))
                .is_some()
            {
                continue;
            }
            if let Some(vin) = tx.get("vin").and_then(|v| v.as_array()) {
                for i in vin {
                    if let Some(t) = i.get("txid").and_then(|v| v.as_str()) {
                        prev_txids.insert(t.to_string());
                    }
                }
            }
        }
        let prevouts = fetch_prevouts(self, &prev_txids).await;
        let mut result = Vec::new();
        for tx in &txs {
            result.extend(expand_zcash_tx(tx, height, &prevouts));
        }
        Ok(result)
    }
}

/// A resolved previous output (`vin[].prevout`), fetched out-of-band because
/// zcashd verbosity-2 blocks do not embed it.
#[derive(Debug, Clone, Default)]
pub(crate) struct PrevOut {
    pub address: Option<String>,
    pub zats: u64,
}

/// Best-effort lookup of a previous transaction's outputs.
///
/// Deliberately does **not** mark the endpoint as failed on a JSON-RPC error:
/// a `-5` ("No such mempool or blockchain transaction") is expected on nodes
/// without `-txindex` and simply means the row keeps `from`/`gas_fee` as
/// `None`, matching the pre-enrichment behaviour.
async fn rpc_prevout(chain: &ZcashChain, txid: &str) -> Option<Value> {
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
                "getrawtransaction prevout lookup failed, from/fee stay unknown"
            );
            None
        }
        _ => {
            chain.pool.mark_success(&url);
            v.get("result").cloned()
        }
    }
}

/// Fetch the previous outputs referenced by a block's inputs, deduped by
/// `txid` and capped in concurrency. Lookups that fail are skipped — callers
/// degrade to `from`/`gas_fee` of `None` rather than failing the whole block.
async fn fetch_prevouts(
    chain: &ZcashChain,
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
            map.insert(((*txid).clone(), n), PrevOut { address: addr, zats: vout_zats(o) });
        }
    }
    map
}

/// Zats for one transparent output. Newer zcashd emits the exact `valueZat`
/// integer; older nodes only have the float `value`.
fn vout_zats(vout: &Value) -> u64 {
    if let Some(zat) = vout.get("valueZat").and_then(|v| v.as_u64()) {
        return zat;
    }
    vout.get("value")
        .and_then(|v| v.as_f64())
        .map(|v| (v * 1e8).round() as u64)
        .unwrap_or(0)
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

/// Net value balance of one shielded pool, in zats. Positive = value flowing
/// **into** the shielded pool from the transparent pool.
fn balance_zat(v: &Value) -> i64 {
    if let Some(zat) = v.get("valueBalanceZat").and_then(|x| x.as_i64()) {
        return zat;
    }
    v.get("valueBalance")
        .and_then(|x| x.as_f64())
        .map(|f| (f * 1e8).round() as i64)
        .unwrap_or(0)
}

/// Net value entering the shielded pools (Sapling + Orchard) from transparent,
/// in zats. Zero or negative means no transparent→shielded transfer.
fn shielded_inflow_zat(tx: &Value) -> u64 {
    let mut total = balance_zat(tx).max(0);
    if let Some(orchard) = tx.get("orchard") {
        total += balance_zat(orchard).max(0);
    }
    total.max(0) as u64
}

/// Expand one `getblock` verbosity-2 tx into transfer records.
///
/// The `prevouts` map is built from `getrawtransaction` lookups because
/// verbosity-2 blocks omit `vin[].prevout`. A many-to-many tx cannot be
/// collapsed into a single `from`/`to` pair (per-input transfer values never
/// line up with per-output values), so each tx is decomposed into two record
/// sets:
///
/// - **Spend records** — one per resolved input: `from` = that input's
///   address, `to` = `None`, `value` = the input's value, `log_index` =
///   negative (`-(vin_pos + 2)`, keeping input records disjoint from output
///   records under the `(chain_index, hash, contract_address, log_index)`
///   unique key). Unresolved inputs (prevout lookup failed) are skipped.
/// - **Receive records** — one per addressed output: `from` = `None`, `to` =
///   the output address, `value` = the output's value, `log_index` = `vout.n`.
///
/// A transparent→shielded transfer that leaves no addressable output also
/// yields a `from`-only record (`method = "shielded_transfer"`, `value` = net
/// shielded inflow). Coinbase and fully shielded (z→z) transactions yield no
/// records. `gas_fee` is the tx-level fee (Σin − Σout − shielded inflow)
/// shared by every record when all inputs were resolved.
pub(crate) fn expand_zcash_tx(
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

    // Resolve each input's previous output (address + value).
    let mut spends: Vec<(String, u64)> = Vec::new();
    let mut total_in: u64 = 0;
    let mut resolved_inputs = 0usize;
    for i in &vin {
        let (Some(prev_txid), Some(prev_vout)) = (
            i.get("txid").and_then(|v| v.as_str()),
            i.get("vout").and_then(|v| v.as_u64()),
        ) else {
            continue;
        };
        if let Some(po) = prevouts.get(&(prev_txid.to_string(), prev_vout)) {
            resolved_inputs += 1;
            total_in += po.zats;
            if let Some(a) = &po.address {
                spends.push((a.clone(), po.zats));
            }
        }
    }
    // Fee is only computable when every input's previous output was resolved.
    let prevouts_complete = !vin.is_empty() && resolved_inputs == vin.len();

    // Collect addressed outputs (n, address, zats).
    let mut receives: Vec<(u64, String, u64)> = Vec::new();
    let mut total_out: u64 = 0;
    for o in &vout {
        let n = o.get("n").and_then(|v| v.as_u64()).unwrap_or(0);
        let zats = vout_zats(o);
        total_out += zats;
        if let Some(a) = o.get("scriptPubKey").and_then(script_address) {
            receives.push((n, a, zats));
        }
    }

    let shielded_inflow = shielded_inflow_zat(tx);
    // Fee = transparent in − transparent out − value entering shielded pools.
    let gas_fee = if prevouts_complete && total_in >= total_out.saturating_add(shielded_inflow) {
        Some(Amount::new(Decimal::from(total_in - total_out - shielded_inflow), 8))
    } else {
        None
    };

    let mut rows = Vec::new();
    // Spend records: one per resolved input, `from`-only.
    for (pos, (addr, zats)) in spends.iter().enumerate() {
        rows.push(NormalizedTx {
            hash: TxHash::new(txid),
            from: Some(Address::new(addr.clone())),
            to: None,
            value: Amount::new(Decimal::from(*zats), 8),
            gas_fee: gas_fee.clone(),
            block_number: height,
            status: TxStatus::Success,
            raw: json!({ "txid": txid, "vin": pos }),
            contract_address: None,
            // Negative and offset by 2: `-(pos+2)` keeps input records out of
            // the `vout.n` space and out of the reserved NULL (→ -1) shielded
            // row's conflict slot.
            log_index: Some(-((pos as i64) + 2)),
            method: Some("native_spend".into()),
        });
    }
    // Receive records: one per addressed output, `to`-only.
    for (n, to_addr, zats) in &receives {
        rows.push(NormalizedTx {
            hash: TxHash::new(txid),
            from: None,
            to: Some(Address::new(to_addr.clone())),
            value: Amount::new(Decimal::from(*zats), 8),
            gas_fee: gas_fee.clone(),
            block_number: height,
            status: TxStatus::Success,
            raw: json!({ "txid": txid, "vout": n }),
            contract_address: None,
            log_index: Some(*n as i64),
            method: Some("native_transfer".into()),
        });
    }
    // Transparent → shielded with no addressable output: a `from`-only record
    // so the t-address's outgoing shielded transfer stays visible.
    if shielded_inflow > 0 && !spends.is_empty() {
        rows.push(NormalizedTx {
            hash: TxHash::new(txid),
            from: Some(Address::new(spends[0].0.clone())),
            to: None,
            value: Amount::new(Decimal::from(shielded_inflow), 8),
            gas_fee,
            block_number: height,
            status: TxStatus::Success,
            raw: json!({ "txid": txid, "shielded": true }),
            contract_address: None,
            log_index: None,
            method: Some("shielded_transfer".into()),
        });
    }
    rows
}

#[async_trait]
impl GasEstimator for ZcashChain {
    async fn estimate_gas(&self, _tx: &GasEstimateRequest) -> AppResult<GasEstimate> {
        // zcashd ≥5.0 exposes `estimatesmartfee` ({feerate}); older nodes only
        // have the legacy `estimatefee` (a bare ZEC/kB number).
        let v = match self.rpc("estimatesmartfee", json!([6])).await {
            Ok(v) => Ok(v),
            Err(_) => self.rpc("estimatefee", json!([6])).await,
        };
        let feerate = match &v {
            Ok(v) => v
                .get("feerate")
                .and_then(|f| f.as_f64())
                .or_else(|| v.as_f64())
                .unwrap_or(0.0001),
            Err(_) => 0.0001,
        };
        // ZEC/kB → zat/vB (1e8 zats per ZEC, 1000 bytes per kB).
        let zat_per_vb = (feerate * 1e8 / 1000.0) as u128;
        Ok(GasEstimate {
            gas_limit: 250,
            max_fee_per_gas: Some(zat_per_vb),
            max_priority_fee_per_gas: None,
            ..GasEstimate::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn prev(txid: &str, vout: u64, addr: &str, zats: u64) -> HashMap<(String, u64), PrevOut> {
        let mut m = HashMap::new();
        m.insert(
            (txid.to_string(), vout),
            PrevOut {
                address: Some(addr.to_string()),
                zats,
            },
        );
        m
    }

    fn payment_tx() -> Value {
        json!({
            "txid": "abc",
            "vin": [{ "txid": "prev", "vout": 0 }],
            "vout": [
                { "n": 0, "value": 0.4, "valueZat": 40000000, "scriptPubKey": { "address": "t1to" } },
                { "n": 1, "value": 0.5999, "valueZat": 59990000, "scriptPubKey": { "address": "t1change" } }
            ]
        })
    }

    #[test]
    fn splits_many_to_many_into_spends_and_receives() {
        // 2 inputs (from different t-addresses) → 2 outputs (payment + change
        // back to A). Cannot be paired 1:1, so decompose per input/output.
        let tx = json!({
            "txid": "abc",
            "vin": [
                { "txid": "prev1", "vout": 0 },
                { "txid": "prev2", "vout": 0 }
            ],
            "vout": [
                { "n": 0, "value": 0.4, "valueZat": 40000000, "scriptPubKey": { "address": "t1to" } },
                { "n": 1, "value": 0.6999, "valueZat": 69990000, "scriptPubKey": { "address": "t1fromA" } }
            ]
        });
        let mut prevouts = prev("prev1", 0, "t1fromA", 60_000_000);
        prevouts.insert(
            ("prev2".to_string(), 0),
            PrevOut { address: Some("t1fromB".into()), zats: 50_000_000 },
        );
        let rows = expand_zcash_tx(&tx, 100, &prevouts);
        assert_eq!(rows.len(), 4);
        // Spend records: from-only, negative log_index, per-input value.
        assert_eq!(rows[0].from.as_ref().unwrap().as_str(), "t1fromA");
        assert!(rows[0].to.is_none());
        assert_eq!(rows[0].value.raw, Decimal::from(60_000_000u64));
        assert_eq!(rows[0].log_index, Some(-2));
        assert_eq!(rows[0].method.as_deref(), Some("native_spend"));
        assert_eq!(rows[1].from.as_ref().unwrap().as_str(), "t1fromB");
        assert_eq!(rows[1].value.raw, Decimal::from(50_000_000u64));
        assert_eq!(rows[1].log_index, Some(-3));
        // Receive records: to-only, log_index = vout.n.
        assert!(rows[2].from.is_none());
        assert_eq!(rows[2].to.as_ref().unwrap().as_str(), "t1to");
        assert_eq!(rows[2].value.raw, Decimal::from(40_000_000u64));
        assert_eq!(rows[2].log_index, Some(0));
        assert_eq!(rows[3].to.as_ref().unwrap().as_str(), "t1fromA");
        assert_eq!(rows[3].value.raw, Decimal::from(69_990_000u64));
        assert_eq!(rows[3].log_index, Some(1));
        // Fee shared by all records: 110000000 − (40000000 + 69990000) = 10000.
        for row in &rows {
            assert_eq!(row.gas_fee.as_ref().unwrap().raw, Decimal::from(10_000u64));
            assert_eq!(row.block_number, 100);
            assert_eq!(row.status, TxStatus::Success);
        }
    }

    #[test]
    fn single_input_output_simple_payment() {
        let prevouts = prev("prev", 0, "t1from", 100_000_000);
        let rows = expand_zcash_tx(&payment_tx(), 100, &prevouts);
        assert_eq!(rows.len(), 3);
        // spend (input) + 2 receives (both outputs kept: no change detection).
        assert_eq!(rows[0].from.as_ref().unwrap().as_str(), "t1from");
        assert_eq!(rows[0].value.raw, Decimal::from(100_000_000u64));
        assert_eq!(rows[0].log_index, Some(-2));
        assert_eq!(rows[1].to.as_ref().unwrap().as_str(), "t1to");
        assert_eq!(rows[1].value.raw, Decimal::from(40_000_000u64));
        assert_eq!(rows[1].log_index, Some(0));
        assert_eq!(rows[1].from, None);
        assert_eq!(rows[2].to.as_ref().unwrap().as_str(), "t1change");
        assert_eq!(rows[2].value.raw, Decimal::from(59_990_000u64));
        assert_eq!(rows[2].log_index, Some(1));
        assert_eq!(rows[2].from, None);
        // fee = 1.0 − (0.4 + 0.5999) = 10000 zats on every record.
        for row in &rows {
            assert_eq!(row.gas_fee.as_ref().unwrap().raw, Decimal::from(10_000u64));
            assert_eq!(row.value.decimals, 8);
        }
    }

    #[test]
    fn keeps_receives_when_prevout_unresolved() {
        // No resolvable prevout → spend records are dropped, receives stay,
        // fee unknown (pre-enrichment degradation).
        let rows = expand_zcash_tx(&payment_tx(), 100, &HashMap::new());
        assert_eq!(rows.len(), 2);
        for row in &rows {
            assert!(row.from.is_none());
            assert!(row.gas_fee.is_none());
        }
        assert_eq!(rows[0].to.as_ref().unwrap().as_str(), "t1to");
        assert_eq!(rows[0].log_index, Some(0));
        assert_eq!(rows[1].to.as_ref().unwrap().as_str(), "t1change");
        assert_eq!(rows[1].log_index, Some(1));
    }

    #[test]
    fn skips_coinbase() {
        let tx = json!({
            "txid": "coin",
            "vin": [{ "coinbase": "04..." }],
            "vout": [{ "n": 0, "value": 6.25, "valueZat": 625000000, "scriptPubKey": { "address": "t1miner" } }]
        });
        assert!(expand_zcash_tx(&tx, 1, &HashMap::new()).is_empty());
    }

    #[test]
    fn shielded_to_transparent_yields_to_only_receive() {
        // Shielded spender, transparent output: no vin to resolve, so only the
        // receive record is indexed.
        let tx = json!({
            "txid": "shielded-in",
            "vin": [],
            "vout": [{ "n": 0, "value": 1.0, "valueZat": 100000000, "scriptPubKey": { "address": "t1recv" } }],
            "vShieldedSpend": [{ "nullifier": "0x.." }],
            "valueBalance": -1.0,
            "valueBalanceZat": -100000000
        });
        let rows = expand_zcash_tx(&tx, 2, &HashMap::new());
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].to.as_ref().unwrap().as_str(), "t1recv");
        assert!(rows[0].from.is_none());
        assert!(rows[0].gas_fee.is_none());
    }

    #[test]
    fn transparent_to_shielded_yields_spend_and_shielded_record() {
        // t-address funds the shielded pool: a spend record (full input value)
        // plus a shielded-transfer record (net inflow, fee excluded).
        let tx = json!({
            "txid": "shielded-out",
            "vin": [{ "txid": "prev", "vout": 0 }],
            "vout": [],
            "vShieldedOutput": [{ "cmu": "0x.." }],
            "valueBalance": 0.4,
            "valueBalanceZat": 40000000
        });
        let prevouts = prev("prev", 0, "t1from", 100_000_000);
        let rows = expand_zcash_tx(&tx, 3, &prevouts);
        assert_eq!(rows.len(), 2);
        // Spend record: full input debited from the t-address.
        assert_eq!(rows[0].from.as_ref().unwrap().as_str(), "t1from");
        assert!(rows[0].to.is_none());
        assert_eq!(rows[0].value.raw, Decimal::from(100_000_000u64));
        assert_eq!(rows[0].log_index, Some(-2));
        // Shielded transfer record: what actually entered the shielded pool.
        assert_eq!(rows[1].from.as_ref().unwrap().as_str(), "t1from");
        assert!(rows[1].to.is_none());
        assert_eq!(rows[1].value.raw, Decimal::from(40_000_000u64));
        assert_eq!(rows[1].log_index, None);
        assert_eq!(rows[1].method.as_deref(), Some("shielded_transfer"));
        // fee = 1.0 − 0.4 ZEC on every record.
        for row in &rows {
            assert_eq!(row.gas_fee.as_ref().unwrap().raw, Decimal::from(60_000_000u64));
        }
    }

    #[test]
    fn skips_fully_shielded_without_resolvable_input() {
        // z→z: no vin and no addressable output → nothing to index, even with a
        // positive valueBalance.
        let tx = json!({
            "txid": "shielded",
            "vin": [],
            "vout": [],
            "vShieldedSpend": [{ "nullifier": "0x.." }],
            "vShieldedOutput": [{ "cmu": "0x.." }],
            "valueBalance": 500000
        });
        assert!(expand_zcash_tx(&tx, 2, &HashMap::new()).is_empty());
    }

    #[test]
    fn skips_op_return() {
        let tx = json!({
            "txid": "x",
            "vin": [{ "txid": "p", "vout": 0 }],
            "vout": [
                { "n": 0, "value": 0.0, "valueZat": 0, "scriptPubKey": { "type": "nulldata" } },
                { "n": 1, "value": 0.09, "valueZat": 9000000, "scriptPubKey": { "address": "t1b" } }
            ]
        });
        let rows = expand_zcash_tx(&tx, 2, &HashMap::new());
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].log_index, Some(1));
        assert_eq!(rows[0].to.as_ref().unwrap().as_str(), "t1b");
    }

    #[test]
    fn falls_back_to_float_value_when_no_valuezat() {
        let tx = json!({
            "txid": "y",
            "vin": [{ "txid": "p", "vout": 0 }],
            "vout": [{ "n": 0, "value": 1.5, "scriptPubKey": { "address": "t1c" } }]
        });
        let rows = expand_zcash_tx(&tx, 3, &HashMap::new());
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].value.raw, Decimal::from(150_000_000u64));
        assert_eq!(rows[0].log_index, Some(0));
    }

    #[test]
    fn skips_vout_without_address() {
        let tx = json!({
            "txid": "z",
            "vin": [{ "txid": "p", "vout": 0 }],
            "vout": [{ "n": 0, "value": 1.0, "valueZat": 100000000, "scriptPubKey": { "type": "scripthash" } }]
        });
        assert!(expand_zcash_tx(&tx, 4, &HashMap::new()).is_empty());
    }
}
