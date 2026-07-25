pub mod cms;
pub mod dapp;
pub mod gaspool;
pub mod market;
pub mod network;
pub mod rent;
pub mod swap;
pub mod token;
pub mod transaction;
pub mod user;

use std::sync::Arc;
use wallet_chain::ChainRegistry;
use wallet_db::{clickhouse::ClickHouseDb, Db};
use wallet_events::EventBus;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub chains: Arc<ChainRegistry>,
    pub events: Arc<dyn EventBus>,
    pub clickhouse: Option<ClickHouseDb>,
    pub http: reqwest::Client,
}

impl AppState {
    pub fn new(
        db: Db,
        chains: ChainRegistry,
        events: Arc<dyn EventBus>,
        clickhouse: Option<ClickHouseDb>,
    ) -> Self {
        Self {
            db,
            chains: Arc::new(chains),
            events,
            clickhouse,
            http: reqwest::Client::new(),
        }
    }
}
