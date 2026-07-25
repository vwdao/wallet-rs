use std::sync::Arc;
use std::time::Duration;
use wallet_chain::{BlockSource, ChainHandle};
use wallet_db::{SyncCursorRepo, TransactionRepo, Db};
use wallet_error::AppResult;
use wallet_events::EventBus;
use wallet_types::ChainIndex;

use crate::parser::parse_block;
use crate::reporter::report_txs;

pub struct SyncRuntime {
    pub db: Db,
    pub chain_index: ChainIndex,
    pub chain: ChainHandle,
    pub events: Arc<dyn EventBus>,
    pub poll_interval_ms: u64,
    pub confirmations: u64,
}

pub async fn run(rt: SyncRuntime) -> AppResult<()> {
    tracing::info!(chain = %rt.chain_index, "wallet-sync started");
    loop {
        if let Err(e) = tick(&rt).await {
            tracing::warn!(error = %e, "sync tick failed");
        }
        tokio::time::sleep(Duration::from_millis(rt.poll_interval_ms)).await;
    }
}

async fn tick(rt: &SyncRuntime) -> AppResult<()> {
    let tip = rt.chain.tip().await?;
    let safe = tip.saturating_sub(rt.confirmations);
    let mut cursor = SyncCursorRepo::new(&rt.db).get(rt.chain_index).await?;
    if cursor == 0 {
        cursor = safe.saturating_sub(1);
    }
    while cursor < safe {
        let next = cursor + 1;
        let raw_txs = rt.chain.fetch_block_txs(next).await?;
        let normalized = parse_block(rt.chain_index, next, raw_txs);
        for tx in &normalized {
            TransactionRepo::new(&rt.db)
                .insert_normalized(rt.chain_index, tx)
                .await?;
        }
        report_txs(rt.events.as_ref(), rt.chain_index, &normalized).await?;
        SyncCursorRepo::new(&rt.db)
            .set(rt.chain_index, next)
            .await?;
        cursor = next;
        tracing::info!(height = next, txs = normalized.len(), "synced block");
    }
    Ok(())
}
