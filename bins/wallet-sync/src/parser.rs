use rust_decimal::Decimal;
use wallet_types::{Address, Amount, ChainFamily, ChainIndex, NormalizedTx, TxStatus};

/// Family-specific normalization: EVM raw txs get proper from/to/value extraction.
pub fn parse_block(
    chain_index: ChainIndex,
    height: u64,
    txs: Vec<NormalizedTx>,
) -> Vec<NormalizedTx> {
    let family = ChainFamily::for_index(chain_index);
    txs.into_iter()
        .enumerate()
        .map(|(idx, tx)| normalize_tx(chain_index, family, height, idx, tx))
        .collect()
}

fn normalize_tx(
    chain_index: ChainIndex,
    family: Option<ChainFamily>,
    height: u64,
    idx: usize,
    mut tx: NormalizedTx,
) -> NormalizedTx {
    tx.block_number = height;
    match family {
        Some(ChainFamily::Evm) => normalize_evm_tx(chain_index, idx, &mut tx),
        Some(ChainFamily::Solana) => normalize_solana_tx(height, idx, &mut tx),
        Some(ChainFamily::Bitcoin) => normalize_utxo_tx(height, idx, &mut tx),
        Some(ChainFamily::Tron) => normalize_tron_tx(height, idx, &mut tx),
        Some(ChainFamily::UtxoOther) => normalize_utxo_tx(height, idx, &mut tx),
        Some(ChainFamily::Zcash) => normalize_utxo_tx(height, idx, &mut tx),
        // TON (The Open Network) and Sui use non-account models; their
        // per-block tx normalization is intentionally not implemented yet
        // (see wallet-chain/src/ton and wallet-chain/src/sui). Pass txs
        // through unchanged so callers can still surface the raw payload.
        Some(ChainFamily::Ton) | Some(ChainFamily::Sui) => {}
        None => {
            tracing::warn!(
                chain_index = chain_index.as_i64(),
                "unknown chain family, skipping normalization"
            );
        }
    }
    tx
}

/// EVM: extract proper from/to/value from raw JSON.
fn normalize_evm_tx(_chain_index: ChainIndex, _idx: usize, tx: &mut NormalizedTx) {
    let raw = &tx.raw;
    if tx.from.is_none() {
        tx.from = raw.get("from").and_then(|v| v.as_str()).map(Address::new);
    }
    if tx.to.is_none() {
        tx.to = raw.get("to").and_then(|v| v.as_str()).map(Address::new);
    }
    if let Some(val_hex) = raw.get("value").and_then(|v| v.as_str()) {
        if let Ok(raw_val) = u128::from_str_radix(val_hex.trim_start_matches("0x"), 16) {
            tx.value = Amount::new(wallet_types::decimal_from_u128(raw_val), 18);
        }
    }
    // Detect contract creation (to is null)
    if raw.get("to").and_then(|v| v.as_str()).is_none() && raw.get("input").is_some() {
        tx.to = None;
    }
}

/// Solana: enrich with slot info.
fn normalize_solana_tx(_height: u64, _idx: usize, tx: &mut NormalizedTx) {
    // Solana txs from fetch_block_txs only have signatures; keep as-is
    // The from/to will be filled in by downstream tx parsing (wallet-jobs)
    if tx.value.raw == Decimal::ZERO {
        // Check if raw has token transfer info
        if let Some(amt) = tx.raw.get("lamports").and_then(|v| v.as_u64()) {
            tx.value = Amount::new(Decimal::from(amt), 9);
        }
    }
}

/// UTXO-based chains (BTC/DOGE/ZCASH): derive status from block height.
fn normalize_utxo_tx(_height: u64, _idx: usize, tx: &mut NormalizedTx) {
    if tx.status == TxStatus::Pending && _height > 0 {
        tx.status = TxStatus::Success;
    }
}

/// TRON: status from `ret`, and ensure address fields are base58 (`T…`).
///
/// gRPC → JSON conversion leaves `owner_address` / `to_address` as `0x41…`
/// hex in `raw`; [`wallet_chain::tron::normalize_tron_address`] converts the
/// normalized columns. Raw JSON is left unchanged.
fn normalize_tron_tx(_height: u64, _idx: usize, tx: &mut NormalizedTx) {
    let raw = &tx.raw;
    if let Some(ret) = raw.get("ret").and_then(|v| v.as_array()) {
        if let Some(first) = ret.first() {
            let contract_ret = first
                .get("contractRet")
                .and_then(|v| v.as_str())
                .unwrap_or("SUCCESS");
            tx.status = if contract_ret == "SUCCESS" {
                TxStatus::Success
            } else {
                TxStatus::Failed
            };
        }
    }
    if let Some(from) = tx.from.take() {
        tx.from = wallet_chain::tron::normalize_tron_address(from.as_str()).or(Some(from));
    } else {
        tx.from = raw
            .get("raw_data")
            .and_then(|r| r.get("contract"))
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
            .and_then(|c| c.get("parameter"))
            .and_then(|p| p.get("value"))
            .and_then(|v| v.get("owner_address"))
            .and_then(|v| v.as_str())
            .and_then(wallet_chain::tron::normalize_tron_address);
    }
    if let Some(to) = tx.to.take() {
        tx.to = wallet_chain::tron::normalize_tron_address(to.as_str()).or(Some(to));
    }
    if let Some(contract) = tx.contract_address.take() {
        tx.contract_address =
            wallet_chain::tron::normalize_tron_address(contract.as_str()).or(Some(contract));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wallet_types::TxHash;

    fn tx(hash: &str, raw: serde_json::Value) -> NormalizedTx {
        NormalizedTx {
            hash: TxHash::new(hash),
            from: None,
            to: None,
            value: Amount::zero(18),
            gas_fee: None,
            block_number: 0,
            status: TxStatus::Pending,
            raw,
            contract_address: None,
            log_index: None,
            method: None,
        }
    }

    #[test]
    fn test_parse_block_sets_height() {
        let txs = parse_block(ChainIndex::ETH, 42, vec![tx("0xabc", json!({}))]);
        assert_eq!(txs[0].block_number, 42);
        assert_eq!(txs[0].status, TxStatus::Pending);
    }

    #[test]
    fn test_evm_extracts_from_to_value() {
        let raw = json!({
            "hash": "0xabc",
            "from": "0x111",
            "to": "0x222",
            "value": "0x64",
            "input": "0x"
        });
        let txs = parse_block(ChainIndex::ETH, 10, vec![tx("0xabc", raw)]);
        let out = &txs[0];
        assert_eq!(out.from.as_ref().unwrap().as_str(), "0x111");
        assert_eq!(out.to.as_ref().unwrap().as_str(), "0x222");
        assert_eq!(out.value.raw, Decimal::from(100));
        assert_eq!(out.value.decimals, 18);
    }

    #[test]
    fn test_evm_contract_creation_keeps_to_none() {
        let raw = json!({
            "hash": "0xabc",
            "from": "0x111",
            "to": null,
            "input": "0x6001"
        });
        let txs = parse_block(ChainIndex::ETH, 10, vec![tx("0xabc", raw)]);
        assert!(txs[0].to.is_none());
    }

    #[test]
    fn test_tron_status_from_ret() {
        let ok = json!({ "ret": [{ "contractRet": "SUCCESS" }] });
        let ok_txs = parse_block(ChainIndex::TRON, 5, vec![tx("0x1", ok)]);
        assert_eq!(ok_txs[0].status, TxStatus::Success);

        let fail = json!({ "ret": [{ "contractRet": "FAILED" }] });
        let fail_txs = parse_block(ChainIndex::TRON, 5, vec![tx("0x2", fail)]);
        assert_eq!(fail_txs[0].status, TxStatus::Failed);
    }

    #[test]
    fn test_tron_extracts_from() {
        let raw = json!({
            "raw_data": {
                "contract": [{
                    "parameter": { "value": { "owner_address": "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t" } }
                }]
            }
        });
        let txs = parse_block(ChainIndex::TRON, 5, vec![tx("0x1", raw)]);
        assert_eq!(
            txs[0].from.as_ref().unwrap().as_str(),
            "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t"
        );
    }

    #[test]
    fn test_tron_hex_owner_address_becomes_base58() {
        let raw = json!({
            "raw_data": {
                "contract": [{
                    "parameter": {
                        "value": {
                            "owner_address": "0x41a614f803b6fd780986a42c78ec9c7f77e6ded13c"
                        }
                    }
                }]
            }
        });
        let txs = parse_block(ChainIndex::TRON, 5, vec![tx("0x1", raw)]);
        assert_eq!(
            txs[0].from.as_ref().unwrap().as_str(),
            "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t"
        );
    }

    #[test]
    fn test_tron_rewrites_hex_from_to_fields() {
        let mut t = tx("0x1", json!({}));
        t.from = Some(Address::new("0x41a614f803b6fd780986a42c78ec9c7f77e6ded13c"));
        t.to = Some(Address::new("41a614f803b6fd780986a42c78ec9c7f77e6ded13c"));
        t.contract_address =
            Some(Address::new("0x41a614f803b6fd780986a42c78ec9c7f77e6ded13c"));
        let txs = parse_block(ChainIndex::TRON, 5, vec![t]);
        let out = &txs[0];
        assert_eq!(out.from.as_ref().unwrap().as_str(), "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t");
        assert_eq!(out.to.as_ref().unwrap().as_str(), "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t");
        assert_eq!(
            out.contract_address.as_ref().unwrap().as_str(),
            "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t"
        );
    }

    #[test]
    fn test_utxo_status_sets_success() {
        let txs = parse_block(ChainIndex::BTC, 100, vec![tx("btc1", json!({}))]);
        assert_eq!(txs[0].status, TxStatus::Success);
    }

    #[test]
    fn test_zcash_status_sets_success() {
        // Zcash is its own family but shares UTXO normalization.
        let txs = parse_block(ChainIndex::ZCASH, 100, vec![tx("zec1", json!({}))]);
        assert_eq!(txs[0].status, TxStatus::Success);
        assert_eq!(txs[0].block_number, 100);
    }

    #[test]
    fn test_ton_passes_through() {
        // TON txs are already fully populated by `TonChain::fetch_block_txs`;
        // parse_block should be a pass-through that just stamps the height.
        let raw = json!({
            "transaction_id": { "hash": "tonhash" },
            "in_msg": { "source": "EQ..src", "destination": "EQ..dst", "value": "1000" },
            "fee": "10",
        });
        let txs = parse_block(ChainIndex::TON, 42, vec![tx("tonhash", raw)]);
        assert_eq!(txs[0].block_number, 42);
        assert_eq!(txs[0].hash.as_str(), "tonhash");
    }

    #[test]
    fn test_sui_passes_through() {
        let raw = json!({
            "digest": "suihash",
            "transaction": {
                "data": { "sender": "0xsender", "transaction": { "kind": "ProgrammableTransaction" } }
            },
            "effects": { "status": { "status": "success" } },
        });
        let txs = parse_block(ChainIndex::SUI, 99, vec![tx("suihash", raw)]);
        assert_eq!(txs[0].block_number, 99);
        assert_eq!(txs[0].hash.as_str(), "suihash");
    }
}
