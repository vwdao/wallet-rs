use clickhouse::Client;
use serde::{Deserialize, Serialize};
use wallet_config::ClickHouseConfig;
use wallet_error::{AppError, AppResult};

#[derive(Clone)]
pub struct ClickHouseDb {
    client: Client,
}

impl ClickHouseDb {
    pub fn connect(cfg: &ClickHouseConfig) -> Self {
        let mut client = Client::default()
            .with_url(&cfg.url)
            .with_database(&cfg.database);
        if let Some(user) = &cfg.user {
            client = client.with_user(user);
        }
        if let Some(password) = &cfg.password {
            client = client.with_password(password);
        }
        Self { client }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub async fn ensure_schema(&self) -> AppResult<()> {
        self.client
            .query(
                r#"
                CREATE TABLE IF NOT EXISTS kline_1m (
                    symbol String,
                    open_time Int64,
                    open String,
                    high String,
                    low String,
                    close String,
                    volume String
                ) ENGINE = MergeTree ORDER BY (symbol, open_time)
                "#,
            )
            .execute()
            .await
            .map_err(|e| AppError::Unavailable(format!("clickhouse: {e}")))?;

        self.client
            .query(
                r#"
                CREATE TABLE IF NOT EXISTS dex_trade (
                    chain_index Int64,
                    tx_hash String,
                    ts Int64,
                    payload String
                ) ENGINE = MergeTree ORDER BY (chain_index, ts)
                "#,
            )
            .execute()
            .await
            .map_err(|e| AppError::Unavailable(format!("clickhouse: {e}")))?;

        self.client
            .query(
                r#"
                CREATE TABLE IF NOT EXISTS price_tick (
                    symbol String,
                    price Float64,
                    ts Int64
                ) ENGINE = MergeTree ORDER BY (symbol, ts)
                "#,
            )
            .execute()
            .await
            .map_err(|e| AppError::Unavailable(format!("clickhouse: {e}")))?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, clickhouse::Row)]
pub struct KlineRow {
    pub symbol: String,
    pub open_time: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
}
