use std::sync::Arc;
use std::time::Duration;
use wallet_chain::{BlockSource, ChainHandle};
use wallet_db::{Db, RedisStore, SyncSettingRepo, TransactionRepo};
use wallet_error::AppResult;
use wallet_events::EventBus;
use wallet_types::{ChainIndex, NormalizedTx};

use crate::parser::parse_block;
use crate::reporter::report_txs;

pub struct SyncRuntime {
    pub db: Db,
    pub redis: RedisStore,
    pub chain_index: ChainIndex,
    pub chain: ChainHandle,
    pub events: Arc<dyn EventBus>,
    pub poll_interval_ms: u64,
    pub confirmations: u64,
    /// First block to backfill from when no cursor exists yet.
    pub start_height: Option<u64>,
    /// How many blocks to fetch from the RPC in parallel per tick.
    ///
    /// High-BPS chains (Arbitrum, Base, OP, Polygon) can produce blocks
    /// faster than a sequential `fetch_block_txs` round trip can keep up
    /// with; fetching `block_fetch_concurrency` heights at once and then
    /// processing them in order keeps the cursor monotonic while letting
    /// the RPC latency overlap with later block fetches.
    pub block_fetch_concurrency: usize,
}

pub async fn run(rt: SyncRuntime) -> AppResult<()> {
    tracing::info!(chain = %rt.chain_index, "wallet-sync started");
    let mut last_poll_ms = rt.poll_interval_ms;
    loop {
        // `tick` already loads SyncSetting; reuse it for the sleep so we
        // don't issue a second SELECT just to learn the poll interval.
        let (next_poll, _) = match tick(&rt, last_poll_ms).await {
            Ok(t) => t,
            Err(e) => {
                tracing::warn!(error = %e, "sync tick failed");
                (rt.poll_interval_ms, None)
            }
        };
        last_poll_ms = next_poll;
        tokio::time::sleep(Duration::from_millis(next_poll)).await;
    }
}

/// Run one catch-up sweep: process every block between the cursor and the
/// safe tip, then advance the cursor in a single write.
///
/// Returns the `poll_interval_ms` to use for the next sleep, plus the height
/// of the last block that was persisted (for logging/testing).
async fn tick(
    rt: &SyncRuntime,
    default_poll_ms: u64,
) -> AppResult<(u64, Option<u64>)> {
    let setting = SyncSettingRepo::new(&rt.db).get(rt.chain_index).await?;
    let poll_ms = setting
        .as_ref()
        .map(|s| s.poll_interval_ms as u64)
        .unwrap_or(default_poll_ms);
    let confirmations = setting
        .as_ref()
        .map(|s| s.confirmations as u64)
        .unwrap_or(rt.confirmations);

    if let Some(s) = &setting {
        if !s.enabled {
            tracing::debug!(chain = %rt.chain_index, "sync disabled via db settings");
            return Ok((poll_ms, None));
        }
    }

    let tip = rt.chain.tip().await?;
    // A real chain never reports height 0; treat it as a transient RPC/gateway
    // failure and skip the tick rather than rewinding the cursor.
    if tip == 0 {
        tracing::warn!(chain = %rt.chain_index, "rpc reported tip height 0, skipping tick");
        return Ok((poll_ms, None));
    }
    let safe = tip.saturating_sub(confirmations);
    let mut cursor = rt.redis.cursor_get(rt.chain_index).await?;

    if cursor == 0 {
        cursor = match rt.start_height {
            Some(start) => start.saturating_sub(1),
            None => safe.saturating_sub(1),
        };
        tracing::info!(chain = %rt.chain_index, height = cursor + 1, "initializing sync cursor");
    }

    if cursor > safe {
        tracing::warn!(
            chain = %rt.chain_index,
            cursor,
            safe,
            "chain reorged, rewinding cursor"
        );
        cursor = safe;
        rt.redis.cursor_set(rt.chain_index, safe).await?;
    }

    let repo = TransactionRepo::new(&rt.db);
    let mut synced = 0u64;
    let mut last_synced: Option<u64> = None;
    let concurrency = rt.block_fetch_concurrency.max(1);

    while cursor < safe {
        // Fetch a window of up to `concurrency` blocks in parallel. We
        // process the results back in height order so the cursor advances
        // monotonically; if any block in the window fails to fetch, we stop
        // the batch and let the next tick retry from the failing height.
        // Empty tx lists are *not* treated as failures: high-BPS chains
        // (Arbitrum, Base, OP, Polygon) emit legitimate empty blocks, and
        // genuine gateway flakes already surface as `Err` from the chain
        // implementation (e.g. a missing block).
        let batch_end = std::cmp::min(safe, cursor + concurrency as u64);
        let heights: Vec<u64> = (cursor + 1..=batch_end).collect();
        let fetches = fetch_window(&rt.chain, &heights).await;

        let mut advanced_this_batch = 0u64;
        for (height, res) in fetches {
            let raw_txs = match res {
                Ok(r) => r,
                Err(e) => {
                    // Don't advance the cursor past this height; the next
                    // tick will re-fetch. The unsent fetches in this
                    // window are discarded — the DB upsert is idempotent
                    // so re-fetching the same heights is safe.
                    tracing::warn!(chain = %rt.chain_index, height, error = %e, "fetch block failed, holding cursor for retry");
                    break;
                }
            };
            let normalized = parse_block(rt.chain_index, height, raw_txs);
            // Persist and publish concurrently: the DB write is bounded by
            // the batched UPSERT, the event publish is bounded by
            // `REPORT_INFLIGHT`, and the two are independent.
            tokio::try_join!(
                repo.insert_normalized_batch(rt.chain_index, &normalized),
                report_txs(rt.events.as_ref(), rt.chain_index, &normalized),
            )?;
            cursor = height;
            synced += 1;
            advanced_this_batch += 1;
            last_synced = Some(height);
            tracing::info!(chain = %rt.chain_index, height, txs = normalized.len(), "synced block");
        }

        if advanced_this_batch == 0 {
            // The very first block in the window failed; no progress this
            // tick. Sleep and let the next tick try again.
            break;
        }
    }

    if synced > 0 {
        // Single cursor write at the end of a catch-up sweep instead of one
        // per block; the worst case (single-block tick) still does one write.
        rt.redis.cursor_set(rt.chain_index, cursor).await?;
        tracing::info!(chain = %rt.chain_index, tip, safe, synced, "sync catch-up complete");
    }
    Ok((poll_ms, last_synced))
}

/// Fan out `fetch_block_txs` over a window of heights in parallel.
///
/// `ChainHandle` is an `enum` of `Arc<…Chain>` so cloning it for each
/// async block is cheap and gives each future an independent `&dyn
/// BlockSource` to poll — required because `join_all` polls all
/// futures from the same task and a borrowed `&rt.chain` would not
/// be available everywhere it's needed.
async fn fetch_window(
    chain: &ChainHandle,
    heights: &[u64],
) -> Vec<(u64, AppResult<Vec<NormalizedTx>>)> {
    let handles: Vec<_> = heights
        .iter()
        .map(|&h| {
            let chain = chain.clone();
            async move {
                let res: AppResult<Vec<NormalizedTx>> = chain.fetch_block_txs(h).await;
                (h, res)
            }
        })
        .collect();
    futures::future::join_all(handles).await
}
