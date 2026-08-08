# BTC Multi-Output Transfers Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Expand each Bitcoin chain tx into one `NormalizedTx` per non-change payment output, with correct from/to/value and `log_index = vout.n`.

**Architecture:** Pure function `expand_bitcoin_tx(tx, height) -> Vec<NormalizedTx>` parses getblock verbosity-3 JSON; `fetch_block_txs` calls `getblock(hash, 3)` and flat-maps the expander. Unit tests use JSON fixtures only (no RPC).

**Tech Stack:** Rust, `serde_json`, `wallet-types::{NormalizedTx, Address, Amount}`, existing unique key on `(chain, hash, contract, log_index)`.

## Global Constraints

- Spec: `docs/superpowers/specs/2026-08-08-btc-multi-output-transfers-design.md`
- Skip coinbase; skip OP_RETURN / addressless outputs; skip change (`to == from`)
- `from` = first input address not in output set; else first input address
- `method` = `"native_transfer"`; fee only on first payment row
- No DB migration; no auto-delete of dirty rows; no commits unless user asks
- Config HTTP port already fixed separately (`8545`)

---

## File Structure

| File | Role |
|------|------|
| `crates/wallet-chain/src/bitcoin/mod.rs` | Helpers + `expand_bitcoin_tx` + wire `fetch_block_txs` + `#[cfg(test)]` |

### Task 1: `expand_bitcoin_tx` + tests + wire fetch

**Files:**
- Modify: `crates/wallet-chain/src/bitcoin/mod.rs`

**Interfaces:**
- Produces: `fn expand_bitcoin_tx(tx: &Value, height: u64) -> Vec<NormalizedTx>`
- Produces: `fn script_address(script: &Value) -> Option<String>`
- Produces: `fn btc_to_sats(v: f64) -> u64`

- [x] **Step 1: Write failing tests** at bottom of `bitcoin/mod.rs`

```rust
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
    fn multi_payment_fee_on_first_only() {
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
        assert!(rows[0].gas_fee.is_some());
        assert!(rows[1].gas_fee.is_none());
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
```

- [x] **Step 2: Run tests — expect FAIL** (function missing)

```bash
cargo test -p wallet-chain --features bitcoin expand_bitcoin -- --nocapture
```

Expected: compile error `cannot find function expand_bitcoin_tx`

- [x] **Step 3: Implement helpers + `expand_bitcoin_tx` + wire `fetch_block_txs`**

```rust
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

/// Expand one getblock verbosity-3 tx into payment rows (see design spec).
pub(crate) fn expand_bitcoin_tx(tx: &Value, height: u64) -> Vec<NormalizedTx> {
    let vin = tx.get("vin").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    if vin.first().and_then(|v| v.get("coinbase")).is_some() {
        return Vec::new();
    }
    let txid = tx.get("txid").and_then(|v| v.as_str()).unwrap_or("");
    let vout = tx.get("vout").and_then(|v| v.as_array()).cloned().unwrap_or_default();

    let mut input_addrs: Vec<String> = Vec::new();
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
        let addr = o
            .get("scriptPubKey")
            .and_then(script_address);
        if let Some(a) = addr {
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
        if from.as_deref() == Some(to_addr.as_str()) {
            continue; // change
        }
        let fee = if rows.is_empty() { gas_fee.clone() } else { None };
        rows.push(NormalizedTx {
            hash: TxHash::new(txid),
            from: from.as_ref().map(|a| Address::new(a.clone())),
            to: Some(Address::new(to_addr)),
            value: Amount::new(Decimal::from(sats), 8),
            gas_fee: fee,
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
```

In `fetch_block_txs`, replace the loop body with:

```rust
let block = self.rpc("getblock", json!([hash, 3])).await?;
// ...
let mut result = Vec::new();
for tx in &txs {
    result.extend(expand_bitcoin_tx(tx, height));
}
Ok(result)
```

- [x] **Step 4: Run tests — expect PASS**

```bash
cargo test -p wallet-chain --features bitcoin expand_bitcoin -- --nocapture
```

Expected: all `expand_bitcoin_*` / related tests PASS

- [ ] **Step 5: Commit only if user asks** — skip by default per repo rules
