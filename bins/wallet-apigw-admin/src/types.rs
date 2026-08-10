use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

// DTOs shared verbatim with the app gateway live in `wallet_gateway::dto`.
pub use wallet_gateway::dto::{
    DappInfo, ListDappsResponse, ListNetworksResponse, ListTokensResponse, NetworkInfo, PageMeta,
    TokenInfo,
};

// ─────────────────────── Request types ───────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct NetworkBody {
    /// Chain index
    pub chain_index: i64,
    /// Network name
    pub name: String,
    /// Chain family (e.g. "evm", "tron")
    pub family: String,
    /// EVM chain ID
    pub evm_chain_id: u64,
    /// Whether the network is enabled
    pub enabled: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TokenBody {
    /// Unique token identifier
    pub id: String,
    /// Chain index
    pub chain_index: i64,
    /// Token contract address
    pub address: String,
    /// Token symbol
    pub symbol: String,
    /// Token name
    pub name: String,
    /// Token decimals
    pub decimals: u32,
    /// Logo image URL
    pub logo_url: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DappBody {
    /// Dapp identifier
    pub id: String,
    /// Dapp name
    pub name: String,
    /// Dapp URL
    pub url: String,
    /// Logo image URL
    pub logo_url: String,
    /// Supported chain indexes
    pub chain_indexes: Vec<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RpcEndpointBody {
    /// Endpoint identifier
    pub id: String,
    /// Chain index
    pub chain_index: i64,
    /// RPC URL
    pub url: String,
    /// Load balancing weight
    pub weight: u32,
    /// Whether the endpoint is enabled
    pub enabled: bool,
    /// Custom request headers sent to the RPC endpoint
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,
    /// Explicit transport protocol override: http/ws/grpc/tcp (empty = auto from url scheme)
    #[serde(default)]
    pub protocol: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SwapProviderBody {
    /// Provider name
    pub name: String,
    /// Whether the provider is enabled
    pub enabled: bool,
    /// Provider configuration as JSON
    pub config_json: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SetGasPoolEnabledBody {
    /// Chain index
    pub chain_index: i64,
    /// Whether to enable or disable the gas pool
    pub enabled: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ReindexBody {
    /// Chain index
    pub chain_index: i64,
    /// Start block number
    pub from_block: u64,
    /// End block number
    pub to_block: u64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AppConfigBody {
    /// Platform identifier (e.g. "ios", "android", "web")
    pub platform: String,
    /// Minimum supported app version
    pub min_version: String,
    /// Latest app version
    pub latest_version: String,
    /// URL for forced updates
    pub force_update_url: String,
    /// Feature flags as JSON
    pub features_json: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct GuideBody {
    /// Guide identifier (empty to auto-generate)
    pub id: String,
    /// Locale code (e.g. "en", "zh")
    pub locale: String,
    /// Guide title
    pub title: String,
    /// Guide body content
    pub body: String,
}

// ─────────────────────── Response types ───────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminUserInfo {
    /// User identifier
    pub id: String,
    /// External ID
    pub external_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListUsersResponse {
    /// List of users
    pub items: Vec<AdminUserInfo>,
    /// Pagination metadata
    pub meta: PageMeta,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RpcEndpointInfo {
    /// Endpoint identifier
    pub id: String,
    /// Chain index
    pub chain_index: i64,
    /// RPC URL
    pub url: String,
    /// Weight
    pub weight: u32,
    /// Whether enabled
    pub enabled: bool,
    /// Custom request headers sent to the RPC endpoint
    pub headers: std::collections::HashMap<String, String>,
    /// Explicit transport protocol override: http/ws/grpc/tcp (empty = auto from url scheme)
    pub protocol: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListRpcEndpointsResponse {
    /// List of RPC endpoints
    pub items: Vec<RpcEndpointInfo>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OkResponse {
    /// Success status
    pub ok: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DeletedResponse {
    /// Whether the resource was deleted
    pub deleted: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AppConfigInfo {
    /// Platform identifier
    pub platform: String,
    /// Minimum supported version
    pub min_version: String,
    /// Latest version
    pub latest_version: String,
    /// Forced update URL
    pub force_update_url: String,
    /// Feature flags JSON
    pub features_json: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GuideInfo {
    /// Guide identifier
    pub id: String,
    /// Locale code
    pub locale: String,
    /// Guide title
    pub title: String,
    /// Guide body
    pub body: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListGuidesResponse {
    /// List of guides
    pub items: Vec<GuideInfo>,
    /// Pagination metadata
    pub meta: PageMeta,
}

// ─────────────────────── Proto conversions ───────────────────────

impl From<wallet_proto::wallet::v1::admin::RpcEndpoint> for RpcEndpointInfo {
    fn from(r: wallet_proto::wallet::v1::admin::RpcEndpoint) -> Self {
        Self {
            id: r.id,
            chain_index: r.chain_index,
            url: r.url,
            weight: r.weight,
            enabled: r.enabled,
            headers: r.headers,
            protocol: r.protocol,
        }
    }
}

impl From<wallet_proto::wallet::v1::admin::AdminUser> for AdminUserInfo {
    fn from(u: wallet_proto::wallet::v1::admin::AdminUser) -> Self {
        Self {
            id: u.id,
            external_id: u.external_id,
        }
    }
}

impl From<wallet_proto::wallet::v1::AppConfig> for AppConfigInfo {
    fn from(c: wallet_proto::wallet::v1::AppConfig) -> Self {
        Self {
            platform: c.platform,
            min_version: c.min_version,
            latest_version: c.latest_version,
            force_update_url: c.force_update_url,
            features_json: c.features_json,
        }
    }
}

impl From<wallet_proto::wallet::v1::Guide> for GuideInfo {
    fn from(g: wallet_proto::wallet::v1::Guide) -> Self {
        Self {
            id: g.id,
            locale: g.locale,
            title: g.title,
            body: g.body,
        }
    }
}
