use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone, toasty::Model)]
pub struct User {
    #[key]
    #[auto]
    pub id: Uuid,

    #[unique]
    pub external_id: String,

    #[default(jiff::Timestamp::now())]
    pub created_at: jiff::Timestamp,
}

#[derive(Debug, Clone, toasty::Model)]
pub struct Address {
    #[key]
    #[auto]
    pub id: Uuid,

    #[index]
    pub user_id: Uuid,

    pub chain_index: i64,

    pub address: String,

    #[default(jiff::Timestamp::now())]
    pub created_at: jiff::Timestamp,
}

#[derive(Debug, Clone, toasty::Model)]
pub struct Network {
    #[key]
    pub chain_index: i64,

    pub name: String,

    pub family: String,

    pub evm_chain_id: Option<i64>,

    pub enabled: bool,

    #[default(jiff::Timestamp::now())]
    pub created_at: jiff::Timestamp,
}

#[derive(Debug, Clone, toasty::Model)]
pub struct RpcEndpoint {
    #[key]
    #[auto]
    pub id: Uuid,

    #[index]
    pub chain_index: i64,

    pub url: String,

    pub weight: i32,

    pub enabled: bool,

    #[default(jiff::Timestamp::now())]
    pub created_at: jiff::Timestamp,
}

#[derive(Debug, Clone, toasty::Model)]
pub struct Token {
    #[key]
    #[auto]
    pub id: Uuid,

    #[index]
    pub chain_index: i64,

    pub address: String,

    pub symbol: String,

    pub name: String,

    pub decimals: i32,

    pub logo_url: Option<String>,

    #[default(jiff::Timestamp::now())]
    pub created_at: jiff::Timestamp,
}

#[derive(Debug, Clone, toasty::Model)]
pub struct Tx {
    #[key]
    #[auto]
    pub id: Uuid,

    #[index]
    pub chain_index: i64,

    pub hash: String,

    pub from_address: Option<String>,

    pub to_address: Option<String>,

    pub value: Decimal,

    pub block_number: i64,

    pub status: String,

    #[default(jiff::Timestamp::now())]
    pub created_at: jiff::Timestamp,
}

#[derive(Debug, Clone, toasty::Model)]
pub struct GasPool {
    #[key]
    pub chain_index: i64,

    pub balance: Decimal,

    pub enabled: bool,

    pub hot_wallet: String,

    pub cold_wallet: String,
}

#[derive(Debug, Clone, toasty::Model)]
pub struct SyncCursor {
    #[key]
    pub chain_index: i64,

    pub height: i64,
}

#[derive(Debug, Clone, toasty::Model)]
pub struct SwapProvider {
    #[key]
    pub name: String,

    pub enabled: bool,

    pub config_json: String,
}

#[derive(Debug, Clone, toasty::Model)]
pub struct LatestPrice {
    #[key]
    pub symbol: String,

    pub price: String,
}

#[derive(Debug, Clone, toasty::Model)]
pub struct DexQuote {
    #[key]
    #[auto]
    pub id: Uuid,

    pub chain_index: i64,

    pub wallet: String,

    pub from_token: String,

    pub to_token: String,

    pub amount: String,

    pub price_impact: String,

    pub tx_hash: String,

    #[default(jiff::Timestamp::now())]
    pub created_at: jiff::Timestamp,
}

#[derive(Debug, Clone, toasty::Model)]
pub struct AppConfig {
    #[key]
    pub platform: String,

    pub min_version: String,

    pub latest_version: String,

    pub force_update_url: Option<String>,

    pub features_json: String,
}

#[derive(Debug, Clone, toasty::Model)]
pub struct Guide {
    #[key]
    #[auto]
    pub id: Uuid,

    pub locale: String,

    pub title: String,

    pub body: String,

    #[default(jiff::Timestamp::now())]
    pub created_at: jiff::Timestamp,
}

#[derive(Debug, Clone, toasty::Model)]
pub struct ChainGatewayKey {
    #[key]
    #[auto]
    pub id: Uuid,

    #[unique]
    pub api_key: String,

    pub name: String,

    pub rate_limit_per_min: i32,

    pub enabled: bool,
}

#[derive(Debug, Clone, toasty::Model)]
pub struct Dapp {
    #[key]
    #[auto]
    pub id: Uuid,

    pub name: String,

    pub url: String,

    pub logo_url: Option<String>,

    pub chain_indexes: Vec<i64>,

    #[default(jiff::Timestamp::now())]
    pub created_at: jiff::Timestamp,
}
