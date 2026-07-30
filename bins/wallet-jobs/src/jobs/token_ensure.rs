use std::sync::Arc;
use std::time::Duration;
use wallet_db::{TokenRepo, TokenRow};
use wallet_domain::AppState;
use wallet_types::ChainIndex;

const TOKENS_PER_CHAIN: &[(&str, &str, &str, i32)] = &[
    // (chain_index, address, symbol, decimals) — popular tokens
    (
        "60",
        "0xdac17f958d2ee523a2206206994597c13d831ec7",
        "USDT",
        6,
    ),
    (
        "60",
        "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
        "USDC",
        6,
    ),
    (
        "60",
        "0x6b175474e89094c44da98b954eedeac495271d0f",
        "DAI",
        18,
    ),
    (
        "60",
        "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        "WETH",
        18,
    ),
    (
        "20000714",
        "0x55d398326f99059ff775485246999027b3197955",
        "USDT",
        18,
    ),
    (
        "20000714",
        "0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d",
        "USDC",
        18,
    ),
    (
        "137",
        "0xc2132d05d31c914a87c6611c10748aeb04b58e8f",
        "USDT",
        6,
    ),
    (
        "137",
        "0x2791bca1f2de4661ed88a30c99a7a9449aa84174",
        "USDC",
        6,
    ),
    (
        "8453",
        "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913",
        "USDC",
        6,
    ),
];

pub async fn run(state: Arc<AppState>) -> anyhow::Result<()> {
    loop {
        if let Err(e) = tick(state.as_ref()).await {
            tracing::warn!(error = %e, "token_ensure tick failed");
        }
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

async fn tick(state: &AppState) -> wallet_error::AppResult<()> {
    let repo = TokenRepo::new(&state.db);
    for &(chain_idx_str, address, symbol, decimals) in TOKENS_PER_CHAIN {
        let chain_index: i64 = chain_idx_str.parse().unwrap_or(0);
        let chain_index = ChainIndex(chain_index);
        match repo.get(chain_index, address).await {
            Ok(None) => {
                let row = TokenRow {
                    id: uuid::Uuid::new_v4(),
                    chain_index: chain_index.as_i64(),
                    address: address.to_string(),
                    symbol: symbol.to_string(),
                    name: symbol.to_string(),
                    decimals,
                    logo_url: None,
                    created_at: jiff::Timestamp::now(),
                };
                match repo.upsert(&row).await {
                    Ok(_) => {
                        tracing::info!(symbol, chain_index = chain_index.as_i64(), "token ensured")
                    }
                    Err(e) => tracing::warn!(error = %e, symbol, "token ensure upsert failed"),
                }
            }
            Ok(Some(_)) => {}
            Err(e) => tracing::warn!(error = %e, symbol, "token ensure query failed"),
        }
    }
    Ok(())
}
