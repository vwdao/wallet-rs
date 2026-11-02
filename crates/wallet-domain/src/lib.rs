pub mod cms;
pub mod dapp;
pub mod gaspool;
pub mod market;
pub mod metrics;
pub mod network;
pub mod rent;
pub mod swap;
pub mod token;
pub mod transaction;
pub mod user;

use std::sync::Arc;
use std::time::Duration;
use wallet_chain::ChainRegistry;
use wallet_db::{clickhouse::ClickHouseDb, Db};
use wallet_events::EventBus;

pub use metrics::Metrics;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub chains: Arc<ChainRegistry>,
    pub events: Arc<dyn EventBus>,
    pub clickhouse: Option<ClickHouseDb>,
    pub http: reqwest::Client,
    pub metrics: Metrics,
}

impl AppState {
    pub fn new(
        db: Db,
        chains: ChainRegistry,
        events: Arc<dyn EventBus>,
        clickhouse: Option<ClickHouseDb>,
    ) -> Self {
        let http = reqwest::Client::builder()
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(32)
            .tcp_keepalive(Duration::from_secs(60))
            .tcp_nodelay(true)
            .connect_timeout(Duration::from_secs(8))
            .timeout(Duration::from_secs(25))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            db,
            chains: Arc::new(chains),
            events,
            clickhouse,
            http,
            metrics: Metrics::new(),
        }
    }

    pub fn with_http_client(mut self, http: reqwest::Client) -> Self {
        self.http = http;
        self
    }
}
