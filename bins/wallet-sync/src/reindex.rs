//! Reindex consumer — listens for admin reindex requests and replays blocks.

use std::sync::Arc;
use tracing::{error, info};
use wallet_chain::{BlockSource, ChainHandle};
use wallet_db::{Db, SyncCursorRepo};
use wallet_error::AppResult;
use wallet_events::EventBus;
use wallet_types::ChainIndex;

use crate::parser::parse_block;
use crate::reporter::report_txs;

pub async fn start(
    db: Db,
    chain_index: ChainIndex,
    chain: Arc<ChainHandle>,
    bus: Arc<dyn EventBus>,
) -> AppResult<()> {
    let mut sub = bus
        .subscribe("wallet.sync.reindex", "wallet-sync-reindex")
        .await?;
    info!("reindex consumer started");
    loop {
        match sub.next().await {
            Ok(Some(env)) => {
                let v = &env.payload;
                let target_chain = v.get("chain_index").and_then(|c| c.as_i64()).unwrap_or(0);
                let from_block = v.get("from_block").and_then(|b| b.as_u64()).unwrap_or(0);
                let to_block = v.get("to_block").and_then(|b| b.as_u64()).unwrap_or(0);

                if target_chain != chain_index.as_i64() {
                    info!(
                        target = target_chain,
                        current = chain_index.as_i64(),
                        "reindex: ignoring for different chain"
                    );
                    let _ = sub.ack_last().await;
                    continue;
                }

                if from_block > to_block {
                    error!(
                        from = from_block,
                        to = to_block,
                        "reindex: invalid block range"
                    );
                    let _ = sub.ack_last().await;
                    continue;
                }

                info!(
                    from = from_block,
                    to = to_block,
                    "reindex: starting block replay"
                );

                let mut reindexed = 0u64;
                for height in from_block..=to_block {
                    match chain.fetch_block_txs(height).await {
                        Ok(raw_txs) => {
                            let normalized = parse_block(chain_index, height, raw_txs);
                            let tx_count = normalized.len();
                            for tx in &normalized {
                                if let Err(e) = wallet_db::TransactionRepo::new(&db)
                                    .insert_normalized(chain_index, tx)
                                    .await
                                {
                                    error!(height, error = %e, "reindex: insert failed");
                                }
                            }
                            if let Err(e) = report_txs(bus.as_ref(), chain_index, &normalized).await
                            {
                                error!(height, error = %e, "reindex: report failed");
                            }
                            reindexed += 1;
                            if reindexed.is_multiple_of(100) {
                                info!(height, txs = tx_count, "reindex progress");
                            }
                        }
                        Err(e) => {
                            error!(height, error = %e, "reindex: fetch block failed");
                        }
                    }
                }

                // Update sync cursor to the reindexed range
                if let Err(e) = SyncCursorRepo::new(&db).set(chain_index, to_block).await {
                    error!(error = %e, "reindex: cursor update failed");
                }

                info!(
                    blocks = reindexed,
                    from = from_block,
                    to = to_block,
                    "reindex: completed"
                );

                if let Err(e) = sub.ack_last().await {
                    error!("reindex: ack error: {e}");
                }
            }
            Ok(None) => {
                info!("reindex consumer: stream ended");
                break;
            }
            Err(e) => {
                error!("reindex consume error: {e}");
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    }
    Ok(())
}
