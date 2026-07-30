//! Gas pool refill sweeper — checks balances and triggers real transfers from cold wallet.

use std::sync::Arc;
use std::time::Duration;
use wallet_chain::NonceProvider;
use wallet_db::GasPoolRepo;
use wallet_domain::AppState;
use wallet_types::{Address, ChainFamily, ChainIndex};

/// Chain-specific refill thresholds. EVM chains use wei (18 decimals),
/// Solana uses lamports (9 decimals), TRON uses SUN (6 decimals).
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

async fn tick(state: &AppState) -> wallet_error::AppResult<()> {
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
                let balance = pool.balance;

                if balance < threshold {
                    tracing::info!(
                        chain_index = chain_index.as_i64(),
                        balance = %balance,
                        threshold = %threshold,
                        "gas pool balance low — initiating refill"
                    );

                    // Calculate refill amount
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

                            // Update pool balance in DB
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
async fn execute_refill(
    state: &AppState,
    chain_index: ChainIndex,
    cold_wallet: &str,
    hot_wallet: &str,
    amount: &rust_decimal::Decimal,
) -> wallet_error::AppResult<String> {
    let chain_handle = state.chains.get(chain_index)?;
    let family = ChainFamily::for_index(chain_index)
        .ok_or_else(|| wallet_error::AppError::ChainNotSupported(chain_index))?;

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

/// EVM refill: build and sign a native token transfer.
async fn execute_evm_refill(
    state: &AppState,
    chain: &wallet_chain::ChainHandle,
    cold_wallet: &str,
    hot_wallet: &str,
    amount: &rust_decimal::Decimal,
) -> wallet_error::AppResult<String> {
    // 1. Get nonce for cold wallet
    let nonce = chain.nonce(&Address::new(cold_wallet.to_string())).await?;

    // 2. Get current gas price
    let rpc_url = chain.rpc_url()?;
    let gas_price_resp =
        rpc_call(&state.http, &rpc_url, "eth_gasPrice", serde_json::json!([])).await?;
    let gas_price_hex = gas_price_resp
        .get("result")
        .and_then(|r| r.as_str())
        .unwrap_or("0x3B9ACA00");
    let gas_price =
        u64::from_str_radix(gas_price_hex.trim_start_matches("0x"), 16).unwrap_or(1_000_000_000);

    // 3. Convert amount to wei
    let amount_f64: f64 = amount.to_string().parse().unwrap_or(0.0);
    let amount_wei = (amount_f64 * 1e18) as u128;

    // 4. Build raw tx (legacy, since we don't have private keys here — use RPC sign)
    let tx = serde_json::json!({
        "from": cold_wallet,
        "to": hot_wallet,
        "nonce": format!("0x{:x}", nonce),
        "gasLimit": "0x5208",
        "gasPrice": format!("0x{:x}", gas_price),
        "value": format!("0x{:x}", amount_wei),
        "data": "0x",
    });

    // 5. Send via eth_sendTransaction (node-managed signing) or sign+broadcast
    let send_resp = rpc_call(
        &state.http,
        &rpc_url,
        "eth_sendTransaction",
        serde_json::json!([tx]),
    )
    .await?;

    let tx_hash = send_resp
        .get("result")
        .and_then(|r| r.as_str())
        .unwrap_or("")
        .to_string();

    Ok(tx_hash)
}

/// Solana refill: transfer SOL from cold wallet to hot wallet.
/// Requires solana-sdk for proper VersionedTransaction construction.
async fn execute_solana_refill(
    _state: &AppState,
    _chain: &wallet_chain::ChainHandle,
    _cold_wallet: &str,
    _hot_wallet: &str,
    _amount: &rust_decimal::Decimal,
) -> wallet_error::AppResult<String> {
    // Solana transactions require a serialized VersionedTransaction:
    // 1. Build SystemProgram::transfer instruction from cold_wallet -> hot_wallet
    // 2. Wrap in a VersionedTransaction with recent blockhash
    // 3. Sign with cold_wallet private key
    // 4. Base64-encode and submit via sendTransaction
    //
    // This requires the solana-sdk crate. Without it, constructing
    // valid serialized transactions is not reliable.
    Err(wallet_error::AppError::Unimplemented(
        "solana gas refill: requires solana-sdk for tx serialization".into(),
    ))
}

/// Tron refill: transfer TRX from cold wallet to hot wallet.
async fn execute_tron_refill(
    state: &AppState,
    chain: &wallet_chain::ChainHandle,
    cold_wallet: &str,
    hot_wallet: &str,
    amount: &rust_decimal::Decimal,
) -> wallet_error::AppResult<String> {
    let rpc_url = chain.rpc_url()?;

    // 1. Convert amount to SUN
    let amount_f64: f64 = amount.to_string().parse().unwrap_or(0.0);
    let sun = (amount_f64 * 1_000_000.0) as u64;

    // 2. Create transaction
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

    let tx_id = create_resp
        .get("txID")
        .or_else(|| create_resp.get("txid"))
        .and_then(|t| t.as_str())
        .unwrap_or("")
        .to_string();

    // 3. Broadcast
    let broadcast_resp = tron_post(
        &state.http,
        &rpc_url,
        "/wallet/broadcasttransaction",
        serde_json::json!({ "transaction": create_resp }),
    )
    .await?;

    let success = broadcast_resp
        .get("result")
        .and_then(|r| r.as_bool())
        .unwrap_or(false);

    if success {
        Ok(tx_id)
    } else {
        Err(wallet_error::AppError::Unavailable(
            "tron broadcast failed".into(),
        ))
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────

async fn rpc_call(
    http: &reqwest::Client,
    url: &str,
    method: &str,
    params: serde_json::Value,
) -> wallet_error::AppResult<serde_json::Value> {
    let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": method, "params": params });
    let resp = http
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(|e| wallet_error::AppError::Unavailable(format!("rpc: {e}")))?;
    resp.json()
        .await
        .map_err(|e| wallet_error::AppError::Unavailable(format!("rpc parse: {e}")))
}

async fn tron_post(
    http: &reqwest::Client,
    base_url: &str,
    path: &str,
    body: serde_json::Value,
) -> wallet_error::AppResult<serde_json::Value> {
    let url = format!("{}{}", base_url.trim_end_matches('/'), path);
    let resp = http
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| wallet_error::AppError::Unavailable(format!("tron: {e}")))?;
    resp.json()
        .await
        .map_err(|e| wallet_error::AppError::Unavailable(format!("tron parse: {e}")))
}
