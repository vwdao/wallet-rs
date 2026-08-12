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
        let result = self
            .rpc("getSlot", json!([{ "commitment": "finalized" }]))
            .await?;
        Ok(result.as_u64().unwrap_or(0))
    }

    /// Fetch block transactions using `transactionDetails: "full"` so that SPL
    /// token transfers can be derived from pre/post token balance deltas.
    async fn fetch_block_txs(&self, height: u64) -> AppResult<Vec<NormalizedTx>> {
        let result = self
            .rpc(
                "getBlock",
                json!([
                    height,
                    {
                        "encoding": "json",
                        "transactionDetails": "full",
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
        let mut out = Vec::with_capacity(txs.len());
        for tx in txs {
            if is_pure_vote_tx(&tx) {
                continue;
            }
            out.extend(expand_solana_tx(height, &tx));
        }
        Ok(out)
    }
}

/// Expand one getBlock transaction into native + SPL balance-delta rows only
/// (no feePayer→accountKeys[1] shell row).
fn expand_solana_tx(height: u64, tx: &Value) -> Vec<NormalizedTx> {
    let sig = tx
        .get("transaction")
        .and_then(|t| t.get("signatures"))
        .and_then(|s| s.as_array())
        .and_then(|s| s.first())
        .and_then(|s| s.as_str())
        .unwrap_or("unknown");
    let status = match tx.pointer("/meta/err") {
        None | Some(Value::Null) => TxStatus::Success,
        _ => TxStatus::Failed,
    };
    let fee = tx
        .pointer("/meta/fee")
        .and_then(|v| v.as_u64())
        .map(|f| Amount::new(Decimal::from(f), 9));

    let mut out = solana_native_transfers(sig, height, status, fee.clone(), tx);
    out.extend(solana_token_transfers(sig, height, status, fee, tx));
    out
}

const VOTE_PROGRAM_ID: &str = "Vote111111111111111111111111111111111111111";
const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";
const SYSTEM_IX_TRANSFER: u32 = 2;
const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const TOKEN_2022_PROGRAM_ID: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
const TOKEN_IX_CLOSE_ACCOUNT: u8 = 9;
const WSOL_MINT: &str = "So11111111111111111111111111111111111111112";

fn account_key_str(key: &Value) -> Option<&str> {
    key.as_str()
        .or_else(|| key.get("pubkey").and_then(|v| v.as_str()))
}

/// Static `message.accountKeys` plus v0 `meta.loadedAddresses` (writable then readonly).
fn resolved_account_keys(tx: &Value) -> Vec<String> {
    let mut keys = Vec::new();
    if let Some(arr) = tx
        .pointer("/transaction/message/accountKeys")
        .and_then(|a| a.as_array())
    {
        for k in arr {
            if let Some(s) = account_key_str(k) {
                keys.push(s.to_string());
            }
        }
    }
    for path in [
        "/meta/loadedAddresses/writable",
        "/meta/loadedAddresses/readonly",
    ] {
        if let Some(arr) = tx.pointer(path).and_then(|a| a.as_array()) {
            for k in arr {
                if let Some(s) = account_key_str(k) {
                    keys.push(s.to_string());
                }
            }
        }
    }
    keys
}

fn token_account_addrs(tx: &Value, account_keys: &[String]) -> std::collections::HashSet<String> {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for path in ["/meta/preTokenBalances", "/meta/postTokenBalances"] {
        let Some(arr) = tx.pointer(path).and_then(|v| v.as_array()) else {
            continue;
        };
        for b in arr {
            let Some(idx) = b.get("accountIndex").and_then(|v| v.as_u64()) else {
                continue;
            };
            if let Some(addr) = account_keys.get(idx as usize) {
                set.insert(addr.clone());
            }
        }
    }
    set
}

/// True when every top-level instruction targets the Vote program.
/// Empty instruction lists are not treated as votes.
fn is_pure_vote_tx(tx: &Value) -> bool {
    let account_keys = resolved_account_keys(tx);
    let Some(instructions) = tx
        .pointer("/transaction/message/instructions")
        .and_then(|i| i.as_array())
    else {
        return false;
    };
    if instructions.is_empty() {
        return false;
    }
    instructions.iter().all(|ix| {
        let Some(idx) = ix.get("programIdIndex").and_then(|v| v.as_u64()) else {
            return false;
        };
        account_keys
            .get(idx as usize)
            .is_some_and(|id| id == VOTE_PROGRAM_ID)
    })
}

/// Derive native SOL rows: System `transfer` + Token `closeAccount`, then residual
/// fee-adjusted balance deltas for anything still unexplained.
fn solana_native_transfers(
    sig: &str,
    height: u64,
    status: TxStatus,
    gas_fee: Option<Amount>,
    tx: &Value,
) -> Vec<NormalizedTx> {
    use std::collections::HashMap;

    let account_keys = resolved_account_keys(tx);
    if account_keys.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    let mut seq: u64 = 0;
    let mut explained_from: HashMap<String, Decimal> = HashMap::new();
    let mut explained_to: HashMap<String, Decimal> = HashMap::new();

    let push_native = |from: String,
                       to: String,
                       amount: Decimal,
                       gas_fee: &Option<Amount>,
                       seq: &mut u64,
                       out: &mut Vec<NormalizedTx>| {
        out.push(NormalizedTx {
            hash: TxHash::new(sig),
            from: Some(Address::new(from)),
            to: Some(Address::new(to)),
            value: Amount::new(amount, 9),
            gas_fee: gas_fee.clone(),
            block_number: height,
            status,
            raw: json!({ "signature": sig, "type": "native_transfer" }),
            contract_address: None,
            log_index: Some(*seq as i64),
            method: Some("native_transfer".to_string()),
        });
        *seq += 1;
    };

    for (from, to, lamports) in parse_system_transfers(tx, &account_keys) {
        let amount = Decimal::from(lamports);
        *explained_from.entry(from.clone()).or_default() += amount;
        *explained_to.entry(to.clone()).or_default() += amount;
        push_native(from, to, amount, &gas_fee, &mut seq, &mut out);
    }

    let Some(pre) = tx.pointer("/meta/preBalances").and_then(|v| v.as_array()) else {
        return out;
    };
    let Some(post) = tx.pointer("/meta/postBalances").and_then(|v| v.as_array()) else {
        return out;
    };

    for (from, to, lamports) in parse_token_closes(tx, &account_keys, pre, post) {
        let amount = Decimal::from(lamports);
        if amount.is_zero() {
            continue;
        }
        *explained_from.entry(from.clone()).or_default() += amount;
        *explained_to.entry(to.clone()).or_default() += amount;
        push_native(from, to, amount, &gas_fee, &mut seq, &mut out);
    }

    let fee_lamports = tx.pointer("/meta/fee").and_then(|v| v.as_u64()).unwrap_or(0);
    let token_accts = token_account_addrs(tx, &account_keys);

    let mut deltas: Vec<(String, i128)> = Vec::new();
    let len = pre.len().min(post.len()).min(account_keys.len());
    for i in 0..len {
        let addr = &account_keys[i];
        let pre_bal = pre[i].as_u64().unwrap_or(0) as i128;
        let post_bal = post[i].as_u64().unwrap_or(0) as i128;
        let mut delta = post_bal - pre_bal;
        if i == 0 {
            delta += fee_lamports as i128;
        }
        if delta != 0 {
            deltas.push((addr.clone(), delta));
        }
    }

    // Subtract amounts already covered by System::transfer / Token::closeAccount.
    // Keep token-account losers for residual matching, but skip token-account gainers.
    let mut losers: Vec<(String, Decimal)> = Vec::new();
    let mut gainers: Vec<(String, Decimal)> = Vec::new();
    for (addr, delta) in deltas {
        if delta < 0 {
            let loss = Decimal::from((-delta) as u64);
            let explained = explained_from.remove(&addr).unwrap_or(Decimal::ZERO);
            let residual = loss - explained;
            if residual > Decimal::ZERO {
                losers.push((addr, residual));
            }
        } else if delta > 0 {
            let gain = Decimal::from(delta as u64);
            let explained = explained_to.remove(&addr).unwrap_or(Decimal::ZERO);
            let residual = gain - explained;
            if residual > Decimal::ZERO && !token_accts.contains(&addr) {
                gainers.push((addr, residual));
            }
        }
    }

    if gainers.is_empty() {
        // If instruction rows already explain the transfers, skip leftover
        // loser-only noise (ambiguous co-losers with the same amount).
        if !out.is_empty() {
            return out;
        }
        for (owner, amount) in losers {
            out.push(NormalizedTx {
                hash: TxHash::new(sig),
                from: Some(Address::new(owner)),
                to: None,
                value: Amount::new(amount, 9),
                gas_fee: gas_fee.clone(),
                block_number: height,
                status,
                raw: json!({ "signature": sig, "type": "native_transfer" }),
                contract_address: None,
                log_index: Some(seq as i64),
                method: Some("native_transfer".to_string()),
            });
            seq += 1;
        }
        return out;
    }

    for (from, to, amount) in match_balance_transfers(&losers, &gainers) {
        out.push(NormalizedTx {
            hash: TxHash::new(sig),
            from: from.map(Address::new),
            to: Some(Address::new(to)),
            value: Amount::new(amount, 9),
            gas_fee: gas_fee.clone(),
            block_number: height,
            status,
            raw: json!({ "signature": sig, "type": "native_transfer" }),
            contract_address: None,
            log_index: Some(seq as i64),
            method: Some("native_transfer".to_string()),
        });
        seq += 1;
    }
    out
}

/// Decode System Program `transfer` ixs from top-level and inner instructions.
fn parse_system_transfers(tx: &Value, account_keys: &[String]) -> Vec<(String, String, u64)> {
    let mut out = Vec::new();
    let mut push_from = |ixs: &[Value]| {
        for ix in ixs {
            if let Some(t) = decode_system_transfer(ix, account_keys) {
                out.push(t);
            }
        }
    };
    if let Some(ixs) = tx
        .pointer("/transaction/message/instructions")
        .and_then(|a| a.as_array())
    {
        push_from(ixs);
    }
    if let Some(groups) = tx
        .pointer("/meta/innerInstructions")
        .and_then(|a| a.as_array())
    {
        for g in groups {
            if let Some(ixs) = g.get("instructions").and_then(|a| a.as_array()) {
                push_from(ixs);
            }
        }
    }
    out
}

fn decode_system_transfer(ix: &Value, account_keys: &[String]) -> Option<(String, String, u64)> {
    let prog_idx = ix.get("programIdIndex").and_then(|v| v.as_u64())? as usize;
    if account_keys.get(prog_idx).map(String::as_str) != Some(SYSTEM_PROGRAM_ID) {
        return None;
    }
    let accounts = ix.get("accounts").and_then(|a| a.as_array())?;
    let from_idx = accounts.first()?.as_u64()? as usize;
    let to_idx = accounts.get(1)?.as_u64()? as usize;
    let from = account_keys.get(from_idx)?.clone();
    let to = account_keys.get(to_idx)?.clone();
    let data_b58 = ix.get("data").and_then(|v| v.as_str())?;
    let data = bs58::decode(data_b58).into_vec().ok()?;
    if data.len() < 12 {
        return None;
    }
    let disc = u32::from_le_bytes(data[0..4].try_into().ok()?);
    if disc != SYSTEM_IX_TRANSFER {
        return None;
    }
    let lamports = u64::from_le_bytes(data[4..12].try_into().ok()?);
    Some((from, to, lamports))
}

/// Token/Token-2022 `closeAccount` → lamports from closed account to destination.
fn parse_token_closes(
    tx: &Value,
    account_keys: &[String],
    pre: &[Value],
    post: &[Value],
) -> Vec<(String, String, u64)> {
    let mut out = Vec::new();
    let mut push_from = |ixs: &[Value]| {
        for ix in ixs {
            if let Some(t) = decode_token_close(ix, account_keys, pre, post) {
                out.push(t);
            }
        }
    };
    if let Some(ixs) = tx
        .pointer("/transaction/message/instructions")
        .and_then(|a| a.as_array())
    {
        push_from(ixs);
    }
    if let Some(groups) = tx
        .pointer("/meta/innerInstructions")
        .and_then(|a| a.as_array())
    {
        for g in groups {
            if let Some(ixs) = g.get("instructions").and_then(|a| a.as_array()) {
                push_from(ixs);
            }
        }
    }
    out
}

fn decode_token_close(
    ix: &Value,
    account_keys: &[String],
    pre: &[Value],
    post: &[Value],
) -> Option<(String, String, u64)> {
    let prog_idx = ix.get("programIdIndex").and_then(|v| v.as_u64())? as usize;
    let prog = account_keys.get(prog_idx).map(String::as_str)?;
    if prog != TOKEN_PROGRAM_ID && prog != TOKEN_2022_PROGRAM_ID {
        return None;
    }
    let data_b58 = ix.get("data").and_then(|v| v.as_str())?;
    let data = bs58::decode(data_b58).into_vec().ok()?;
    if data.first().copied() != Some(TOKEN_IX_CLOSE_ACCOUNT) {
        return None;
    }
    let accounts = ix.get("accounts").and_then(|a| a.as_array())?;
    let closed_idx = accounts.first()?.as_u64()? as usize;
    let dest_idx = accounts.get(1)?.as_u64()? as usize;
    let from = account_keys.get(closed_idx)?.clone();
    let to = account_keys.get(dest_idx)?.clone();
    let pre_bal = pre.get(closed_idx)?.as_u64()?;
    let post_bal = post.get(closed_idx).and_then(|v| v.as_u64()).unwrap_or(0);
    let lamports = pre_bal.saturating_sub(post_bal);
    Some((from, to, lamports))
}

/// Pair each gainer with a loser: prefer exact amount match, else debit the
/// largest remaining loser capacity. Avoids stamping one "max loser" onto every row.
fn match_balance_transfers(
    losers: &[(String, Decimal)],
    gainers: &[(String, Decimal)],
) -> Vec<(Option<String>, String, Decimal)> {
    let mut remaining: Vec<(String, Decimal)> = losers.to_vec();
    let mut out = Vec::with_capacity(gainers.len());
    for (gainer, amount) in gainers {
        let sender = if let Some(i) = remaining.iter().position(|(_, a)| *a == *amount) {
            let (addr, _) = remaining.remove(i);
            Some(addr)
        } else if let Some((i, _)) = remaining
            .iter()
            .enumerate()
            .filter(|(_, (_, a))| *a >= *amount)
            .max_by(|(_, a), (_, b)| a.1.cmp(&b.1))
        {
            let addr = remaining[i].0.clone();
            remaining[i].1 -= *amount;
            if remaining[i].1.is_zero() {
                remaining.remove(i);
            }
            Some(addr)
        } else {
            None
        };
        out.push((sender, gainer.clone(), *amount));
    }
    out
}

/// Derive SPL token transfer rows from pre/post token balance deltas in `meta`.
fn solana_token_transfers(
    sig: &str,
    height: u64,
    status: TxStatus,
    gas_fee: Option<Amount>,
    tx: &Value,
) -> Vec<NormalizedTx> {
    use std::collections::HashMap;

    // (mint, owner) -> amount delta, aggregated across accounts
    let mut deltas: HashMap<(String, String), Decimal> = HashMap::new();
    if let Some(arr) = tx.pointer("/meta/preTokenBalances").and_then(|v| v.as_array()) {
        apply_token_balances(&mut deltas, arr, Decimal::NEGATIVE_ONE);
    }
    if let Some(arr) = tx.pointer("/meta/postTokenBalances").and_then(|v| v.as_array()) {
        apply_token_balances(&mut deltas, arr, Decimal::ONE);
    }

    // group deltas by mint
    let mut by_mint: HashMap<String, Vec<(String, Decimal)>> = HashMap::new();
    for ((mint, owner), delta) in deltas {
        if delta.is_zero() {
            continue;
        }
        by_mint.entry(mint).or_default().push((owner, delta));
    }

    let mut out = Vec::new();
    for (mint, parties) in by_mint {
        let losers: Vec<(String, Decimal)> = parties
            .iter()
            .filter(|(_, d)| d.is_sign_negative())
            .map(|(o, d)| (o.clone(), -*d))
            .collect();
        let gainers: Vec<(String, Decimal)> = parties
            .iter()
            .filter(|(_, d)| d.is_sign_positive())
            .map(|(o, d)| (o.clone(), *d))
            .collect();
        // WSOL wrap (gainer-only) / unwrap (loser-only): covered by native SOL rows.
        if mint == WSOL_MINT && (losers.is_empty() || gainers.is_empty()) {
            continue;
        }
        let mut seq: u64 = 0;
        if gainers.is_empty() {
            // Loser-only token delta = burn (explorer shows Burn, no recipient).
            for (owner, amount) in losers {
                out.push(NormalizedTx {
                    hash: TxHash::new(sig),
                    from: Some(Address::new(owner)),
                    to: None,
                    value: Amount::new(amount, 9),
                    gas_fee: gas_fee.clone(),
                    block_number: height,
                    status,
                    raw: json!({ "signature": sig, "mint": mint.clone(), "type": "spl_burn" }),
                    contract_address: Some(Address::new(mint.clone())),
                    log_index: Some(seq as i64),
                    method: Some("spl_burn".to_string()),
                });
                seq += 1;
            }
            continue;
        }
        if losers.is_empty() {
            // Gainer-only = mint (do not fake from=feePayer).
            for (owner, amount) in gainers {
                out.push(NormalizedTx {
                    hash: TxHash::new(sig),
                    from: None,
                    to: Some(Address::new(owner)),
                    value: Amount::new(amount, 9),
                    gas_fee: gas_fee.clone(),
                    block_number: height,
                    status,
                    raw: json!({ "signature": sig, "mint": mint.clone(), "type": "spl_mint" }),
                    contract_address: Some(Address::new(mint.clone())),
                    log_index: Some(seq as i64),
                    method: Some("spl_mint".to_string()),
                });
                seq += 1;
            }
            continue;
        }
        for (from, to, amount) in match_balance_transfers(&losers, &gainers) {
            out.push(NormalizedTx {
                hash: TxHash::new(sig),
                from: from.map(Address::new),
                to: Some(Address::new(to)),
                value: Amount::new(amount, 9),
                gas_fee: gas_fee.clone(),
                block_number: height,
                status,
                raw: json!({ "signature": sig, "mint": mint.clone(), "type": "spl_transfer" }),
                contract_address: Some(Address::new(mint.clone())),
                log_index: Some(seq as i64),
                method: Some("spl_transfer".to_string()),
            });
            seq += 1;
        }
    }
    out
}

/// Accumulate raw token amounts from a `preTokenBalances`/`postTokenBalances`
/// array into `deltas` (keyed by mint+owner), weighted by `sign`.
fn apply_token_balances(
    deltas: &mut std::collections::HashMap<(String, String), Decimal>,
    arr: &[Value],
    sign: Decimal,
) {
    for b in arr {
        let Some(mint) = b.get("mint").and_then(|v| v.as_str()) else {
            continue;
        };
        let owner = b
            .get("owner")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let Some(amount) = b
            .pointer("/uiTokenAmount/amount")
            .and_then(|v| v.as_str())
            .and_then(|s| Decimal::from_str_exact(s).ok())
        else {
            continue;
        };
        let e = deltas.entry((mint.to_string(), owner)).or_default();
        *e += sign * amount;
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
            ..GasEstimate::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VOTE: &str = "Vote111111111111111111111111111111111111111";
    const SYSTEM: &str = "11111111111111111111111111111111";

    fn compiled_tx(account_keys: Vec<Value>, instructions: Vec<Value>) -> Value {
        json!({
            "transaction": {
                "signatures": ["sig1"],
                "message": {
                    "accountKeys": account_keys,
                    "instructions": instructions
                }
            },
            "meta": { "err": null }
        })
    }

    #[test]
    fn pure_vote_tx_is_detected() {
        let tx = compiled_tx(
            vec![json!("voter"), json!(VOTE)],
            vec![json!({ "programIdIndex": 1, "accounts": [0], "data": "" })],
        );
        assert!(is_pure_vote_tx(&tx));
    }

    #[test]
    fn mixed_vote_and_system_is_not_pure_vote() {
        let tx = compiled_tx(
            vec![json!("payer"), json!(VOTE), json!(SYSTEM)],
            vec![
                json!({ "programIdIndex": 1, "accounts": [0], "data": "" }),
                json!({ "programIdIndex": 2, "accounts": [0], "data": "" }),
            ],
        );
        assert!(!is_pure_vote_tx(&tx));
    }

    #[test]
    fn empty_instructions_is_not_pure_vote() {
        let tx = compiled_tx(vec![json!("payer"), json!(VOTE)], vec![]);
        assert!(!is_pure_vote_tx(&tx));
    }

    #[test]
    fn pure_vote_with_pubkey_objects() {
        let tx = compiled_tx(
            vec![json!({ "pubkey": "voter" }), json!({ "pubkey": VOTE })],
            vec![json!({ "programIdIndex": 1, "accounts": [0], "data": "" })],
        );
        assert!(is_pure_vote_tx(&tx));
    }

    fn balance(account_index: u64, mint: &str, owner: &str, amount: &str) -> Value {
        json!({
            "accountIndex": account_index,
            "mint": mint,
            "owner": owner,
            "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "uiTokenAmount": { "amount": amount, "decimals": 9, "uiAmount": 1.0 }
        })
    }

    fn tx(pre: Vec<Value>, post: Vec<Value>) -> Value {
        json!({
            "transaction": {
                "signatures": ["sig1"],
                "message": { "accountKeys": ["feePayer", "bob", "tokenProgram"] }
            },
            "meta": { "err": null, "preTokenBalances": pre, "postTokenBalances": post }
        })
    }

    #[test]
    fn test_spl_transfer_delta() {
        let pre = vec![balance(0, "MINT", "alice", "1000")];
        let post = vec![
            balance(0, "MINT", "alice", "500"),
            balance(1, "MINT", "bob", "500"),
        ];
        let rows = solana_token_transfers("sig1", 42, TxStatus::Success, Some(Amount::new(Decimal::from(5000), 9)), &tx(pre, post));
        assert_eq!(rows.len(), 1);
        let r = &rows[0];
        assert_eq!(r.from.as_ref().unwrap().as_str(), "alice");
        assert_eq!(r.to.as_ref().unwrap().as_str(), "bob");
        assert_eq!(r.value.raw, Decimal::from(500));
        assert_eq!(r.gas_fee.as_ref().unwrap().raw, Decimal::from(5000));
        assert_eq!(r.contract_address.as_ref().unwrap().as_str(), "MINT");
        assert_eq!(r.block_number, 42);
    }

    #[test]
    fn test_spl_no_balance_change() {
        let pre = vec![balance(0, "MINT", "alice", "1000")];
        let post = vec![balance(0, "MINT", "alice", "1000")];
        let rows = solana_token_transfers("sig1", 1, TxStatus::Success, None, &tx(pre, post));
        assert!(rows.is_empty());
    }

    #[test]
    fn test_spl_burn_no_recipient() {
        let pre = vec![balance(0, "MINT", "alice", "1000")];
        let post: Vec<Value> = vec![];
        let rows = solana_token_transfers("sig1", 1, TxStatus::Success, None, &tx(pre, post));
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].from.as_ref().unwrap().as_str(), "alice");
        assert!(rows[0].to.is_none());
        assert_eq!(rows[0].value.raw, Decimal::from(1000));
        assert_eq!(rows[0].method.as_deref(), Some("spl_burn"));
        assert_eq!(rows[0].raw.get("type").and_then(|v| v.as_str()), Some("spl_burn"));
    }

    #[test]
    fn native_close_account_keeps_token_account_as_from() {
        // Token::closeAccount returns lamports; from must be the closed ATA.
        let close_data = bs58::encode([TOKEN_IX_CLOSE_ACCOUNT]).into_string();
        let tx = json!({
            "transaction": {
                "signatures": ["sig1"],
                "message": {
                    "accountKeys": ["wallet", "closedAta", TOKEN_PROGRAM_ID],
                    "instructions": [{
                        "programIdIndex": 2,
                        "accounts": [1, 0, 0],
                        "data": close_data
                    }]
                }
            },
            "meta": {
                "err": null,
                "fee": 5_000,
                "preBalances": [1_000_000, 9_695_280, 1],
                "postBalances": [10_690_280, 0, 1],
                "preTokenBalances": [{
                    "accountIndex": 1,
                    "mint": "MINT",
                    "owner": "wallet",
                    "uiTokenAmount": { "amount": "0", "decimals": 0 }
                }],
                "postTokenBalances": []
            }
        });
        let fee = Amount::new(Decimal::from(5_000u64), 9);
        let rows = solana_native_transfers("sig1", 1, TxStatus::Success, Some(fee), &tx);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].from.as_ref().unwrap().as_str(), "closedAta");
        assert_eq!(rows[0].to.as_ref().unwrap().as_str(), "wallet");
        assert_eq!(rows[0].value.raw, Decimal::from(9_695_280u64));
    }

    #[test]
    fn native_close_account_from_inner_instruction() {
        let close_data = bs58::encode([TOKEN_IX_CLOSE_ACCOUNT]).into_string();
        let tx = json!({
            "transaction": {
                "signatures": ["sig1"],
                "message": {
                    "accountKeys": ["wallet", "craftPda", TOKEN_PROGRAM_ID],
                    "instructions": []
                }
            },
            "meta": {
                "err": null,
                "fee": 0,
                "preBalances": [1_000_000, 9_695_280, 1],
                "postBalances": [10_695_280, 0, 1],
                "innerInstructions": [{
                    "index": 0,
                    "instructions": [{
                        "programIdIndex": 2,
                        "accounts": [1, 0, 0],
                        "data": close_data
                    }]
                }],
                "preTokenBalances": [],
                "postTokenBalances": []
            }
        });
        let rows = solana_native_transfers("sig1", 1, TxStatus::Success, None, &tx);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].from.as_ref().unwrap().as_str(), "craftPda");
        assert_eq!(rows[0].to.as_ref().unwrap().as_str(), "wallet");
    }

    const WSOL: &str = "So11111111111111111111111111111111111111112";

    #[test]
    fn wsol_gainer_only_wrap_rows_are_dropped() {
        let post = vec![
            balance(1, WSOL, "bob", "10125"),
            balance(2, WSOL, "carol", "93143"),
        ];
        let rows = solana_token_transfers("sig1", 1, TxStatus::Success, None, &tx(vec![], post));
        assert!(rows.is_empty());
    }

    #[test]
    fn non_wsol_gainer_only_is_mint() {
        let post = vec![balance(1, "MINT", "bob", "500")];
        let rows = solana_token_transfers("sig1", 1, TxStatus::Success, None, &tx(vec![], post));
        assert_eq!(rows.len(), 1);
        assert!(rows[0].from.is_none());
        assert_eq!(rows[0].to.as_ref().unwrap().as_str(), "bob");
        assert_eq!(rows[0].method.as_deref(), Some("spl_mint"));
    }

    #[test]
    fn wsol_loser_only_unwrap_rows_are_dropped() {
        let pre = vec![balance(1, WSOL, "alice", "1000")];
        let rows = solana_token_transfers("sig1", 1, TxStatus::Success, None, &tx(pre, vec![]));
        assert!(rows.is_empty());
    }

    #[test]
    fn wsol_with_loser_is_kept() {
        let pre = vec![balance(0, WSOL, "alice", "1000")];
        let post = vec![
            balance(0, WSOL, "alice", "400"),
            balance(1, WSOL, "bob", "600"),
        ];
        let rows = solana_token_transfers("sig1", 1, TxStatus::Success, None, &tx(pre, post));
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].from.as_ref().unwrap().as_str(), "alice");
        assert_eq!(rows[0].to.as_ref().unwrap().as_str(), "bob");
    }

    fn native_tx(keys: Vec<&str>, pre: Vec<u64>, post: Vec<u64>, fee: u64) -> Value {
        json!({
            "transaction": {
                "signatures": ["sig1"],
                "message": { "accountKeys": keys, "instructions": [] }
            },
            "meta": {
                "err": null,
                "fee": fee,
                "preBalances": pre,
                "postBalances": post,
                "preTokenBalances": [],
                "postTokenBalances": []
            }
        })
    }

    #[test]
    fn native_sol_transfer_fee_adjusted() {
        // alice pays 5000 fee and sends 1_000_000 lamports to bob
        let tx = native_tx(
            vec!["alice", "bob", SYSTEM],
            vec![10_000_000, 1_000_000, 1],
            vec![8_995_000, 2_000_000, 1],
            5_000,
        );
        let fee = Amount::new(Decimal::from(5_000u64), 9);
        let rows = solana_native_transfers("sig1", 10, TxStatus::Success, Some(fee), &tx);
        assert_eq!(rows.len(), 1);
        let r = &rows[0];
        assert_eq!(r.from.as_ref().unwrap().as_str(), "alice");
        assert_eq!(r.to.as_ref().unwrap().as_str(), "bob");
        assert_eq!(r.value.raw, Decimal::from(1_000_000u64));
        assert_eq!(r.method.as_deref(), Some("native_transfer"));
        assert!(r.contract_address.is_none());
    }

    #[test]
    fn native_multi_gainer_keeps_from_on_every_row() {
        // alice funds bob (+1000) and carol (+2000); fee 5000 neutralized
        let tx = native_tx(
            vec!["alice", "bob", "carol"],
            vec![10_000_000, 0, 0],
            vec![9_992_000, 1_000, 2_000],
            5_000,
        );
        let fee = Amount::new(Decimal::from(5_000u64), 9);
        let rows = solana_native_transfers("sig1", 1, TxStatus::Success, Some(fee), &tx);
        assert_eq!(rows.len(), 2);
        for r in &rows {
            assert_eq!(r.from.as_ref().unwrap().as_str(), "alice");
            assert!(r.to.is_some());
        }
    }

    #[test]
    fn native_pairs_from_by_matching_amount() {
        // Two independent transfers: alice→bob 147552, carol→dave 1844400.
        // Largest loser must NOT be stamped onto both rows.
        let tx = native_tx(
            vec!["feePayer", "alice", "bob", "carol", "dave"],
            vec![1_000_000, 1_000_000, 0, 5_000_000, 0],
            vec![990_000, 852_448, 147_552, 3_155_600, 1_844_400],
            10_000,
        );
        let fee = Amount::new(Decimal::from(10_000u64), 9);
        let rows = solana_native_transfers("sig1", 1, TxStatus::Success, Some(fee), &tx);
        let mut by_to: std::collections::HashMap<String, (&str, Decimal)> =
            std::collections::HashMap::new();
        for r in &rows {
            by_to.insert(
                r.to.as_ref().unwrap().as_str().to_string(),
                (r.from.as_ref().unwrap().as_str(), r.value.raw),
            );
        }
        assert_eq!(
            by_to.get("bob"),
            Some(&("alice", Decimal::from(147_552u64)))
        );
        assert_eq!(
            by_to.get("dave"),
            Some(&("carol", Decimal::from(1_844_400u64)))
        );
    }

    fn system_transfer_data(lamports: u64) -> String {
        let mut raw = Vec::with_capacity(12);
        raw.extend_from_slice(&SYSTEM_IX_TRANSFER.to_le_bytes());
        raw.extend_from_slice(&lamports.to_le_bytes());
        bs58::encode(raw).into_string()
    }

    #[test]
    fn native_uses_system_transfer_ix_when_balance_match_ambiguous() {
        // Balances: sponsor also loses 1844400, so amount-matching alone can pick the
        // wrong from. System::transfer ixs identify the real senders.
        // keys: feePayer(0), alice(1), bob(2), carol(3), dave(4), sponsor(5), system(6)
        let tx = json!({
            "transaction": {
                "signatures": ["sig1"],
                "message": {
                    "accountKeys": [
                        "feePayer", "alice", "bob", "carol", "dave", "sponsor", SYSTEM
                    ],
                    "instructions": [
                        {
                            "programIdIndex": 6,
                            "accounts": [1, 2],
                            "data": system_transfer_data(147_552)
                        },
                        {
                            "programIdIndex": 6,
                            "accounts": [3, 4],
                            "data": system_transfer_data(1_844_400)
                        }
                    ]
                }
            },
            "meta": {
                "err": null,
                "fee": 10_000,
                "preBalances": [1_000_000, 1_000_000, 0, 5_000_000, 0, 2_000_000, 1],
                "postBalances": [990_000, 852_448, 147_552, 3_155_600, 1_844_400, 155_600, 1],
                "preTokenBalances": [],
                "postTokenBalances": []
            }
        });
        // sponsor: 2000000-155600=1844400 loss — same amount as carol→dave
        let fee = Amount::new(Decimal::from(10_000u64), 9);
        let rows = solana_native_transfers("sig1", 1, TxStatus::Success, Some(fee), &tx);
        let mut by_to: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        for r in &rows {
            by_to.insert(
                r.to.as_ref().unwrap().as_str().to_string(),
                r.from.as_ref().unwrap().as_str().to_string(),
            );
        }
        assert_eq!(by_to.get("bob").map(String::as_str), Some("alice"));
        assert_eq!(by_to.get("dave").map(String::as_str), Some("carol"));
        assert!(!by_to.values().any(|f| f == "sponsor"));
    }

    #[test]
    fn resolved_keys_include_loaded_addresses() {
        let tx = json!({
            "transaction": {
                "message": { "accountKeys": ["alice", "bob"] }
            },
            "meta": {
                "loadedAddresses": {
                    "writable": ["vault", "tempAta"],
                    "readonly": ["program"]
                }
            }
        });
        let keys = resolved_account_keys(&tx);
        assert_eq!(keys, vec!["alice", "bob", "vault", "tempAta", "program"]);
    }

    #[test]
    fn native_uses_loaded_addresses_for_from() {
        // Static keys alone cannot explain bob's gain; vault (loaded) is the loser.
        let tx = json!({
            "transaction": {
                "signatures": ["sig1"],
                "message": { "accountKeys": ["bob"], "instructions": [] }
            },
            "meta": {
                "err": null,
                "fee": 0,
                "preBalances": [1_000_000, 5_000_000],
                "postBalances": [2_000_000, 4_000_000],
                "loadedAddresses": { "writable": ["vault"], "readonly": [] },
                "preTokenBalances": [],
                "postTokenBalances": []
            }
        });
        let rows = solana_native_transfers("sig1", 1, TxStatus::Success, None, &tx);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].from.as_ref().unwrap().as_str(), "vault");
        assert_eq!(rows[0].to.as_ref().unwrap().as_str(), "bob");
        assert_eq!(rows[0].value.raw, Decimal::from(1_000_000u64));
    }

    #[test]
    fn native_skips_token_account_gainers() {
        // WSOL ATA receives lamports (already covered by SPL); only system account row kept.
        let tx = json!({
            "transaction": {
                "signatures": ["sig1"],
                "message": {
                    "accountKeys": ["alice", "bobAta", "carol"],
                    "instructions": []
                }
            },
            "meta": {
                "err": null,
                "fee": 0,
                "preBalances": [10_000_000, 0, 0],
                "postBalances": [9_000_000, 600_000, 400_000],
                "preTokenBalances": [],
                "postTokenBalances": [
                    {
                        "accountIndex": 1,
                        "mint": WSOL,
                        "owner": "bobOwner",
                        "uiTokenAmount": { "amount": "600000", "decimals": 9 }
                    }
                ]
            }
        });
        let rows = solana_native_transfers("sig1", 1, TxStatus::Success, None, &tx);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].from.as_ref().unwrap().as_str(), "alice");
        assert_eq!(rows[0].to.as_ref().unwrap().as_str(), "carol");
        assert_eq!(rows[0].value.raw, Decimal::from(400_000u64));
    }

    #[test]
    fn create_close_temp_account_nets_zero_no_native_row() {
        // temp account created then closed in-tx: pre=0, post=0
        let tx = native_tx(
            vec!["payer", "temp", SYSTEM],
            vec![5_000_000, 0, 1],
            vec![4_995_000, 0, 1],
            5_000,
        );
        let fee = Amount::new(Decimal::from(5_000u64), 9);
        let rows = solana_native_transfers("sig1", 1, TxStatus::Success, Some(fee), &tx);
        assert!(rows.iter().all(|r| {
            r.to.as_ref().map(|a| a.as_str()) != Some("temp")
                && r.from.as_ref().map(|a| a.as_str()) != Some("temp")
        }));
        assert!(rows.is_empty());
    }

    #[test]
    fn expand_solana_tx_skips_shell_row() {
        let tx = native_tx(
            vec!["payer", "tempAccount", SYSTEM],
            vec![1_000_000, 0, 1],
            vec![995_000, 0, 1],
            5_000,
        );
        let rows = expand_solana_tx(7, &tx);
        assert!(rows.iter().all(|r| r.method.is_some()));
        assert!(rows
            .iter()
            .all(|r| !(r.value.raw.is_zero() && r.method.is_none())));
        assert!(!rows.iter().any(|r| {
            r.to.as_ref().map(|a| a.as_str()) == Some("tempAccount") && r.method.is_none()
        }));
    }
}
