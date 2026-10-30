use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

// DTOs shared verbatim with the admin gateway live in `wallet_gateway::dto`.
pub use wallet_gateway::dto::{
    DappInfo, ListDappsResponse, ListNetworksResponse, ListTokensResponse, NetworkInfo, PageMeta,
    TokenInfo,
};

// ─────────────────────── Request types ───────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct BroadcastBody {
    /// Chain index
    pub chain_index: i64,
    /// Raw transaction hex (with or without 0x prefix)
    pub raw_tx_hex: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct QuoteBody {
    /// Chain index
    pub chain_index: i64,
    /// Source token contract address
    pub from_token: String,
    /// Destination token contract address
    pub to_token: String,
    /// Amount in smallest unit (wei, sun, etc.)
    pub amount: String,
    /// DEX provider name (default: "1inch")
    pub provider: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EstimateGasBody {
    /// Chain index
    pub chain_index: i64,
    /// Sender address
    pub from_address: String,
    /// Recipient address (optional for contract creation)
    pub to_address: Option<String>,
    /// Hex-encoded calldata (optional)
    pub data: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RentBody {
    /// TRON address to estimate energy for
    pub address: String,
    /// Amount of energy to rent
    pub energy: u64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SponsorBody {
    /// Chain index
    pub chain_index: i64,
    /// User's address
    pub user_address: String,
    /// Hex-encoded user operation or transaction
    pub user_op_or_tx: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterBody {
    /// External ID from the identity provider
    pub external_id: String,
}

// ─────────────────────── Response types ───────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct BalanceItem {
    /// Token contract address
    pub token: String,
    /// Balance in smallest unit
    pub amount: String,
    /// Token decimals
    pub decimals: u32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GetBalancesResponse {
    /// Token balances
    pub items: Vec<BalanceItem>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransactionInfo {
    /// Transaction hash
    pub hash: String,
    /// Chain index
    pub chain_index: i64,
    /// Sender address
    pub from_address: String,
    /// Recipient address
    pub to_address: String,
    /// Value in smallest unit
    pub value: String,
    /// Block number
    pub block_number: u64,
    /// Transaction status (e.g. "success", "pending", "failed")
    pub status: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListTransactionsResponse {
    /// List of transactions
    pub items: Vec<TransactionInfo>,
    /// Pagination metadata
    pub meta: PageMeta,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BroadcastResponse {
    /// Transaction hash
    pub hash: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EstimateGasResponse {
    /// Estimated gas limit
    pub gas_limit: u64,
    /// Maximum fee per gas (wei)
    pub max_fee_per_gas: String,
    /// Maximum priority fee per gas (wei)
    pub max_priority_fee_per_gas: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QuoteResponse {
    /// DEX provider name
    pub provider: String,
    /// Output amount in smallest unit
    pub amount_out: String,
    /// Route information as JSON string
    pub route_json: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PriceInfo {
    /// Trading pair symbol (e.g. "ETHUSDT")
    pub symbol: String,
    /// Current price
    pub price: String,
    /// Last update time (Unix timestamp)
    pub updated_at_unix: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct KlineInfo {
    /// Kline open time (Unix timestamp)
    pub open_time: i64,
    /// Open price
    pub open: String,
    /// High price
    pub high: String,
    /// Low price
    pub low: String,
    /// Close price
    pub close: String,
    /// Trading volume
    pub volume: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GetKlinesResponse {
    /// List of kline data points
    pub items: Vec<KlineInfo>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GasPoolInfo {
    /// Chain index
    pub chain_index: i64,
    /// Available gas balance
    pub balance: String,
    /// Whether gas pool is enabled
    pub enabled: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SponsorResponse {
    /// Sponsorship status
    pub status: String,
    /// Transaction hash (if submitted)
    pub tx_hash: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EstimateEnergyResponse {
    /// Energy price (TRX per energy)
    pub price: String,
    /// Rental duration in hours
    pub duration_hours: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AppConfigInfo {
    /// Minimum supported app version
    pub min_version: String,
    /// Latest app version
    pub latest_version: String,
    /// Force update download URL
    pub force_update_url: String,
    /// Feature flags as JSON string
    pub features_json: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GuideInfo {
    /// Guide identifier
    pub id: String,
    /// Guide title
    pub title: String,
    /// Guide body (Markdown)
    pub body: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListGuidesResponse {
    /// List of guides
    pub items: Vec<GuideInfo>,
    /// Pagination metadata
    pub meta: PageMeta,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserInfo {
    /// User identifier
    pub id: String,
    /// External identity provider ID
    pub external_id: String,
    /// Registration time (Unix timestamp)
    pub created_at_unix: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AddressInfo {
    /// Wallet address
    pub address: String,
    /// Chain index
    pub chain_index: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListAddressesResponse {
    /// List of wallet addresses
    pub items: Vec<AddressInfo>,
}

// ─────────────────────── Proto conversions ───────────────────────

impl From<wallet_proto::wallet::v1::BalanceItem> for BalanceItem {
    fn from(b: wallet_proto::wallet::v1::BalanceItem) -> Self {
        Self {
            token: b.token,
            amount: b.amount,
            decimals: b.decimals,
        }
    }
}

impl From<wallet_proto::wallet::v1::Transaction> for TransactionInfo {
    fn from(t: wallet_proto::wallet::v1::Transaction) -> Self {
        Self {
            hash: t.hash,
            chain_index: t.chain_index,
            from_address: t.from_address,
            to_address: t.to_address,
            value: t.value,
            block_number: t.block_number,
            status: t.status,
        }
    }
}

impl From<wallet_proto::wallet::v1::Price> for PriceInfo {
    fn from(p: wallet_proto::wallet::v1::Price) -> Self {
        Self {
            symbol: p.symbol,
            price: p.price,
            updated_at_unix: p.updated_at_unix,
        }
    }
}

impl From<wallet_proto::wallet::v1::Kline> for KlineInfo {
    fn from(k: wallet_proto::wallet::v1::Kline) -> Self {
        Self {
            open_time: k.open_time,
            open: k.open,
            high: k.high,
            low: k.low,
            close: k.close,
            volume: k.volume,
        }
    }
}

impl From<wallet_proto::wallet::v1::GasPool> for GasPoolInfo {
    fn from(g: wallet_proto::wallet::v1::GasPool) -> Self {
        Self {
            chain_index: g.chain_index,
            balance: g.balance,
            enabled: g.enabled,
        }
    }
}

impl From<wallet_proto::wallet::v1::AppConfig> for AppConfigInfo {
    fn from(a: wallet_proto::wallet::v1::AppConfig) -> Self {
        Self {
            min_version: a.min_version,
            latest_version: a.latest_version,
            force_update_url: a.force_update_url,
            features_json: a.features_json,
        }
    }
}

impl From<wallet_proto::wallet::v1::Guide> for GuideInfo {
    fn from(g: wallet_proto::wallet::v1::Guide) -> Self {
        Self {
            id: g.id,
            title: g.title,
            body: g.body,
        }
    }
}

impl From<wallet_proto::wallet::v1::User> for UserInfo {
    fn from(u: wallet_proto::wallet::v1::User) -> Self {
        Self {
            id: u.id,
            external_id: u.external_id,
            created_at_unix: u.created_at_unix,
        }
    }
}

impl From<wallet_proto::wallet::v1::AddressInfo> for AddressInfo {
    fn from(a: wallet_proto::wallet::v1::AddressInfo) -> Self {
        Self {
            address: a.address,
            chain_index: a.chain_index,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MetricsResponse {
    /// Total requests processed
    pub requests_total: u64,
    /// Successful requests
    pub requests_success: u64,
    /// Failed requests
    pub requests_error: u64,
    /// Total DB queries
    pub db_queries_total: u64,
    /// Failed DB queries
    pub db_queries_error: u64,
    /// Total RPC calls
    pub rpc_calls_total: u64,
    /// Failed RPC calls
    pub rpc_calls_error: u64,
    /// Events published
    pub events_published: u64,
}

// ─────────────────────── Recipient Check ───────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct CheckRecipientBody {
    pub chain_index: i64,
    pub address: String,
    pub sender_address: String,
    pub current_token_contract: Option<String>,
    pub reference_addresses: Option<Vec<String>>,
    pub trusted_recipient: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecipientCheckItemDto {
    pub kind: String,
    pub outcome: String,
    pub action: String,
    pub source: String,
    pub checked_at_ms: i64,
    pub expires_at_ms: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CheckRecipientResponse {
    pub action: String,
    pub reason: String,
    pub complete: bool,
    pub rules_version: String,
    pub valid_until_ms: i64,
    pub items: Vec<RecipientCheckItemDto>,
    pub similarity_prefix: i32,
    pub similarity_suffix: i32,
    pub black_hole_addresses: Vec<String>,
}

// ─────────────────────── Batch Balances ───────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct WalletAddressChainsDto {
    pub address: String,
    pub chain_indexes: Vec<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct BatchBalancesBody {
    pub addresses: Vec<WalletAddressChainsDto>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenBalanceDetailDto {
    pub chain_index: i64,
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: i64,
    pub raw_balance: String,
    pub price: String,
    pub logo_url: String,
    pub is_official: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WalletBalanceEntryDto {
    pub address: String,
    pub balances: Vec<TokenBalanceDetailDto>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BatchBalancesResponse {
    pub list: Vec<WalletBalanceEntryDto>,
}

// ─────────────────────── Version Check ───────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct VersionCheckQuery {
    pub platform: String,
    pub version: String,
    pub locale: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VersionCheckResponse {
    pub need_update: bool,
    pub force_update: bool,
    pub latest_version: String,
    pub min_version: String,
    pub download_url: String,
    pub title: String,
    pub description: String,
}

// ─────────────────────── Geo & IP Check ───────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct GeoCheckResponse {
    pub is_allowed: bool,
    pub country_code: String,
    pub country_name: String,
    pub reason: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IpAccessResponse {
    pub is_whitelisted: bool,
    pub is_blocked: bool,
    pub reason: String,
}

// ─────────────────────── Transactions Filter & Summary ───────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct TransactionsFilterQuery {
    pub chain_index: Option<i64>,
    pub addresses: String,
    pub status: Option<i64>,
    pub token_address: Option<String>,
    pub keyword: Option<String>,
    pub transfer_type: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub include_spam: Option<bool>,
    pub is_spam: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransactionsFilterResponse {
    pub count: i64,
    pub list: Vec<TransactionInfo>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransactionSummaryDto {
    pub txn: String,
    pub chain_index: i64,
    pub chain_name: String,
    pub from_address: String,
    pub to_address: String,
    pub value: String,
    pub fee: String,
    pub tx_type: String,
    pub token_address: String,
    pub timestamp: i64,
    pub status: String,
    pub block_number: i64,
    pub token_decimal: i64,
    pub token_symbol: String,
    pub token_logo_url: String,
    pub token_name: String,
    pub risk_level: String,
    pub is_spam: bool,
    pub price: String,
    pub native_price: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransactionsSummaryResponse {
    pub count: i64,
    pub list: Vec<TransactionSummaryDto>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransactionDetailResponse {
    pub txn: String,
    pub from_address: String,
    pub to_address: String,
    pub value: String,
    pub fee: String,
    pub tx_type: String,
    pub token_address: String,
    pub timestamp: i64,
    pub status: String,
    pub block_number: i64,
    pub chain_index: i64,
    pub use_gaspool: bool,
    pub price: String,
    pub native_price: String,
}

// ─────────────────────── DApp Approvals & Search ───────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct DappApprovalsQuery {
    pub chain_index: i64,
    pub address: String,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenApprovalItemDto {
    pub chain_index: i64,
    pub dapp_name: String,
    pub dapp_url: String,
    pub spender_address: String,
    pub token_address: String,
    pub token_symbol: String,
    pub allowance: String,
    pub is_unlimited: bool,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DappApprovalsResponse {
    pub items: Vec<TokenApprovalItemDto>,
    pub meta: PageMeta,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DappSearchQuery {
    pub query: String,
    pub chain_index: Option<i64>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}
