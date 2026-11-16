//! Typed YAML configuration loaders.

use serde::Deserialize;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("yaml: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("missing required secret: {0}")]
    MissingSecret(String),
}

pub fn load_yaml<T: for<'de> Deserialize<'de>>(path: impl AsRef<Path>) -> Result<T, ConfigError> {
    let raw = fs::read_to_string(path)?;
    Ok(serde_yaml::from_str(&raw)?)
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

fn default_max_connections() -> u32 {
    16
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RedisMode {
    Standalone,
    Cluster,
    Sentinel,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub addrs: Vec<String>,
    #[serde(default)]
    pub mode: Option<RedisMode>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub db: Option<u8>,
    #[serde(default)]
    pub tls: bool,
    #[serde(default)]
    pub sentinel_service: Option<String>,
}

impl RedisConfig {
    /// Resolve candidate connection URLs for standalone, cluster, or sentinel
    /// deployments while remaining backward-compatible with a single `url`.
    pub fn candidate_urls(&self) -> Vec<String> {
        let mut raw_targets: Vec<&str> = self
            .addrs
            .iter()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        let url_trimmed = self.url.trim();
        if !url_trimmed.is_empty() && !raw_targets.contains(&url_trimmed) {
            if raw_targets.is_empty() {
                raw_targets.push(url_trimmed);
            } else {
                raw_targets.insert(0, url_trimmed);
            }
        }

        let multi_addrs = self.addrs.iter().filter(|s| !s.trim().is_empty()).count() > 1;
        raw_targets
            .into_iter()
            .map(|raw| self.normalize_url(raw, multi_addrs))
            .collect()
    }

    fn normalize_url(&self, raw: &str, multi_addrs: bool) -> String {
        let (has_tls_scheme, inferred_mode, rest) = if let Some(r) = raw.strip_prefix("rediss-cluster://") {
            (true, Some(RedisMode::Cluster), r)
        } else if let Some(r) = raw.strip_prefix("redis-cluster://") {
            (false, Some(RedisMode::Cluster), r)
        } else if let Some(r) = raw.strip_prefix("rediss-sentinel://") {
            (true, Some(RedisMode::Sentinel), r)
        } else if let Some(r) = raw.strip_prefix("redis-sentinel://") {
            (false, Some(RedisMode::Sentinel), r)
        } else if let Some(r) = raw.strip_prefix("rediss://") {
            (true, None, r)
        } else if let Some(r) = raw.strip_prefix("redis://") {
            (false, None, r)
        } else {
            (false, None, raw)
        };

        let mode = self.mode.or(inferred_mode).unwrap_or({
            if multi_addrs {
                RedisMode::Cluster
            } else {
                RedisMode::Standalone
            }
        });
        let tls = self.tls || has_tls_scheme;

        let scheme = match (mode, tls) {
            (RedisMode::Standalone, false) => "redis",
            (RedisMode::Standalone, true) => "rediss",
            (RedisMode::Cluster, false) => "redis-cluster",
            (RedisMode::Cluster, true) => "rediss-cluster",
            (RedisMode::Sentinel, false) => "redis-sentinel",
            (RedisMode::Sentinel, true) => "rediss-sentinel",
        };

        let mut body = rest.to_string();
        if !body.contains('@') {
            let user = self.username.as_deref().unwrap_or("");
            let pass = self.password.as_deref().unwrap_or("");
            if !user.is_empty() || !pass.is_empty() {
                body = format!("{user}:{pass}@{body}");
            }
        }

        if mode == RedisMode::Standalone {
            if let Some(db) = self.db {
                let host_part = body.rsplit('@').next().unwrap_or(&body);
                if !host_part.contains('/') {
                    body = format!("{body}/{db}");
                }
            }
        } else if mode == RedisMode::Sentinel {
            if let Some(service) = self.sentinel_service.as_deref().filter(|s| !s.is_empty()) {
                let host_part = body.rsplit('@').next().unwrap_or(&body);
                if !host_part.contains('/') {
                    body = format!("{body}/{service}");
                }
            }
        }

        format!("{scheme}://{body}")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct NatsConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClickHouseConfig {
    pub url: String,
    pub database: String,
    #[serde(default)]
    pub user: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcEndpoint {
    pub url: String,
    #[serde(default = "default_weight")]
    pub weight: u32,
}

fn default_weight() -> u32 {
    1
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChainRuntimeConfig {
    pub chain_index: i64,
    pub family: String,
    #[serde(default)]
    pub evm_chain_id: Option<u64>,
    #[serde(default)]
    pub ton_api_version: Option<u32>, // TON API version: 2, 3, or 4
    pub endpoints: Vec<RpcEndpoint>,
    #[serde(default = "default_confirmations")]
    pub confirmations: u64,
}

fn default_confirmations() -> u64 {
    12
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    /// Optional JWT secret from YAML. Prefer the env var named by `env_key`
    /// (default `JWT_SECRET`); see [`JwtConfig::secret`].
    #[serde(default)]
    secret: Option<String>,
    /// Env var name that overrides `secret` (default `JWT_SECRET`).
    #[serde(default)]
    env_key: Option<String>,
    #[serde(default = "default_issuer")]
    pub issuer: String,
}

impl JwtConfig {
    /// JWT signing/verification secret. Prefers the env var named by `env_key`
    /// (default `JWT_SECRET`), then the YAML value. Errors when neither is set
    /// so a missing secret can never silently weaken to a known default.
    pub fn secret(&self) -> Result<String, ConfigError> {
        let key = self.env_key.as_deref().unwrap_or("JWT_SECRET");
        match std::env::var(key) {
            Ok(s) if !s.trim().is_empty() => Ok(s),
            _ => self.secret.clone().ok_or_else(|| {
                ConfigError::MissingSecret(format!("{key} env or jwt.secret yaml key"))
            }),
        }
    }
}

fn default_issuer() -> String {
    "viva-wallet".into()
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiConfig {
    pub listen: String,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub nats: NatsConfig,
    #[serde(default)]
    pub clickhouse: Option<ClickHouseConfig>,
    #[serde(default)]
    pub chains: Vec<ChainRuntimeConfig>,
    pub jwt: JwtConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GatewayConfig {
    pub listen: String,
    pub grpc_endpoint: String,
    pub jwt: JwtConfig,
    #[serde(default)]
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SyncConfig {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub nats: NatsConfig,
    pub chain: ChainRuntimeConfig,
    #[serde(default = "default_poll_ms")]
    pub poll_interval_ms: u64,
    /// First block height to backfill from when no cursor exists yet.
    /// When absent, sync starts at the current safe tip.
    #[serde(default)]
    pub start_height: Option<u64>,
    /// How many blocks to fetch from the RPC in parallel inside one tick.
    ///
    /// High-BPS chains (Arbitrum, Base, Optimism, Polygon) produce blocks
    /// faster than a single sequential `fetch_block_txs` round trip can
    /// keep up with, so we batch `block_fetch_concurrency` heights per
    /// `join_all` and then process them in height order to keep the cursor
    /// monotonic. Tune to roughly the number of healthy RPC endpoints you
    /// have, or to whatever the gateway's per-endpoint QPS ceiling allows.
    #[serde(default = "default_block_fetch_concurrency")]
    pub block_fetch_concurrency: usize,
}

fn default_poll_ms() -> u64 {
    2000
}

fn default_block_fetch_concurrency() -> usize {
    4
}

#[derive(Debug, Clone, Deserialize)]
pub struct JobsConfig {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub nats: NatsConfig,
    #[serde(default)]
    pub clickhouse: Option<ClickHouseConfig>,
    #[serde(default)]
    pub chains: Vec<ChainRuntimeConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsConfig {
    pub listen: String,
    pub nats: NatsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChainGatewayConfig {
    pub listen: String,
    #[serde(default)]
    pub grpc_listen: Option<String>,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    #[serde(default = "default_health_check_interval_ms")]
    pub health_check_interval_ms: u64,
    #[serde(default = "default_failure_threshold")]
    pub failure_threshold: u32,
    #[serde(default)]
    pub admin_key: Option<String>,
    #[serde(default)]
    pub admin_username: Option<String>,
    #[serde(default)]
    pub admin_password: Option<String>,
    #[serde(default = "default_stats_batch_interval_ms")]
    pub stats_batch_interval_ms: u64,
}

fn default_health_check_interval_ms() -> u64 {
    30_000
}

fn default_failure_threshold() -> u32 {
    3
}

fn default_stats_batch_interval_ms() -> u64 {
    1_000
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookConfig {
    pub listen: String,
    pub nats: NatsConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Validate that the in-repo per-chain sync configs deserialize into
    /// `SyncConfig` and have the expected chain_index / family. This catches
    /// schema drift (e.g. a renamed field) before `wallet-sync` boots with
    /// a broken config.
    #[test]
    fn sync_configs_deserialize() {
        let ton: SyncConfig = load_yaml("../../configs/wallet-sync-ton.yaml")
            .expect("wallet-sync-ton.yaml must deserialize");
        assert_eq!(ton.chain.chain_index, 607);
        assert_eq!(ton.chain.family, "ton");
        assert_eq!(ton.chain.confirmations, 5);
        assert!(!ton.chain.endpoints.is_empty());
        assert_eq!(ton.poll_interval_ms, 5000);
        assert!(ton.start_height.is_none());

        let sui: SyncConfig = load_yaml("../../configs/wallet-sync-sui.yaml")
            .expect("wallet-sync-sui.yaml must deserialize");
        assert_eq!(sui.chain.chain_index, 784);
        assert_eq!(sui.chain.family, "sui");
        assert_eq!(sui.chain.confirmations, 50);
        assert!(!sui.chain.endpoints.is_empty());
        assert_eq!(sui.poll_interval_ms, 5000);

        // Regression guard: TRON config must keep chain_index 195 + family
        // "tron" so the existing wallet-sync-tron deployment isn't broken
        // by the TON/Sui addition.
        let tron: SyncConfig = load_yaml("../../configs/wallet-sync-tron.yaml")
            .expect("wallet-sync-tron.yaml must deserialize");
        assert_eq!(tron.chain.chain_index, 195);
        assert_eq!(tron.chain.family, "tron");

        let zec: SyncConfig = load_yaml("../../configs/wallet-sync-zec.yaml")
            .expect("wallet-sync-zec.yaml must deserialize");
        assert_eq!(zec.chain.chain_index, 133);
        assert_eq!(zec.chain.family, "zcash");
        assert_eq!(zec.chain.confirmations, 12);
        assert!(!zec.chain.endpoints.is_empty());

        // DOGE (chain_index 3) shares the `bitcoin` family protocol, so the
        // family string must stay "bitcoin" and chain_index 3.
        let doge: SyncConfig = load_yaml("../../configs/wallet-sync-doge.yaml")
            .expect("wallet-sync-doge.yaml must deserialize");
        assert_eq!(doge.chain.chain_index, 3);
        assert_eq!(doge.chain.family, "bitcoin");
        assert_eq!(doge.chain.confirmations, 12);
        assert!(!doge.chain.endpoints.is_empty());
    }

    #[test]
    fn redis_config_resolves_standalone_cluster_and_sentinel() {
        let standalone: RedisConfig = serde_yaml::from_str("url: redis://127.0.0.1:6379")
            .expect("standalone yaml");
        assert_eq!(
            standalone.candidate_urls(),
            vec!["redis://127.0.0.1:6379".to_string()]
        );

        let cluster_multi: RedisConfig = serde_yaml::from_str(
            r#"
addrs:
  - "10.0.0.1:6379"
  - "10.0.0.2:6379"
password: "secret"
tls: true
"#,
        )
        .expect("cluster multi addrs yaml");
        assert_eq!(
            cluster_multi.candidate_urls(),
            vec![
                "rediss-cluster://:secret@10.0.0.1:6379".to_string(),
                "rediss-cluster://:secret@10.0.0.2:6379".to_string(),
            ]
        );

        let sentinel: RedisConfig = serde_yaml::from_str(
            r#"
mode: sentinel
addrs:
  - "10.0.0.10:26379"
sentinel_service: "mymaster"
password: "pw"
"#,
        )
        .expect("sentinel yaml");
        assert_eq!(
            sentinel.candidate_urls(),
            vec!["redis-sentinel://:pw@10.0.0.10:26379/mymaster".to_string()]
        );
    }
}
