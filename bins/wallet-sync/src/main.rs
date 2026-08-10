mod parser;
mod reindex;
mod reporter;
mod syncer;

use clap::Parser;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use wallet_chain::build_registry;
use wallet_config::{load_yaml, SyncConfig};
use wallet_db::Db;
use wallet_events::{MemoryEventBus, NatsEventBus};
use wallet_types::ChainIndex;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "configs/wallet-sync.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();
    let args = Args::parse();
    let cfg: SyncConfig = load_yaml(&args.config)?;
    let db = Db::connect(&cfg.database).await?;
    if let Err(e) = db.migrate().await {
        tracing::warn!("schema push skipped: {e}");
    }
    let registry = build_registry(std::slice::from_ref(&cfg.chain))?;
    let events = match NatsEventBus::connect(&cfg.nats.url).await {
        Ok(b) => b as Arc<dyn wallet_events::EventBus>,
        Err(e) => {
            tracing::warn!(error = %e, "NATS unavailable, falling back to in-memory event bus");
            MemoryEventBus::new(256)
        }
    };
    let chain_index = ChainIndex(cfg.chain_index);
    let handle = registry.get(chain_index)?.clone();
    let handle_for_reindex = handle.clone();
    let events_for_reindex = events.clone();
    let db_for_reindex = db.clone();

    tokio::select! {
        r = syncer::run(syncer::SyncRuntime {
            db,
            chain_index,
            chain: handle,
            events,
            poll_interval_ms: cfg.poll_interval_ms,
            confirmations: cfg.chain.confirmations,
            start_height: cfg.start_height,
            block_fetch_concurrency: cfg.block_fetch_concurrency,
        }) => { r?; }
        r = reindex::start(db_for_reindex, chain_index, Arc::new(handle_for_reindex), events_for_reindex) => { r?; }
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("shutting down wallet-sync...");
        }
    }
    Ok(())
}
