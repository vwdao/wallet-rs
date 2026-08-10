use fred::prelude::*;
use wallet_config::RedisConfig;
use wallet_error::{AppError, AppResult};
use wallet_types::ChainIndex;

/// Redis-backed store.
///
/// Per-chain sync cursors live here instead of Postgres: they are written
/// on every catch-up sweep and a Redis key is a much cheaper fit for that
/// workload than a row in `sync_cursors`.
#[derive(Clone)]
pub struct RedisStore {
    client: Client,
}

impl RedisStore {
    pub async fn connect(cfg: &RedisConfig) -> AppResult<Self> {
        let config = Config::from_url(&cfg.url)
            .map_err(|e| AppError::Unavailable(format!("redis url parse: {e}")))?;
        let client = Builder::from_config(config)
            .build()
            .map_err(|e| AppError::Unavailable(format!("redis client build: {e}")))?;
        client
            .init()
            .await
            .map_err(|e| AppError::Unavailable(format!("redis connect: {e}")))?;
        Ok(Self { client })
    }

    fn cursor_key(chain_index: ChainIndex) -> String {
        format!("sync:cursor:{}", chain_index.as_i64())
    }

    /// Highest persisted block height for `chain_index`, or 0 when no cursor
    /// has been written yet.
    pub async fn cursor_get(&self, chain_index: ChainIndex) -> AppResult<u64> {
        let height = self
            .client
            .get::<Option<i64>, _>(Self::cursor_key(chain_index))
            .await
            .map_err(|e| AppError::internal(format!("redis get cursor: {e}")))?;
        Ok(height.unwrap_or(0).max(0) as u64)
    }

    pub async fn cursor_set(&self, chain_index: ChainIndex, height: u64) -> AppResult<()> {
        self.client
            .set::<(), _, _>(Self::cursor_key(chain_index), height as i64, None, None, false)
            .await
            .map_err(|e| AppError::internal(format!("redis set cursor: {e}")))?;
        Ok(())
    }
}
