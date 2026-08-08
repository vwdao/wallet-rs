use dashmap::DashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::watch;
use wallet_db::GatewaySettings;
use wallet_error::AppResult;

#[derive(Debug, Clone)]
pub struct GatewayConfig {
    pub health_check_interval_ms: u64,
    pub failure_threshold: u32,
    pub rpc_timeout_secs: u64,
    pub max_retries: u32,
    pub max_block_lag: i64,
    /// Solana slot lag tolerance. Slots are ~400ms; tens of slots of skew across
    /// providers is common, so this is intentionally higher than `max_block_lag`.
    pub solana_max_block_lag: i64,
    pub global_rate_limit_per_min: usize,
    pub stats_batch_interval_ms: u64,
    pub log_requests: bool,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            health_check_interval_ms: 30_000,
            failure_threshold: 3,
            rpc_timeout_secs: 30,
            max_retries: 3,
            max_block_lag: 10,
            solana_max_block_lag: 64,
            global_rate_limit_per_min: 0,
            stats_batch_interval_ms: 1_000,
            log_requests: false,
        }
    }
}

impl GatewayConfig {
    pub fn max_block_lag_for_family(&self, family: &str) -> i64 {
        match family {
            "solana" => self.solana_max_block_lag.max(0),
            _ => self.max_block_lag.max(0),
        }
    }
}

#[derive(Clone)]
pub struct SettingsHandle {
    config: Arc<watch::Receiver<GatewayConfig>>,
    ready: Arc<AtomicBool>,
}

impl SettingsHandle {
    pub fn get(&self) -> GatewayConfig {
        self.config.borrow().clone()
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Relaxed)
    }

    pub fn mark_ready(&self) {
        self.ready.store(true, Ordering::Relaxed);
    }
}

pub struct SettingsReloader {
    db: wallet_db::Db,
    tx: watch::Sender<GatewayConfig>,
    ready: Arc<AtomicBool>,
    interval: Duration,
}

impl SettingsReloader {
    pub fn new(db: wallet_db::Db, interval: Duration) -> (Self, SettingsHandle) {
        let (tx, rx) = watch::channel(GatewayConfig::default());
        let ready = Arc::new(AtomicBool::new(false));
        (
            Self {
                db,
                tx,
                ready: ready.clone(),
                interval,
            },
            SettingsHandle {
                config: Arc::new(rx),
                ready,
            },
        )
    }

    pub async fn load_now(&self) -> AppResult<GatewayConfig> {
        let config = self.fetch_from_db().await?;
        let _ = self.tx.send(config.clone());
        self.ready.store(true, Ordering::Relaxed);
        Ok(config)
    }

    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            if let Err(e) = self.load_now().await {
                tracing::error!("initial settings load failed: {e}");
            }
            loop {
                tokio::time::sleep(self.interval).await;
                match self.fetch_from_db().await {
                    Ok(config) => {
                        let _ = self.tx.send(config);
                        tracing::debug!("gateway settings reloaded");
                    }
                    Err(e) => {
                        tracing::warn!("settings reload failed: {e}");
                    }
                }
            }
        })
    }

    async fn fetch_from_db(&self) -> AppResult<GatewayConfig> {
        let mut db = self.db.clone_inner();
        let rows: Vec<GatewaySettings> = GatewaySettings::all()
            .exec(&mut db)
            .await
            .map_err(|e| wallet_error::AppError::internal(e.to_string()))?;

        let map: DashMap<String, String> = DashMap::new();
        for row in &rows {
            map.insert(row.key.clone(), row.value.clone());
        }

        let get = |key: &str| -> Option<String> { map.get(key).map(|r| r.value().clone()) };

        Ok(GatewayConfig {
            health_check_interval_ms: get("health_check_interval_ms")
                .and_then(|v| v.parse().ok())
                .unwrap_or(30_000),
            failure_threshold: get("failure_threshold")
                .and_then(|v| v.parse().ok())
                .unwrap_or(3),
            rpc_timeout_secs: get("rpc_timeout_secs")
                .and_then(|v| v.parse().ok())
                .unwrap_or(30),
            max_retries: get("max_retries").and_then(|v| v.parse().ok()).unwrap_or(3),
            max_block_lag: get("max_block_lag")
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            solana_max_block_lag: get("solana_max_block_lag")
                .and_then(|v| v.parse().ok())
                .unwrap_or(64),
            global_rate_limit_per_min: get("global_rate_limit_per_min")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0),
            stats_batch_interval_ms: get("stats_batch_interval_ms")
                .and_then(|v| v.parse().ok())
                .unwrap_or(1_000),
            log_requests: get("log_requests")
                .and_then(|v| v.parse().ok())
                .unwrap_or(false),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::GatewayConfig;

    #[test]
    fn solana_uses_dedicated_max_block_lag() {
        let cfg = GatewayConfig {
            max_block_lag: 10,
            solana_max_block_lag: 64,
            ..GatewayConfig::default()
        };
        assert_eq!(cfg.max_block_lag_for_family("solana"), 64);
        assert_eq!(cfg.max_block_lag_for_family("evm"), 10);
        assert_eq!(cfg.max_block_lag_for_family("bitcoin"), 10);
        assert_eq!(cfg.max_block_lag_for_family("tron"), 10);
    }

    #[test]
    fn default_solana_max_block_lag_allows_typical_slot_skew() {
        let cfg = GatewayConfig::default();
        assert!(cfg.solana_max_block_lag >= 40);
        assert_eq!(cfg.max_block_lag_for_family("solana"), cfg.solana_max_block_lag);
    }
}
