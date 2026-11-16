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
        let urls = cfg.candidate_urls();
        if urls.is_empty() {
            return Err(AppError::InvalidArgument(
                "redis url or addrs must be configured".into(),
            ));
        }

        let mut last_err = None;
        for url in urls {
            let masked = mask_redis_url(&url);
            let config = match Config::from_url(&url) {
                Ok(c) => c,
                Err(e) => {
                    last_err = Some(AppError::Unavailable(format!(
                        "redis url parse ({masked}): {e}"
                    )));
                    continue;
                }
            };
            let client = match Builder::from_config(config).build() {
                Ok(c) => c,
                Err(e) => {
                    last_err = Some(AppError::Unavailable(format!(
                        "redis client build ({masked}): {e}"
                    )));
                    continue;
                }
            };
            match client.init().await {
                Ok(_) => return Ok(Self { client }),
                Err(e) => {
                    tracing::warn!(target = %masked, error = %e, "redis seed connect failed, trying next");
                    last_err = Some(AppError::Unavailable(format!(
                        "redis connect ({masked}): {e}"
                    )));
                }
            }
        }

        Err(last_err.unwrap_or_else(|| AppError::Unavailable("redis connect failed".into())))
    }

    pub fn client(&self) -> &Client {
        &self.client
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

fn mask_redis_url(url: &str) -> String {
    if let Some((scheme, rest)) = url.split_once("://") {
        if let Some((_, host)) = rest.rsplit_once('@') {
            return format!("{scheme}://***@{host}");
        }
    }
    url.to_string()
}
