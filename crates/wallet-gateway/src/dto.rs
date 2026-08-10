//! HTTP response DTOs shared by the app and admin gateways so the two surfaces
//! cannot drift apart for the same entity. Only entities whose DTOs are
//! byte-for-byte identical across gateways live here; gateway-specific DTOs
//! stay local to each binary.

use salvo::oapi::ToSchema;
use serde::Serialize;

#[derive(Debug, Serialize, ToSchema)]
pub struct NetworkInfo {
    /// Chain index identifier
    pub chain_index: i64,
    /// Network name (e.g. "Ethereum", "TRON")
    pub name: String,
    /// Chain family (e.g. "evm", "tron")
    pub family: String,
    /// EVM chain ID (0 for non-EVM chains)
    pub evm_chain_id: u64,
    /// Whether the network is enabled
    pub enabled: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListNetworksResponse {
    /// List of blockchain networks
    pub items: Vec<NetworkInfo>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenInfo {
    /// Unique token identifier
    pub id: String,
    /// Chain index
    pub chain_index: i64,
    /// Token contract address
    pub address: String,
    /// Token symbol (e.g. "ETH", "USDT")
    pub symbol: String,
    /// Token name (e.g. "Ethereum", "Tether")
    pub name: String,
    /// Token decimals
    pub decimals: u32,
    /// Logo image URL
    pub logo_url: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PageMeta {
    /// Current page number
    pub page: u32,
    /// Items per page
    pub page_size: u32,
    /// Total number of items
    pub total: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListTokensResponse {
    /// List of tokens
    pub items: Vec<TokenInfo>,
    /// Pagination metadata
    pub meta: PageMeta,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DappInfo {
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

#[derive(Debug, Serialize, ToSchema)]
pub struct ListDappsResponse {
    /// List of dapps
    pub items: Vec<DappInfo>,
    /// Pagination metadata
    pub meta: PageMeta,
}

impl From<wallet_proto::wallet::v1::Network> for NetworkInfo {
    fn from(n: wallet_proto::wallet::v1::Network) -> Self {
        Self {
            chain_index: n.chain_index,
            name: n.name,
            family: n.family,
            evm_chain_id: n.evm_chain_id,
            enabled: n.enabled,
        }
    }
}

impl From<wallet_proto::wallet::v1::Token> for TokenInfo {
    fn from(t: wallet_proto::wallet::v1::Token) -> Self {
        Self {
            id: t.id,
            chain_index: t.chain_index,
            address: t.address,
            symbol: t.symbol,
            name: t.name,
            decimals: t.decimals,
            logo_url: t.logo_url,
        }
    }
}

impl From<wallet_proto::wallet::v1::PageMeta> for PageMeta {
    fn from(m: wallet_proto::wallet::v1::PageMeta) -> Self {
        Self {
            page: m.page,
            page_size: m.page_size,
            total: m.total,
        }
    }
}

impl From<wallet_proto::wallet::v1::Dapp> for DappInfo {
    fn from(d: wallet_proto::wallet::v1::Dapp) -> Self {
        Self {
            id: d.id,
            name: d.name,
            url: d.url,
            logo_url: d.logo_url,
            chain_indexes: d.chain_indexes,
        }
    }
}
