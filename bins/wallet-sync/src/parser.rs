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
        None => {}
    }
    tx
}

/// EVM: extract proper from/to/value from raw JSON.
fn normalize_evm_tx(_chain_index: ChainIndex, _idx: usize, tx: &mut NormalizedTx) {
    let raw = &tx.raw;
    if tx.from.is_none() {
        tx.from = raw
            .get("from")
            .and_then(|v| v.as_str())
            .map(Address::new);
    }
    if tx.to.is_none() {
        tx.to = raw
            .get("to")
            .and_then(|v| v.as_str())
            .map(Address::new);
    }
    if let Some(val_hex) = raw.get("value").and_then(|v| v.as_str()) {
        if let Ok(raw_val) = u128::from_str_radix(val_hex.trim_start_matches("0x"), 16) {
            tx.value = Amount::new(Decimal::from(raw_val), 18);
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

/// TRON: extract from raw JSON.
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
    if tx.from.is_none() {
        tx.from = raw
            .get("raw_data")
            .and_then(|r| r.get("contract"))
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
            .and_then(|c| c.get("parameter"))
            .and_then(|p| p.get("value"))
            .and_then(|v| v.get("owner_address"))
            .and_then(|v| v.as_str())
            .map(Address::new);
    }
}
