mod consumer;
mod jobs;

use clap::Parser;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use wallet_chain::build_registry;
use wallet_config::{load_yaml, JobsConfig};
use wallet_db::clickhouse::ClickHouseDb;
use wallet_db::Db;
use wallet_domain::AppState;
use wallet_events::{MemoryEventBus, NatsEventBus};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "configs/wallet-jobs.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();
    let args = Args::parse();
    let cfg: JobsConfig = load_yaml(&args.config)?;
    let db = Db::connect(&cfg.database).await?;
    if let Err(e) = db.migrate().await {
        tracing::warn!("schema push skipped: {e}");
    }
    let chains = build_registry(&cfg.chains)?;
    let events = match NatsEventBus::connect(&cfg.nats.url).await {
        Ok(b) => b as Arc<dyn wallet_events::EventBus>,
        Err(_) => MemoryEventBus::new(256),
    };
    let clickhouse = cfg.clickhouse.as_ref().map(ClickHouseDb::connect);
    if let Some(ch) = &clickhouse {
        if let Err(e) = ch.ensure_schema().await {
            tracing::warn!(error = %e, "clickhouse schema push skipped");
        }
    }
    let state = Arc::new(AppState::new(db.clone(), chains, events.clone(), clickhouse));

    let shutdown = async {
        tokio::signal::ctrl_c().await.ok();
        tracing::info!("shutting down wallet-jobs...");
    };

    let price = tokio::spawn(jobs::price_sync::run(state.clone()));
    let token = tokio::spawn(jobs::token_ensure::run(state.clone()));
    let gas = tokio::spawn(jobs::gas_refill::run(state.clone()));

    let tx_cons = tokio::spawn(consumer::tx_event::start(db.clone(), events.clone()));
    let dex_cons = tokio::spawn(consumer::dex_event::start(db.clone(), events));

    tokio::select! {
        _ = shutdown => {
            tracing::info!("wallet-jobs stopping");
        }
        _ = price => {}
        _ = token => {}
        _ = gas => {}
        _ = tx_cons => {}
        _ = dex_cons => {}
    }
    Ok(())
}
