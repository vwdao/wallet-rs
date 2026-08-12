//! Gas pool refill sweeper — checks balances and triggers real transfers from cold wallet.

use std::sync::Arc;
use std::time::Duration;
use wallet_chain::NonceProvider;
use wallet_db::GasPoolRepo;
use wallet_domain::AppState;
use wallet_error::{AppError, AppResult};
use wallet_types::{Address, ChainFamily, ChainIndex};

/// Chain-specific refill thresholds. All amounts are expressed in the chain's
/// smallest native unit: EVM chains use wei (18 decimals), Solana uses
/// lamports (9 decimals), TRON uses SUN (6 decimals).
fn threshold_for_chain(chain_index: ChainIndex) -> (rust_decimal::Decimal, rust_decimal::Decimal) {
    match chain_index {
        // EVM: 0.1 ETH threshold, 0.5 ETH target
        ChainIndex::ETH
        | ChainIndex::BSC
        | ChainIndex::POL
        | ChainIndex::ARB
        | ChainIndex::OP
        | ChainIndex::BASE => (
            rust_decimal::Decimal::from_str_radix("100000000000000000", 10).unwrap(),
            rust_decimal::Decimal::from_str_radix("500000000000000000", 10).unwrap(),
        ),
        // Solana: 0.5 SOL threshold, 2 SOL target
        ChainIndex::SOL => (
            rust_decimal::Decimal::from_str_radix("500000000", 10).unwrap(),
            rust_decimal::Decimal::from_str_radix("2000000000", 10).unwrap(),
        ),
        // TRON: 100 TRX threshold, 500 TRX target
        ChainIndex::TRON => (
            rust_decimal::Decimal::from_str_radix("100000000", 10).unwrap(),
            rust_decimal::Decimal::from_str_radix("500000000", 10).unwrap(),
        ),
        // BTC: no gas refill concept
        _ => (rust_decimal::Decimal::ZERO, rust_decimal::Decimal::ZERO),
    }
}

/// Gas pool refill sweeper: checks hot wallet balances and triggers real refill transfers.
pub async fn run(state: Arc<AppState>) -> anyhow::Result<()> {
    loop {
        if let Err(e) = tick(state.as_ref()).await {
            tracing::warn!(error = %e, "gas_refill tick failed");
        }
        tokio::time::sleep(Duration::from_secs(120)).await;
    }
}

async fn tick(state: &AppState) -> AppResult<()> {
    let gas_repo = GasPoolRepo::new(&state.db);

    // All EVM + Solana + TRON chains to monitor
    for chain_index in [
        ChainIndex::ETH,
        ChainIndex::BSC,
        ChainIndex::POL,
        ChainIndex::ARB,
        ChainIndex::OP,
        ChainIndex::BASE,
        ChainIndex::SOL,
        ChainIndex::TRON,
    ] {
        let (threshold, target) = threshold_for_chain(chain_index);
        if threshold == rust_decimal::Decimal::ZERO {
            continue; // no gas concept for this chain
        }

        match gas_repo.get(chain_index).await {
            Ok(Some(pool)) if pool.enabled => {
                if pool.cold_wallet.is_empty() || pool.hot_wallet.is_empty() {
                    tracing::warn!(
                        chain_index = chain_index.as_i64(),
                        "gas pool enabled but cold_wallet/hot_wallet not configured"
                    );
                    continue;
                }

                let balance = pool.balance;

                if balance < threshold {
                    tracing::info!(
                        chain_index = chain_index.as_i64(),
                        balance = %balance,
                        threshold = %threshold,
                        "gas pool balance low — initiating refill"
                    );

                    // Calculate refill amount (already in native base units)
                    let refill_amount = target - balance;
                    if refill_amount <= rust_decimal::Decimal::ZERO {
                        continue;
                    }

                    // Execute refill transfer from cold wallet
                    match execute_refill(
                        state,
                        chain_index,
                        &pool.cold_wallet,
                        &pool.hot_wallet,
                        &refill_amount,
                    )
                    .await
                    {
                        Ok(tx_hash) => {
                            tracing::info!(
                                chain_index = chain_index.as_i64(),
                                from = %pool.cold_wallet,
                                to = %pool.hot_wallet,
                                amount = %refill_amount,
                                tx_hash = %tx_hash,
                                "gas pool refill submitted"
                            );

                            // Update pool balance in DB (optimistic; sync confirms later)
                            let new_balance = balance + refill_amount;
                            if let Err(e) = gas_repo.update_balance(chain_index, &new_balance).await
                            {
                                tracing::warn!(error = %e, "gas pool balance update failed");
                            }
                        }
                        Err(e) => {
                            tracing::error!(
                                chain_index = chain_index.as_i64(),
                                error = %e,
                                "gas pool refill transfer failed"
                            );
                        }
                    }
                }
            }
            Ok(_) => {}
            Err(e) => tracing::warn!(
                chain_index = chain_index.as_i64(),
                error = %e,
                "gas pool query failed"
            ),
        }
    }
    Ok(())
}

/// Execute a real on-chain transfer from cold wallet to hot wallet.
/// Cold wallet keys live in an external signing service, so each family signs
/// via its dedicated service and never holds private keys locally.
async fn execute_refill(
    state: &AppState,
    chain_index: ChainIndex,
    cold_wallet: &str,
    hot_wallet: &str,
    amount: &rust_decimal::Decimal,
) -> AppResult<String> {
    if cold_wallet.is_empty() || hot_wallet.is_empty() {
        return Err(AppError::InvalidArgument(
            "gas pool cold_wallet/hot_wallet not configured".into(),
        ));
    }
    let chain_handle = state.chains.get(chain_index)?;
    let family = ChainFamily::for_index(chain_index)
        .ok_or_else(|| AppError::ChainNotSupported(chain_index))?;

    match family {
        ChainFamily::Evm => {
            execute_evm_refill(state, chain_handle, cold_wallet, hot_wallet, amount).await
        }
        ChainFamily::Solana => {
            execute_solana_refill(state, chain_handle, cold_wallet, hot_wallet, amount).await
        }
        ChainFamily::Tron => {
            execute_tron_refill(state, chain_handle, cold_wallet, hot_wallet, amount).await
        }
        _ => Err(wallet_error::AppError::Unimplemented(
            "gas refill for chain family".into(),
        )),
    }
}

/// EVM refill: build a native token transfer and sign it either via the
/// EVM signing service (preferred) or the node (eth_sendTransaction).
async fn execute_evm_refill(
    state: &AppState,
    chain: &wallet_chain::ChainHandle,
    cold_wallet: &str,
    hot_wallet: &str,
    amount: &rust_decimal::Decimal,
) -> AppResult<String> {
    let rpc_url = chain.rpc_url()?;

    // 1. Get nonce for cold wallet (pending, so concurrent refills don't collide)
    let nonce = chain.nonce(&Address::new(cold_wallet.to_string())).await?;

    // 2. Fetch fee data: EIP-1559 when available, legacy gasPrice otherwise
    let fee_fields = evm_fee_fields(&state.http, &rpc_url).await?;

    // 3. Convert amount to wei (already an integer in base units — no rescaling)
    let amount_wei = decimal_to_base_units(amount)?;

    // 4. Build the transaction
    let mut tx = serde_json::Map::new();
    tx.insert("from".into(), serde_json::Value::String(cold_wallet.into()));
    tx.insert("to".into(), serde_json::Value::String(hot_wallet.into()));
    tx.insert("nonce".into(), serde_json::Value::String(format!("0x{nonce:x}")));
    tx.insert("gas".into(), serde_json::Value::String("0x5208".into()));
    tx.insert("value".into(), serde_json::Value::String(format!("0x{amount_wei:x}")));
    tx.insert("data".into(), serde_json::Value::String("0x".into()));
    if let Some(chain_id) = chain.evm_chain_id() {
        tx.insert(
            "chainId".into(),
            serde_json::Value::String(format!("0x{chain_id:x}")),
        );
    }
    match fee_fields {
        (Some(max_fee), Some(max_priority), _) => {
            tx.insert(
                "maxFeePerGas".into(),
                serde_json::Value::String(max_fee),
            );
            tx.insert(
                "maxPriorityFeePerGas".into(),
                serde_json::Value::String(max_priority),
            );
            tx.insert("type".into(), serde_json::Value::String("0x2".into()));
        }
        (_, _, legacy) => {
            tx.insert(
                "gasPrice".into(),
                serde_json::Value::String(legacy),
            );
        }
    }

    // 5. Sign via dedicated signing service (holds the cold wallet key).
    if let Ok(signing_url) = std::env::var("EVM_SIGNING_SERVICE_URL") {
        let resp = state
            .http
            .post(&signing_url)
            .json(&serde_json::Value::Object(tx))
            .send()
            .await
            .map_err(|e| AppError::Unavailable(format!("evm signing service: {e}")))?;
        if !resp.status().is_success() {
            return Err(AppError::Unavailable(format!(
                "evm signing service HTTP {}",
                resp.status()
            )));
        }
        let result: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Unavailable(format!("evm signing parse: {e}")))?;
        let tx_hash = result
            .pointer("/result")
            .and_then(|r| r.as_str())
            .unwrap_or_default()
            .to_string();
        if tx_hash.is_empty() {
            return Err(AppError::Unavailable(
                "evm signing service returned no tx hash".into(),
            ));
        }
        return Ok(tx_hash);
    }

    // 6. Fallback: node-managed signing. Requires the RPC node to hold and
    // unlock the cold wallet key (local/dev nodes only).
    tracing::warn!("EVM_SIGNING_SERVICE_URL not set; using node-managed eth_sendTransaction");
    let send_resp = rpc_call(
        &state.http,
        &rpc_url,
        "eth_sendTransaction",
        serde_json::json!([tx]),
    )
    .await?;

    let tx_hash = send_resp
        .pointer("/result")
        .and_then(|r| r.as_str())
        .unwrap_or_default()
        .to_string();
    if tx_hash.is_empty() {
        return Err(AppError::Unavailable(
            "eth_sendTransaction returned no tx hash".into(),
        ));
    }
    Ok(tx_hash)
}

/// Solana refill: transfer SOL from cold wallet to hot wallet. The cold wallet
/// key lives in the Solana signing service, which signs and broadcasts (or
/// returns a signed base64 transaction for local broadcast).
async fn execute_solana_refill(
    state: &AppState,
    chain: &wallet_chain::ChainHandle,
    cold_wallet: &str,
    hot_wallet: &str,
    amount: &rust_decimal::Decimal,
) -> AppResult<String> {
    let rpc_url = chain.rpc_url()?;
    let lamports = u64::try_from(decimal_to_base_units(amount)?).map_err(|_| {
        AppError::InvalidArgument(format!("solana refill amount too large: {amount}"))
    })?;

    // 1. Recent blockhash for the transaction
    let blockhash_resp = rpc_call(
        &state.http,
        &rpc_url,
        "getLatestBlockhash",
        serde_json::json!([{ "commitment": "finalized" }]),
    )
    .await?;
    let blockhash = blockhash_resp
        .pointer("/result/value/blockhash")
        .and_then(|b| b.as_str())
        .unwrap_or_default()
        .to_string();
    if blockhash.is_empty() {
        return Err(AppError::Unavailable(
            "getLatestBlockhash returned no blockhash".into(),
        ));
    }

    // 2. Build SystemProgram::transfer instruction
    let mut data = vec![2u8, 0, 0, 0]; // u32 LE tag: SystemProgram::Transfer
    data.extend_from_slice(&lamports.to_le_bytes());
    let instructions = serde_json::json!([{
        "programId": "11111111111111111111111111111111",
        "data": data,
        "accounts": [
            { "pubkey": cold_wallet, "isSigner": true, "isWritable": true },
            { "pubkey": hot_wallet, "isSigner": false, "isWritable": true }
        ]
    }]);

    // 3. Sign via the dedicated signing service.
    let signing_url = std::env::var("SOLANA_SIGNING_SERVICE_URL").map_err(|_| {
        AppError::Unimplemented(
            "solana gas refill requires SOLANA_SIGNING_SERVICE_URL to sign the transfer".into(),
        )
    })?;
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "sendTransaction",
        "params": [{
            "instructions": instructions,
            "blockhash": blockhash,
            "feePayer": cold_wallet
        }, { "encoding": "base64" }]
    });
    let resp = state
        .http
        .post(&signing_url)
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::Unavailable(format!("solana signing service: {e}")))?;
    if !resp.status().is_success() {
        return Err(AppError::Unavailable(format!(
            "solana signing service HTTP {}",
            resp.status()
        )));
    }
    let result: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| AppError::Unavailable(format!("solana signing parse: {e}")))?;

    // 3a. Service may return a signed base64 transaction for local broadcast.
    if let Some(signed) = result.get("signedTransaction").and_then(|s| s.as_str()) {
        let broadcast = rpc_call(
            &state.http,
            &rpc_url,
            "sendTransaction",
            serde_json::json!([signed, { "encoding": "base64" }]),
        )
        .await?;
        let tx_hash = broadcast
            .get("result")
            .and_then(|r| r.as_str())
            .unwrap_or_default()
            .to_string();
        if tx_hash.is_empty() {
            return Err(AppError::Unavailable(
                "solana sendTransaction returned no signature".into(),
            ));
        }
        return Ok(tx_hash);
    }

    // 3b. Otherwise the service broadcasts and returns the signature directly.
    let tx_hash = result
        .pointer("/result")
        .and_then(|r| r.as_str())
        .unwrap_or_default()
        .to_string();
    if tx_hash.is_empty() {
        return Err(AppError::Unavailable(
            "solana signing service returned no tx hash".into(),
        ));
    }
    Ok(tx_hash)
}

/// Tron refill: transfer TRX from cold wallet to hot wallet. The unsigned
/// transaction is signed by the TRON signing service before broadcast.
async fn execute_tron_refill(
    state: &AppState,
    chain: &wallet_chain::ChainHandle,
    cold_wallet: &str,
    hot_wallet: &str,
    amount: &rust_decimal::Decimal,
) -> AppResult<String> {
    let rpc_url = chain.rpc_url()?;
    let sun = u64::try_from(decimal_to_base_units(amount)?)
        .map_err(|_| AppError::InvalidArgument(format!("tron refill amount too large: {amount}")))?;

    // 1. Create (unsigned) transaction
    let create_resp = tron_post(
        &state.http,
        &rpc_url,
        "/wallet/createtransaction",
        serde_json::json!({
            "owner_address": cold_wallet,
            "to_address": hot_wallet,
            "amount": sun,
            "visible": true,
        }),
    )
    .await?;

    if let Some(code) = create_resp.get("code") {
        let message = create_resp
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("unknown");
        return Err(AppError::Unavailable(format!(
            "tron createtransaction rejected (code {code}): {message}"
        )));
    }

    let tx_id = create_resp
        .get("txID")
        .or_else(|| create_resp.get("txid"))
        .and_then(|t| t.as_str())
        .unwrap_or_default()
        .to_string();
    if tx_id.is_empty() {
        return Err(AppError::Unavailable(
            "tron createtransaction returned no txID".into(),
        ));
    }

    // 2. Sign via the dedicated signing service (holds the cold wallet key).
    let signing_url = std::env::var("TRON_SIGNING_SERVICE_URL").map_err(|_| {
        AppError::Unimplemented(
            "tron gas refill requires TRON_SIGNING_SERVICE_URL to sign the transfer".into(),
        )
    })?;
    let sign_resp = state
        .http
        .post(&signing_url)
        .json(&create_resp)
        .send()
        .await
        .map_err(|e| AppError::Unavailable(format!("tron signing service: {e}")))?;
    if !sign_resp.status().is_success() {
        return Err(AppError::Unavailable(format!(
            "tron signing service HTTP {}",
            sign_resp.status()
        )));
    }
    let signed: serde_json::Value = sign_resp
        .json()
        .await
        .map_err(|e| AppError::Unavailable(format!("tron signing parse: {e}")))?;

    // 3. Broadcast the signed transaction
    let broadcast_resp = tron_post(
        &state.http,
        &rpc_url,
        "/wallet/broadcasttransaction",
        signed,
    )
    .await?;

    let success = broadcast_resp
        .get("result")
        .and_then(|r| r.as_bool())
        .unwrap_or(false);
    if !success {
        let message = broadcast_resp
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("unknown");
        return Err(AppError::Unavailable(format!(
            "tron broadcast rejected: {message}"
        )));
    }

    Ok(tx_id)
}

// ── Helpers ──────────────────────────────────────────────────────────────

/// Convert a Decimal amount to an integer in native base units (wei / lamports /
/// sun). Rounding half-up any fractional remainder.
fn decimal_to_base_units(v: &rust_decimal::Decimal) -> AppResult<u128> {
    if v.is_sign_negative() {
        return Err(AppError::InvalidArgument(format!(
            "negative refill amount: {v}"
        )));
    }
    let scale = v.scale();
    let factor = 10u128
        .checked_pow(scale)
        .ok_or_else(|| AppError::InvalidArgument(format!("amount scale too large: {v}")))?;
    let mantissa = v.mantissa().unsigned_abs();
    let int_part = mantissa / factor;
    let rem = mantissa % factor;
    let rounded = if rem * 2 >= factor { int_part + 1 } else { int_part };
    Ok(rounded)
}

/// Fetch EIP-1559 fee fields via `eth_feeHistory`, falling back to legacy
/// `eth_gasPrice`. Returns `(max_fee_per_gas, max_priority_fee_per_gas, gas_price)`
/// as hex strings; exactly one of the pairs is populated.
async fn evm_fee_fields(
    http: &reqwest::Client,
    rpc_url: &str,
) -> AppResult<(Option<String>, Option<String>, String)> {
    let default_gas_price = "0x3B9ACA00".to_string(); // 1 gwei

    if let Ok(fee) = rpc_call(http, rpc_url, "eth_feeHistory", serde_json::json!([3, "latest", [25, 50, 75]])).await {
        if let Some(base_hex) = fee
            .pointer("/result/baseFeePerGas/0")
            .and_then(|b| b.as_str())
        {
            if let Ok(base) = u128::from_str_radix(base_hex.trim_start_matches("0x"), 16) {
                let max_priority = 1_500_000_000u128; // 1.5 gwei
                let max_fee = base * 2 + max_priority;
                return Ok((
                    Some(format!("0x{max_fee:x}")),
                    Some(format!("0x{max_priority:x}")),
                    default_gas_price,
                ));
            }
        }
    }

    let gas_price_resp = rpc_call(http, rpc_url, "eth_gasPrice", serde_json::json!([])).await?;
    let gas_price = gas_price_resp
        .pointer("/result")
        .and_then(|r| r.as_str())
        .unwrap_or(&default_gas_price)
        .to_string();
    Ok((None, None, gas_price))
}

async fn rpc_call(
    http: &reqwest::Client,
    url: &str,
    method: &str,
    params: serde_json::Value,
) -> AppResult<serde_json::Value> {
    let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": method, "params": params });
    let resp = http
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::Unavailable(format!("rpc: {e}")))?;
    let v: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| AppError::Unavailable(format!("rpc parse: {e}")))?;
    if let Some(err) = v.get("error").filter(|e| !e.is_null()) {
        return Err(AppError::Unavailable(format!("rpc {method} error: {err}")));
    }
    Ok(v)
}

async fn tron_post(
    http: &reqwest::Client,
    base_url: &str,
    path: &str,
    body: serde_json::Value,
) -> AppResult<serde_json::Value> {
    let url = format!("{}{}", base_url.trim_end_matches('/'), path);
    let resp = http
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::Unavailable(format!("tron: {e}")))?;
    if !resp.status().is_success() {
        return Err(AppError::Unavailable(format!(
            "tron {path} HTTP {}",
            resp.status()
        )));
    }
    let v: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| AppError::Unavailable(format!("tron parse: {e}")))?;
    if let Some(err) = v.get("Error").or_else(|| v.get("error")) {
        return Err(AppError::Unavailable(format!("tron {path} error: {err}")));
    }
    Ok(v)
}
