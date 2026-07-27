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

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub url: String,
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
    pub endpoints: Vec<RpcEndpoint>,
    #[serde(default = "default_confirmations")]
    pub confirmations: u64,
}

fn default_confirmations() -> u64 {
    12
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    #[serde(default = "default_issuer")]
    pub issuer: String,
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
    pub chain_index: i64,
    pub database: DatabaseConfig,
    pub nats: NatsConfig,
    pub chain: ChainRuntimeConfig,
    #[serde(default = "default_poll_ms")]
    pub poll_interval_ms: u64,
}

fn default_poll_ms() -> u64 {
    2000
}

#[derive(Debug, Clone, Deserialize)]
pub struct JobsConfig {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub nats: NatsConfig,
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
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    #[serde(default = "default_health_check_interval_ms")]
    pub health_check_interval_ms: u64,
    #[serde(default = "default_failure_threshold")]
    pub failure_threshold: u32,
    #[serde(default)]
    pub admin_key: Option<String>,
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
