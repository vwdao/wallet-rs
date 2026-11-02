// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endpoint {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="2")]
    pub port: i32,
    #[prost(bytes="vec", tag="3")]
    pub node_id: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for Endpoint {
const NAME: &'static str = "Endpoint";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingMessage {
    #[prost(message, optional, tag="1")]
    pub from: ::core::option::Option<Endpoint>,
    #[prost(message, optional, tag="2")]
    pub to: ::core::option::Option<Endpoint>,
    #[prost(int32, tag="3")]
    pub version: i32,
    #[prost(int64, tag="4")]
    pub timestamp: i64,
}
impl ::prost::Name for PingMessage {
const NAME: &'static str = "PingMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PongMessage {
    #[prost(message, optional, tag="1")]
    pub from: ::core::option::Option<Endpoint>,
    #[prost(int32, tag="2")]
    pub echo: i32,
    #[prost(int64, tag="3")]
    pub timestamp: i64,
}
impl ::prost::Name for PongMessage {
const NAME: &'static str = "PongMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindNeighbours {
    #[prost(message, optional, tag="1")]
    pub from: ::core::option::Option<Endpoint>,
    #[prost(bytes="vec", tag="2")]
    pub target_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub timestamp: i64,
}
impl ::prost::Name for FindNeighbours {
const NAME: &'static str = "FindNeighbours";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Neighbours {
    #[prost(message, optional, tag="1")]
    pub from: ::core::option::Option<Endpoint>,
    #[prost(message, repeated, tag="2")]
    pub neighbours: ::prost::alloc::vec::Vec<Endpoint>,
    #[prost(int64, tag="3")]
    pub timestamp: i64,
}
impl ::prost::Name for Neighbours {
const NAME: &'static str = "Neighbours";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupMessage {
    #[prost(bool, tag="1")]
    pub flag: bool,
    #[prost(int32, tag="2")]
    pub priority: i32,
}
impl ::prost::Name for BackupMessage {
const NAME: &'static str = "BackupMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceCode {
    Bandwidth = 0,
    Energy = 1,
    TronPower = 2,
}
impl ResourceCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResourceCode::Bandwidth => "BANDWIDTH",
            ResourceCode::Energy => "ENERGY",
            ResourceCode::TronPower => "TRON_POWER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BANDWIDTH" => Some(Self::Bandwidth),
            "ENERGY" => Some(Self::Energy),
            "TRON_POWER" => Some(Self::TronPower),
            _ => None,
        }
    }
}
/// AccountId, (name, address) use name, (null, address) use address, (name, null) use name,
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountId {
    #[prost(bytes="vec", tag="1")]
    pub name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for AccountId {
const NAME: &'static str = "AccountId";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
/// vote message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vote {
    /// the super rep address
    #[prost(bytes="vec", tag="1")]
    pub vote_address: ::prost::alloc::vec::Vec<u8>,
    /// the vote num to this super rep.
    #[prost(int64, tag="2")]
    pub vote_count: i64,
}
impl ::prost::Name for Vote {
const NAME: &'static str = "Vote";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
/// Proposal
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proposal {
    #[prost(int64, tag="1")]
    pub proposal_id: i64,
    #[prost(bytes="vec", tag="2")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(map="int64, int64", tag="3")]
    pub parameters: ::std::collections::HashMap<i64, i64>,
    #[prost(int64, tag="4")]
    pub expiration_time: i64,
    #[prost(int64, tag="5")]
    pub create_time: i64,
    #[prost(bytes="vec", repeated, tag="6")]
    pub approvals: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration="proposal::State", tag="7")]
    pub state: i32,
}
/// Nested message and enum types in `Proposal`.
pub mod proposal {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        Pending = 0,
        Disapproved = 1,
        Approved = 2,
        Canceled = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Pending => "PENDING",
                State::Disapproved => "DISAPPROVED",
                State::Approved => "APPROVED",
                State::Canceled => "CANCELED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PENDING" => Some(Self::Pending),
                "DISAPPROVED" => Some(Self::Disapproved),
                "APPROVED" => Some(Self::Approved),
                "CANCELED" => Some(Self::Canceled),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for Proposal {
const NAME: &'static str = "Proposal";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
/// Exchange
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exchange {
    #[prost(int64, tag="1")]
    pub exchange_id: i64,
    #[prost(bytes="vec", tag="2")]
    pub creator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub create_time: i64,
    #[prost(bytes="vec", tag="6")]
    pub first_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="7")]
    pub first_token_balance: i64,
    #[prost(bytes="vec", tag="8")]
    pub second_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="9")]
    pub second_token_balance: i64,
}
impl ::prost::Name for Exchange {
const NAME: &'static str = "Exchange";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
/// market
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrder {
    #[prost(bytes="vec", tag="1")]
    pub order_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub create_time: i64,
    #[prost(bytes="vec", tag="4")]
    pub sell_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub sell_token_quantity: i64,
    #[prost(bytes="vec", tag="6")]
    pub buy_token_id: ::prost::alloc::vec::Vec<u8>,
    /// min to receive
    #[prost(int64, tag="7")]
    pub buy_token_quantity: i64,
    #[prost(int64, tag="9")]
    pub sell_token_quantity_remain: i64,
    /// When state != ACTIVE and sell_token_quantity_return !=0,
    /// it means that some sell tokens are returned to the account due to insufficient remaining amount
    #[prost(int64, tag="10")]
    pub sell_token_quantity_return: i64,
    #[prost(enumeration="market_order::State", tag="11")]
    pub state: i32,
    #[prost(bytes="vec", tag="12")]
    pub prev: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="13")]
    pub next: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `MarketOrder`.
pub mod market_order {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        Active = 0,
        Inactive = 1,
        Canceled = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Active => "ACTIVE",
                State::Inactive => "INACTIVE",
                State::Canceled => "CANCELED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACTIVE" => Some(Self::Active),
                "INACTIVE" => Some(Self::Inactive),
                "CANCELED" => Some(Self::Canceled),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for MarketOrder {
const NAME: &'static str = "MarketOrder";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrderList {
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<MarketOrder>,
}
impl ::prost::Name for MarketOrderList {
const NAME: &'static str = "MarketOrderList";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrderPairList {
    #[prost(message, repeated, tag="1")]
    pub order_pair: ::prost::alloc::vec::Vec<MarketOrderPair>,
}
impl ::prost::Name for MarketOrderPairList {
const NAME: &'static str = "MarketOrderPairList";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrderPair {
    #[prost(bytes="vec", tag="1")]
    pub sell_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub buy_token_id: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for MarketOrderPair {
const NAME: &'static str = "MarketOrderPair";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketAccountOrder {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    /// order_id list
    #[prost(bytes="vec", repeated, tag="2")]
    pub orders: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// active count
    #[prost(int64, tag="3")]
    pub count: i64,
    #[prost(int64, tag="4")]
    pub total_count: i64,
}
impl ::prost::Name for MarketAccountOrder {
const NAME: &'static str = "MarketAccountOrder";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketPrice {
    #[prost(int64, tag="1")]
    pub sell_token_quantity: i64,
    #[prost(int64, tag="2")]
    pub buy_token_quantity: i64,
}
impl ::prost::Name for MarketPrice {
const NAME: &'static str = "MarketPrice";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketPriceList {
    #[prost(bytes="vec", tag="1")]
    pub sell_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub buy_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="3")]
    pub prices: ::prost::alloc::vec::Vec<MarketPrice>,
}
impl ::prost::Name for MarketPriceList {
const NAME: &'static str = "MarketPriceList";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrderIdList {
    #[prost(bytes="vec", tag="1")]
    pub head: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub tail: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for MarketOrderIdList {
const NAME: &'static str = "MarketOrderIdList";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainParameters {
    #[prost(message, repeated, tag="1")]
    pub chain_parameter: ::prost::alloc::vec::Vec<chain_parameters::ChainParameter>,
}
/// Nested message and enum types in `ChainParameters`.
pub mod chain_parameters {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChainParameter {
        #[prost(string, tag="1")]
        pub key: ::prost::alloc::string::String,
        #[prost(int64, tag="2")]
        pub value: i64,
    }
impl ::prost::Name for ChainParameter {
const NAME: &'static str = "ChainParameter";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.ChainParameters.{}", Self::NAME)
            }}
}
impl ::prost::Name for ChainParameters {
const NAME: &'static str = "ChainParameters";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
/// Account 
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// account nick name
    #[prost(bytes="vec", tag="1")]
    pub account_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="AccountType", tag="2")]
    pub r#type: i32,
    /// the create address
    #[prost(bytes="vec", tag="3")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    /// the trx balance
    #[prost(int64, tag="4")]
    pub balance: i64,
    /// the votes
    #[prost(message, repeated, tag="5")]
    pub votes: ::prost::alloc::vec::Vec<Vote>,
    /// the other asset owned by this account
    #[prost(map="string, int64", tag="6")]
    pub asset: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    /// the other asset owned by this account，key is assetId
    #[prost(map="string, int64", tag="56")]
    pub asset_v2: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    /// the frozen balance for bandwidth
    #[prost(message, repeated, tag="7")]
    pub frozen: ::prost::alloc::vec::Vec<account::Frozen>,
    /// bandwidth, get from frozen
    #[prost(int64, tag="8")]
    pub net_usage: i64,
    /// Frozen balance provided by other accounts to this account
    #[prost(int64, tag="41")]
    pub acquired_delegated_frozen_balance_for_bandwidth: i64,
    /// Freeze and provide balances to other accounts
    #[prost(int64, tag="42")]
    pub delegated_frozen_balance_for_bandwidth: i64,
    #[prost(int64, tag="46")]
    pub old_tron_power: i64,
    #[prost(message, optional, tag="47")]
    pub tron_power: ::core::option::Option<account::Frozen>,
    #[prost(bool, tag="60")]
    pub asset_optimized: bool,
    /// this account create time
    #[prost(int64, tag="9")]
    pub create_time: i64,
    /// this last operation time, including transfer, voting and so on. //FIXME fix grammar
    #[prost(int64, tag="10")]
    pub latest_opration_time: i64,
    /// witness block producing allowance
    #[prost(int64, tag="11")]
    pub allowance: i64,
    /// last withdraw time
    #[prost(int64, tag="12")]
    pub latest_withdraw_time: i64,
    /// not used so far
    #[prost(bytes="vec", tag="13")]
    pub code: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="14")]
    pub is_witness: bool,
    #[prost(bool, tag="15")]
    pub is_committee: bool,
    /// frozen asset(for asset issuer)
    #[prost(message, repeated, tag="16")]
    pub frozen_supply: ::prost::alloc::vec::Vec<account::Frozen>,
    /// asset_issued_name
    #[prost(bytes="vec", tag="17")]
    pub asset_issued_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="57")]
    pub asset_issued_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(map="string, int64", tag="18")]
    pub latest_asset_operation_time: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    #[prost(map="string, int64", tag="58")]
    pub latest_asset_operation_time_v2: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    #[prost(int64, tag="19")]
    pub free_net_usage: i64,
    #[prost(map="string, int64", tag="20")]
    pub free_asset_net_usage: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    #[prost(map="string, int64", tag="59")]
    pub free_asset_net_usage_v2: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    #[prost(int64, tag="21")]
    pub latest_consume_time: i64,
    #[prost(int64, tag="22")]
    pub latest_consume_free_time: i64,
    /// the identity of this account, case insensitive
    #[prost(bytes="vec", tag="23")]
    pub account_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="24")]
    pub net_window_size: i64,
    #[prost(bool, tag="25")]
    pub net_window_optimized: bool,
    #[prost(message, optional, tag="26")]
    pub account_resource: ::core::option::Option<account::AccountResource>,
    #[prost(bytes="vec", tag="30")]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="31")]
    pub owner_permission: ::core::option::Option<Permission>,
    #[prost(message, optional, tag="32")]
    pub witness_permission: ::core::option::Option<Permission>,
    #[prost(message, repeated, tag="33")]
    pub active_permission: ::prost::alloc::vec::Vec<Permission>,
    #[prost(message, repeated, tag="34")]
    pub frozen_v2: ::prost::alloc::vec::Vec<account::FreezeV2>,
    #[prost(message, repeated, tag="35")]
    pub unfrozen_v2: ::prost::alloc::vec::Vec<account::UnFreezeV2>,
    #[prost(int64, tag="36")]
    pub delegated_frozen_v2_balance_for_bandwidth: i64,
    #[prost(int64, tag="37")]
    pub acquired_delegated_frozen_v2_balance_for_bandwidth: i64,
}
/// Nested message and enum types in `Account`.
pub mod account {
    /// frozen balance 
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Frozen {
        /// the frozen trx balance
        #[prost(int64, tag="1")]
        pub frozen_balance: i64,
        /// the expire time
        #[prost(int64, tag="2")]
        pub expire_time: i64,
    }
impl ::prost::Name for Frozen {
const NAME: &'static str = "Frozen";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.Account.{}", Self::NAME)
            }}
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccountResource {
        /// energy resource, get from frozen
        #[prost(int64, tag="1")]
        pub energy_usage: i64,
        /// the frozen balance for energy
        #[prost(message, optional, tag="2")]
        pub frozen_balance_for_energy: ::core::option::Option<Frozen>,
        #[prost(int64, tag="3")]
        pub latest_consume_time_for_energy: i64,
        /// Frozen balance provided by other accounts to this account
        #[prost(int64, tag="4")]
        pub acquired_delegated_frozen_balance_for_energy: i64,
        /// Frozen balances provided to other accounts
        #[prost(int64, tag="5")]
        pub delegated_frozen_balance_for_energy: i64,
        /// storage resource, get from market
        #[prost(int64, tag="6")]
        pub storage_limit: i64,
        #[prost(int64, tag="7")]
        pub storage_usage: i64,
        #[prost(int64, tag="8")]
        pub latest_exchange_storage_time: i64,
        #[prost(int64, tag="9")]
        pub energy_window_size: i64,
        #[prost(int64, tag="10")]
        pub delegated_frozen_v2_balance_for_energy: i64,
        #[prost(int64, tag="11")]
        pub acquired_delegated_frozen_v2_balance_for_energy: i64,
        #[prost(bool, tag="12")]
        pub energy_window_optimized: bool,
    }
impl ::prost::Name for AccountResource {
const NAME: &'static str = "AccountResource";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.Account.{}", Self::NAME)
            }}
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FreezeV2 {
        #[prost(enumeration="super::ResourceCode", tag="1")]
        pub r#type: i32,
        #[prost(int64, tag="2")]
        pub amount: i64,
    }
impl ::prost::Name for FreezeV2 {
const NAME: &'static str = "FreezeV2";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.Account.{}", Self::NAME)
            }}
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnFreezeV2 {
        #[prost(enumeration="super::ResourceCode", tag="1")]
        pub r#type: i32,
        #[prost(int64, tag="3")]
        pub unfreeze_amount: i64,
        #[prost(int64, tag="4")]
        pub unfreeze_expire_time: i64,
    }
impl ::prost::Name for UnFreezeV2 {
const NAME: &'static str = "UnFreezeV2";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.Account.{}", Self::NAME)
            }}
}
impl ::prost::Name for Account {
const NAME: &'static str = "Account";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Key {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub weight: i64,
}
impl ::prost::Name for Key {
const NAME: &'static str = "Key";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatedResource {
    #[prost(bytes="vec", tag="1")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub frozen_balance_for_bandwidth: i64,
    #[prost(int64, tag="4")]
    pub frozen_balance_for_energy: i64,
    #[prost(int64, tag="5")]
    pub expire_time_for_bandwidth: i64,
    #[prost(int64, tag="6")]
    pub expire_time_for_energy: i64,
}
impl ::prost::Name for DelegatedResource {
const NAME: &'static str = "DelegatedResource";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authority {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<AccountId>,
    #[prost(bytes="vec", tag="2")]
    pub permission_name: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for Authority {
const NAME: &'static str = "authority";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Permission {
    #[prost(enumeration="permission::PermissionType", tag="1")]
    pub r#type: i32,
    /// Owner id=0, Witness id=1, Active id start by 2
    #[prost(int32, tag="2")]
    pub id: i32,
    #[prost(string, tag="3")]
    pub permission_name: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub threshold: i64,
    #[prost(int32, tag="5")]
    pub parent_id: i32,
    /// 1 bit 1 contract
    #[prost(bytes="vec", tag="6")]
    pub operations: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="7")]
    pub keys: ::prost::alloc::vec::Vec<Key>,
}
/// Nested message and enum types in `Permission`.
pub mod permission {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PermissionType {
        Owner = 0,
        Witness = 1,
        Active = 2,
    }
    impl PermissionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PermissionType::Owner => "Owner",
                PermissionType::Witness => "Witness",
                PermissionType::Active => "Active",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Owner" => Some(Self::Owner),
                "Witness" => Some(Self::Witness),
                "Active" => Some(Self::Active),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for Permission {
const NAME: &'static str = "Permission";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
/// Witness
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Witness {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub vote_count: i64,
    #[prost(bytes="vec", tag="3")]
    pub pub_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub url: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub total_produced: i64,
    #[prost(int64, tag="6")]
    pub total_missed: i64,
    #[prost(int64, tag="7")]
    pub latest_block_num: i64,
    #[prost(int64, tag="8")]
    pub latest_slot_num: i64,
    #[prost(bool, tag="9")]
    pub is_jobs: bool,
}
impl ::prost::Name for Witness {
const NAME: &'static str = "Witness";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
/// Vote Change
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Votes {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="2")]
    pub old_votes: ::prost::alloc::vec::Vec<Vote>,
    #[prost(message, repeated, tag="3")]
    pub new_votes: ::prost::alloc::vec::Vec<Vote>,
}
impl ::prost::Name for Votes {
const NAME: &'static str = "Votes";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
// Transcation

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxOutput {
    #[prost(int64, tag="1")]
    pub value: i64,
    #[prost(bytes="vec", tag="2")]
    pub pub_key_hash: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for TxOutput {
const NAME: &'static str = "TXOutput";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxInput {
    #[prost(message, optional, tag="1")]
    pub raw_data: ::core::option::Option<tx_input::Raw>,
    #[prost(bytes="vec", tag="4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `TXInput`.
pub mod tx_input {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Raw {
        #[prost(bytes="vec", tag="1")]
        pub tx_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="2")]
        pub vout: i64,
        #[prost(bytes="vec", tag="3")]
        pub pub_key: ::prost::alloc::vec::Vec<u8>,
    }
impl ::prost::Name for Raw {
const NAME: &'static str = "raw";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.TXInput.{}", Self::NAME)
            }}
}
impl ::prost::Name for TxInput {
const NAME: &'static str = "TXInput";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxOutputs {
    #[prost(message, repeated, tag="1")]
    pub outputs: ::prost::alloc::vec::Vec<TxOutput>,
}
impl ::prost::Name for TxOutputs {
const NAME: &'static str = "TXOutputs";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceReceipt {
    #[prost(int64, tag="1")]
    pub energy_usage: i64,
    #[prost(int64, tag="2")]
    pub energy_fee: i64,
    #[prost(int64, tag="3")]
    pub origin_energy_usage: i64,
    #[prost(int64, tag="4")]
    pub energy_usage_total: i64,
    #[prost(int64, tag="5")]
    pub net_usage: i64,
    #[prost(int64, tag="6")]
    pub net_fee: i64,
    #[prost(enumeration="transaction::result::ContractResult", tag="7")]
    pub result: i32,
    #[prost(int64, tag="8")]
    pub energy_penalty_total: i64,
}
impl ::prost::Name for ResourceReceipt {
const NAME: &'static str = "ResourceReceipt";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrderDetail {
    #[prost(bytes="vec", tag="1")]
    pub maker_order_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub taker_order_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub fill_sell_quantity: i64,
    #[prost(int64, tag="4")]
    pub fill_buy_quantity: i64,
}
impl ::prost::Name for MarketOrderDetail {
const NAME: &'static str = "MarketOrderDetail";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(message, optional, tag="1")]
    pub raw_data: ::core::option::Option<transaction::Raw>,
    /// only support size = 1,  repeated list here for muti-sig extension
    #[prost(bytes="vec", repeated, tag="2")]
    pub signature: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag="5")]
    pub ret: ::prost::alloc::vec::Vec<transaction::Result>,
}
/// Nested message and enum types in `Transaction`.
pub mod transaction {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Contract {
        #[prost(enumeration="contract::ContractType", tag="1")]
        pub r#type: i32,
        #[prost(message, optional, tag="2")]
        pub parameter: ::core::option::Option<::prost_types::Any>,
        #[prost(bytes="vec", tag="3")]
        pub provider: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes="vec", tag="4")]
        pub contract_name: ::prost::alloc::vec::Vec<u8>,
        #[prost(int32, tag="5")]
        pub permission_id: i32,
    }
    /// Nested message and enum types in `Contract`.
    pub mod contract {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ContractType {
            AccountCreateContract = 0,
            TransferContract = 1,
            TransferAssetContract = 2,
            VoteAssetContract = 3,
            VoteWitnessContract = 4,
            WitnessCreateContract = 5,
            AssetIssueContract = 6,
            WitnessUpdateContract = 8,
            ParticipateAssetIssueContract = 9,
            AccountUpdateContract = 10,
            FreezeBalanceContract = 11,
            UnfreezeBalanceContract = 12,
            WithdrawBalanceContract = 13,
            UnfreezeAssetContract = 14,
            UpdateAssetContract = 15,
            ProposalCreateContract = 16,
            ProposalApproveContract = 17,
            ProposalDeleteContract = 18,
            SetAccountIdContract = 19,
            CustomContract = 20,
            CreateSmartContract = 30,
            TriggerSmartContract = 31,
            GetContract = 32,
            UpdateSettingContract = 33,
            ExchangeCreateContract = 41,
            ExchangeInjectContract = 42,
            ExchangeWithdrawContract = 43,
            ExchangeTransactionContract = 44,
            UpdateEnergyLimitContract = 45,
            AccountPermissionUpdateContract = 46,
            ClearAbiContract = 48,
            UpdateBrokerageContract = 49,
            ShieldedTransferContract = 51,
            MarketSellAssetContract = 52,
            MarketCancelOrderContract = 53,
            FreezeBalanceV2Contract = 54,
            UnfreezeBalanceV2Contract = 55,
            WithdrawExpireUnfreezeContract = 56,
            DelegateResourceContract = 57,
            UnDelegateResourceContract = 58,
            CancelAllUnfreezeV2Contract = 59,
        }
        impl ContractType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ContractType::AccountCreateContract => "AccountCreateContract",
                    ContractType::TransferContract => "TransferContract",
                    ContractType::TransferAssetContract => "TransferAssetContract",
                    ContractType::VoteAssetContract => "VoteAssetContract",
                    ContractType::VoteWitnessContract => "VoteWitnessContract",
                    ContractType::WitnessCreateContract => "WitnessCreateContract",
                    ContractType::AssetIssueContract => "AssetIssueContract",
                    ContractType::WitnessUpdateContract => "WitnessUpdateContract",
                    ContractType::ParticipateAssetIssueContract => "ParticipateAssetIssueContract",
                    ContractType::AccountUpdateContract => "AccountUpdateContract",
                    ContractType::FreezeBalanceContract => "FreezeBalanceContract",
                    ContractType::UnfreezeBalanceContract => "UnfreezeBalanceContract",
                    ContractType::WithdrawBalanceContract => "WithdrawBalanceContract",
                    ContractType::UnfreezeAssetContract => "UnfreezeAssetContract",
                    ContractType::UpdateAssetContract => "UpdateAssetContract",
                    ContractType::ProposalCreateContract => "ProposalCreateContract",
                    ContractType::ProposalApproveContract => "ProposalApproveContract",
                    ContractType::ProposalDeleteContract => "ProposalDeleteContract",
                    ContractType::SetAccountIdContract => "SetAccountIdContract",
                    ContractType::CustomContract => "CustomContract",
                    ContractType::CreateSmartContract => "CreateSmartContract",
                    ContractType::TriggerSmartContract => "TriggerSmartContract",
                    ContractType::GetContract => "GetContract",
                    ContractType::UpdateSettingContract => "UpdateSettingContract",
                    ContractType::ExchangeCreateContract => "ExchangeCreateContract",
                    ContractType::ExchangeInjectContract => "ExchangeInjectContract",
                    ContractType::ExchangeWithdrawContract => "ExchangeWithdrawContract",
                    ContractType::ExchangeTransactionContract => "ExchangeTransactionContract",
                    ContractType::UpdateEnergyLimitContract => "UpdateEnergyLimitContract",
                    ContractType::AccountPermissionUpdateContract => "AccountPermissionUpdateContract",
                    ContractType::ClearAbiContract => "ClearABIContract",
                    ContractType::UpdateBrokerageContract => "UpdateBrokerageContract",
                    ContractType::ShieldedTransferContract => "ShieldedTransferContract",
                    ContractType::MarketSellAssetContract => "MarketSellAssetContract",
                    ContractType::MarketCancelOrderContract => "MarketCancelOrderContract",
                    ContractType::FreezeBalanceV2Contract => "FreezeBalanceV2Contract",
                    ContractType::UnfreezeBalanceV2Contract => "UnfreezeBalanceV2Contract",
                    ContractType::WithdrawExpireUnfreezeContract => "WithdrawExpireUnfreezeContract",
                    ContractType::DelegateResourceContract => "DelegateResourceContract",
                    ContractType::UnDelegateResourceContract => "UnDelegateResourceContract",
                    ContractType::CancelAllUnfreezeV2Contract => "CancelAllUnfreezeV2Contract",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "AccountCreateContract" => Some(Self::AccountCreateContract),
                    "TransferContract" => Some(Self::TransferContract),
                    "TransferAssetContract" => Some(Self::TransferAssetContract),
                    "VoteAssetContract" => Some(Self::VoteAssetContract),
                    "VoteWitnessContract" => Some(Self::VoteWitnessContract),
                    "WitnessCreateContract" => Some(Self::WitnessCreateContract),
                    "AssetIssueContract" => Some(Self::AssetIssueContract),
                    "WitnessUpdateContract" => Some(Self::WitnessUpdateContract),
                    "ParticipateAssetIssueContract" => Some(Self::ParticipateAssetIssueContract),
                    "AccountUpdateContract" => Some(Self::AccountUpdateContract),
                    "FreezeBalanceContract" => Some(Self::FreezeBalanceContract),
                    "UnfreezeBalanceContract" => Some(Self::UnfreezeBalanceContract),
                    "WithdrawBalanceContract" => Some(Self::WithdrawBalanceContract),
                    "UnfreezeAssetContract" => Some(Self::UnfreezeAssetContract),
                    "UpdateAssetContract" => Some(Self::UpdateAssetContract),
                    "ProposalCreateContract" => Some(Self::ProposalCreateContract),
                    "ProposalApproveContract" => Some(Self::ProposalApproveContract),
                    "ProposalDeleteContract" => Some(Self::ProposalDeleteContract),
                    "SetAccountIdContract" => Some(Self::SetAccountIdContract),
                    "CustomContract" => Some(Self::CustomContract),
                    "CreateSmartContract" => Some(Self::CreateSmartContract),
                    "TriggerSmartContract" => Some(Self::TriggerSmartContract),
                    "GetContract" => Some(Self::GetContract),
                    "UpdateSettingContract" => Some(Self::UpdateSettingContract),
                    "ExchangeCreateContract" => Some(Self::ExchangeCreateContract),
                    "ExchangeInjectContract" => Some(Self::ExchangeInjectContract),
                    "ExchangeWithdrawContract" => Some(Self::ExchangeWithdrawContract),
                    "ExchangeTransactionContract" => Some(Self::ExchangeTransactionContract),
                    "UpdateEnergyLimitContract" => Some(Self::UpdateEnergyLimitContract),
                    "AccountPermissionUpdateContract" => Some(Self::AccountPermissionUpdateContract),
                    "ClearABIContract" => Some(Self::ClearAbiContract),
                    "UpdateBrokerageContract" => Some(Self::UpdateBrokerageContract),
                    "ShieldedTransferContract" => Some(Self::ShieldedTransferContract),
                    "MarketSellAssetContract" => Some(Self::MarketSellAssetContract),
                    "MarketCancelOrderContract" => Some(Self::MarketCancelOrderContract),
                    "FreezeBalanceV2Contract" => Some(Self::FreezeBalanceV2Contract),
                    "UnfreezeBalanceV2Contract" => Some(Self::UnfreezeBalanceV2Contract),
                    "WithdrawExpireUnfreezeContract" => Some(Self::WithdrawExpireUnfreezeContract),
                    "DelegateResourceContract" => Some(Self::DelegateResourceContract),
                    "UnDelegateResourceContract" => Some(Self::UnDelegateResourceContract),
                    "CancelAllUnfreezeV2Contract" => Some(Self::CancelAllUnfreezeV2Contract),
                    _ => None,
                }
            }
        }
    }
impl ::prost::Name for Contract {
const NAME: &'static str = "Contract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.Transaction.{}", Self::NAME)
            }}
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        #[prost(int64, tag="1")]
        pub fee: i64,
        #[prost(enumeration="result::Code", tag="2")]
        pub ret: i32,
        #[prost(enumeration="result::ContractResult", tag="3")]
        pub contract_ret: i32,
        #[prost(string, tag="14")]
        pub asset_issue_id: ::prost::alloc::string::String,
        #[prost(int64, tag="15")]
        pub withdraw_amount: i64,
        #[prost(int64, tag="16")]
        pub unfreeze_amount: i64,
        #[prost(int64, tag="18")]
        pub exchange_received_amount: i64,
        #[prost(int64, tag="19")]
        pub exchange_inject_another_amount: i64,
        #[prost(int64, tag="20")]
        pub exchange_withdraw_another_amount: i64,
        #[prost(int64, tag="21")]
        pub exchange_id: i64,
        #[prost(int64, tag="22")]
        pub shielded_transaction_fee: i64,
        #[prost(bytes="vec", tag="25")]
        pub order_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(message, repeated, tag="26")]
        pub order_details: ::prost::alloc::vec::Vec<super::MarketOrderDetail>,
        #[prost(int64, tag="27")]
        pub withdraw_expire_amount: i64,
        #[prost(map="string, int64", tag="28")]
        pub cancel_unfreeze_v2_amount: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    }
    /// Nested message and enum types in `Result`.
    pub mod result {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Code {
            Sucess = 0,
            Failed = 1,
        }
        impl Code {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Code::Sucess => "SUCESS",
                    Code::Failed => "FAILED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "SUCESS" => Some(Self::Sucess),
                    "FAILED" => Some(Self::Failed),
                    _ => None,
                }
            }
        }
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ContractResult {
            Default = 0,
            Success = 1,
            Revert = 2,
            BadJumpDestination = 3,
            OutOfMemory = 4,
            PrecompiledContract = 5,
            StackTooSmall = 6,
            StackTooLarge = 7,
            IllegalOperation = 8,
            StackOverflow = 9,
            OutOfEnergy = 10,
            OutOfTime = 11,
            JvmStackOverFlow = 12,
            Unknown = 13,
            TransferFailed = 14,
            InvalidCode = 15,
        }
        impl ContractResult {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ContractResult::Default => "DEFAULT",
                    ContractResult::Success => "SUCCESS",
                    ContractResult::Revert => "REVERT",
                    ContractResult::BadJumpDestination => "BAD_JUMP_DESTINATION",
                    ContractResult::OutOfMemory => "OUT_OF_MEMORY",
                    ContractResult::PrecompiledContract => "PRECOMPILED_CONTRACT",
                    ContractResult::StackTooSmall => "STACK_TOO_SMALL",
                    ContractResult::StackTooLarge => "STACK_TOO_LARGE",
                    ContractResult::IllegalOperation => "ILLEGAL_OPERATION",
                    ContractResult::StackOverflow => "STACK_OVERFLOW",
                    ContractResult::OutOfEnergy => "OUT_OF_ENERGY",
                    ContractResult::OutOfTime => "OUT_OF_TIME",
                    ContractResult::JvmStackOverFlow => "JVM_STACK_OVER_FLOW",
                    ContractResult::Unknown => "UNKNOWN",
                    ContractResult::TransferFailed => "TRANSFER_FAILED",
                    ContractResult::InvalidCode => "INVALID_CODE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "DEFAULT" => Some(Self::Default),
                    "SUCCESS" => Some(Self::Success),
                    "REVERT" => Some(Self::Revert),
                    "BAD_JUMP_DESTINATION" => Some(Self::BadJumpDestination),
                    "OUT_OF_MEMORY" => Some(Self::OutOfMemory),
                    "PRECOMPILED_CONTRACT" => Some(Self::PrecompiledContract),
                    "STACK_TOO_SMALL" => Some(Self::StackTooSmall),
                    "STACK_TOO_LARGE" => Some(Self::StackTooLarge),
                    "ILLEGAL_OPERATION" => Some(Self::IllegalOperation),
                    "STACK_OVERFLOW" => Some(Self::StackOverflow),
                    "OUT_OF_ENERGY" => Some(Self::OutOfEnergy),
                    "OUT_OF_TIME" => Some(Self::OutOfTime),
                    "JVM_STACK_OVER_FLOW" => Some(Self::JvmStackOverFlow),
                    "UNKNOWN" => Some(Self::Unknown),
                    "TRANSFER_FAILED" => Some(Self::TransferFailed),
                    "INVALID_CODE" => Some(Self::InvalidCode),
                    _ => None,
                }
            }
        }
    }
impl ::prost::Name for Result {
const NAME: &'static str = "Result";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.Transaction.{}", Self::NAME)
            }}
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Raw {
        #[prost(bytes="vec", tag="1")]
        pub ref_block_bytes: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="3")]
        pub ref_block_num: i64,
        #[prost(bytes="vec", tag="4")]
        pub ref_block_hash: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="8")]
        pub expiration: i64,
        #[prost(message, repeated, tag="9")]
        pub auths: ::prost::alloc::vec::Vec<super::Authority>,
        /// data not used
        #[prost(bytes="vec", tag="10")]
        pub data: ::prost::alloc::vec::Vec<u8>,
        /// only support size = 1,  repeated list here for extension
        #[prost(message, repeated, tag="11")]
        pub contract: ::prost::alloc::vec::Vec<Contract>,
        /// scripts not used
        #[prost(bytes="vec", tag="12")]
        pub scripts: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="14")]
        pub timestamp: i64,
        #[prost(int64, tag="18")]
        pub fee_limit: i64,
    }
impl ::prost::Name for Raw {
const NAME: &'static str = "raw";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.Transaction.{}", Self::NAME)
            }}
}
impl ::prost::Name for Transaction {
const NAME: &'static str = "Transaction";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfo {
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub fee: i64,
    #[prost(int64, tag="3")]
    pub block_number: i64,
    #[prost(int64, tag="4")]
    pub block_time_stamp: i64,
    #[prost(bytes="vec", repeated, tag="5")]
    pub contract_result: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="6")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="7")]
    pub receipt: ::core::option::Option<ResourceReceipt>,
    #[prost(message, repeated, tag="8")]
    pub log: ::prost::alloc::vec::Vec<transaction_info::Log>,
    #[prost(enumeration="transaction_info::Code", tag="9")]
    pub result: i32,
    #[prost(bytes="vec", tag="10")]
    pub res_message: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="14")]
    pub asset_issue_id: ::prost::alloc::string::String,
    #[prost(int64, tag="15")]
    pub withdraw_amount: i64,
    #[prost(int64, tag="16")]
    pub unfreeze_amount: i64,
    #[prost(message, repeated, tag="17")]
    pub internal_transactions: ::prost::alloc::vec::Vec<InternalTransaction>,
    #[prost(int64, tag="18")]
    pub exchange_received_amount: i64,
    #[prost(int64, tag="19")]
    pub exchange_inject_another_amount: i64,
    #[prost(int64, tag="20")]
    pub exchange_withdraw_another_amount: i64,
    #[prost(int64, tag="21")]
    pub exchange_id: i64,
    #[prost(int64, tag="22")]
    pub shielded_transaction_fee: i64,
    #[prost(bytes="vec", tag="25")]
    pub order_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="26")]
    pub order_details: ::prost::alloc::vec::Vec<MarketOrderDetail>,
    #[prost(int64, tag="27")]
    pub packing_fee: i64,
    #[prost(int64, tag="28")]
    pub withdraw_expire_amount: i64,
    #[prost(map="string, int64", tag="29")]
    pub cancel_unfreeze_v2_amount: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
}
/// Nested message and enum types in `TransactionInfo`.
pub mod transaction_info {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Log {
        #[prost(bytes="vec", tag="1")]
        pub address: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes="vec", repeated, tag="2")]
        pub topics: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
        #[prost(bytes="vec", tag="3")]
        pub data: ::prost::alloc::vec::Vec<u8>,
    }
impl ::prost::Name for Log {
const NAME: &'static str = "Log";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.TransactionInfo.{}", Self::NAME)
            }}
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        Sucess = 0,
        Failed = 1,
    }
    impl Code {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Code::Sucess => "SUCESS",
                Code::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUCESS" => Some(Self::Sucess),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for TransactionInfo {
const NAME: &'static str = "TransactionInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionRet {
    #[prost(int64, tag="1")]
    pub block_number: i64,
    #[prost(int64, tag="2")]
    pub block_time_stamp: i64,
    #[prost(message, repeated, tag="3")]
    pub transactioninfo: ::prost::alloc::vec::Vec<TransactionInfo>,
}
impl ::prost::Name for TransactionRet {
const NAME: &'static str = "TransactionRet";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transactions {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
impl ::prost::Name for Transactions {
const NAME: &'static str = "Transactions";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    #[prost(message, optional, tag="1")]
    pub raw_data: ::core::option::Option<block_header::Raw>,
    #[prost(bytes="vec", tag="2")]
    pub witness_signature: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `BlockHeader`.
pub mod block_header {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Raw {
        #[prost(int64, tag="1")]
        pub timestamp: i64,
        #[prost(bytes="vec", tag="2")]
        pub tx_trie_root: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes="vec", tag="3")]
        pub parent_hash: ::prost::alloc::vec::Vec<u8>,
        /// bytes nonce = 5;
        /// bytes difficulty = 6;
        #[prost(int64, tag="7")]
        pub number: i64,
        #[prost(int64, tag="8")]
        pub witness_id: i64,
        #[prost(bytes="vec", tag="9")]
        pub witness_address: ::prost::alloc::vec::Vec<u8>,
        #[prost(int32, tag="10")]
        pub version: i32,
        #[prost(bytes="vec", tag="11")]
        pub account_state_root: ::prost::alloc::vec::Vec<u8>,
    }
impl ::prost::Name for Raw {
const NAME: &'static str = "raw";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.BlockHeader.{}", Self::NAME)
            }}
}
impl ::prost::Name for BlockHeader {
const NAME: &'static str = "BlockHeader";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
/// block
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
    #[prost(message, optional, tag="2")]
    pub block_header: ::core::option::Option<BlockHeader>,
}
impl ::prost::Name for Block {
const NAME: &'static str = "Block";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainInventory {
    #[prost(message, repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<chain_inventory::BlockId>,
    #[prost(int64, tag="2")]
    pub remain_num: i64,
}
/// Nested message and enum types in `ChainInventory`.
pub mod chain_inventory {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlockId {
        #[prost(bytes="vec", tag="1")]
        pub hash: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="2")]
        pub number: i64,
    }
impl ::prost::Name for BlockId {
const NAME: &'static str = "BlockId";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.ChainInventory.{}", Self::NAME)
            }}
}
impl ::prost::Name for ChainInventory {
const NAME: &'static str = "ChainInventory";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
/// Inventory
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockInventory {
    #[prost(message, repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<block_inventory::BlockId>,
    #[prost(enumeration="block_inventory::Type", tag="2")]
    pub r#type: i32,
}
/// Nested message and enum types in `BlockInventory`.
pub mod block_inventory {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlockId {
        #[prost(bytes="vec", tag="1")]
        pub hash: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="2")]
        pub number: i64,
    }
impl ::prost::Name for BlockId {
const NAME: &'static str = "BlockId";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.BlockInventory.{}", Self::NAME)
            }}
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Sync = 0,
        Advtise = 1,
        Fetch = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Sync => "SYNC",
                Type::Advtise => "ADVTISE",
                Type::Fetch => "FETCH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SYNC" => Some(Self::Sync),
                "ADVTISE" => Some(Self::Advtise),
                "FETCH" => Some(Self::Fetch),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for BlockInventory {
const NAME: &'static str = "BlockInventory";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inventory {
    #[prost(enumeration="inventory::InventoryType", tag="1")]
    pub r#type: i32,
    #[prost(bytes="vec", repeated, tag="2")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `Inventory`.
pub mod inventory {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InventoryType {
        Trx = 0,
        Block = 1,
    }
    impl InventoryType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InventoryType::Trx => "TRX",
                InventoryType::Block => "BLOCK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TRX" => Some(Self::Trx),
                "BLOCK" => Some(Self::Block),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for Inventory {
const NAME: &'static str = "Inventory";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Items {
    #[prost(enumeration="items::ItemType", tag="1")]
    pub r#type: i32,
    #[prost(message, repeated, tag="2")]
    pub blocks: ::prost::alloc::vec::Vec<Block>,
    #[prost(message, repeated, tag="3")]
    pub block_headers: ::prost::alloc::vec::Vec<BlockHeader>,
    #[prost(message, repeated, tag="4")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
/// Nested message and enum types in `Items`.
pub mod items {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ItemType {
        Err = 0,
        Trx = 1,
        Block = 2,
        Blockheader = 3,
    }
    impl ItemType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ItemType::Err => "ERR",
                ItemType::Trx => "TRX",
                ItemType::Block => "BLOCK",
                ItemType::Blockheader => "BLOCKHEADER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ERR" => Some(Self::Err),
                "TRX" => Some(Self::Trx),
                "BLOCK" => Some(Self::Block),
                "BLOCKHEADER" => Some(Self::Blockheader),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for Items {
const NAME: &'static str = "Items";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
/// DynamicProperties
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicProperties {
    #[prost(int64, tag="1")]
    pub last_solidity_block_num: i64,
}
impl ::prost::Name for DynamicProperties {
const NAME: &'static str = "DynamicProperties";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectMessage {
    #[prost(enumeration="ReasonCode", tag="1")]
    pub reason: i32,
}
impl ::prost::Name for DisconnectMessage {
const NAME: &'static str = "DisconnectMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloMessage {
    #[prost(message, optional, tag="1")]
    pub from: ::core::option::Option<Endpoint>,
    #[prost(int32, tag="2")]
    pub version: i32,
    #[prost(int64, tag="3")]
    pub timestamp: i64,
    #[prost(message, optional, tag="4")]
    pub genesis_block_id: ::core::option::Option<hello_message::BlockId>,
    #[prost(message, optional, tag="5")]
    pub solid_block_id: ::core::option::Option<hello_message::BlockId>,
    #[prost(message, optional, tag="6")]
    pub head_block_id: ::core::option::Option<hello_message::BlockId>,
    #[prost(bytes="vec", tag="7")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="9")]
    pub node_type: i32,
    #[prost(int64, tag="10")]
    pub lowest_block_num: i64,
    #[prost(bytes="vec", tag="11")]
    pub code_version: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `HelloMessage`.
pub mod hello_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlockId {
        #[prost(bytes="vec", tag="1")]
        pub hash: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="2")]
        pub number: i64,
    }
impl ::prost::Name for BlockId {
const NAME: &'static str = "BlockId";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.HelloMessage.{}", Self::NAME)
            }}
}
impl ::prost::Name for HelloMessage {
const NAME: &'static str = "HelloMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalTransaction {
    /// internalTransaction identity, the root InternalTransaction hash
    /// should equals to root transaction id.
    #[prost(bytes="vec", tag="1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// the one send trx (TBD: or token) via function
    #[prost(bytes="vec", tag="2")]
    pub caller_address: ::prost::alloc::vec::Vec<u8>,
    /// the one recieve trx (TBD: or token) via function
    #[prost(bytes="vec", tag="3")]
    pub transfer_to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="4")]
    pub call_value_info: ::prost::alloc::vec::Vec<internal_transaction::CallValueInfo>,
    #[prost(bytes="vec", tag="5")]
    pub note: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="6")]
    pub rejected: bool,
    #[prost(string, tag="7")]
    pub extra: ::prost::alloc::string::String,
}
/// Nested message and enum types in `InternalTransaction`.
pub mod internal_transaction {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CallValueInfo {
        /// trx (TBD: or token) value
        #[prost(int64, tag="1")]
        pub call_value: i64,
        /// TBD: tokenName, trx should be empty
        #[prost(string, tag="2")]
        pub token_id: ::prost::alloc::string::String,
    }
impl ::prost::Name for CallValueInfo {
const NAME: &'static str = "CallValueInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.InternalTransaction.{}", Self::NAME)
            }}
}
impl ::prost::Name for InternalTransaction {
const NAME: &'static str = "InternalTransaction";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatedResourceAccountIndex {
    #[prost(bytes="vec", tag="1")]
    pub account: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub from_accounts: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub to_accounts: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(int64, tag="4")]
    pub timestamp: i64,
}
impl ::prost::Name for DelegatedResourceAccountIndex {
const NAME: &'static str = "DelegatedResourceAccountIndex";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInfo {
    #[prost(int64, tag="1")]
    pub begin_sync_num: i64,
    #[prost(string, tag="2")]
    pub block: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub solidity_block: ::prost::alloc::string::String,
    /// connect information
    #[prost(int32, tag="4")]
    pub current_connect_count: i32,
    #[prost(int32, tag="5")]
    pub active_connect_count: i32,
    #[prost(int32, tag="6")]
    pub passive_connect_count: i32,
    #[prost(int64, tag="7")]
    pub total_flow: i64,
    #[prost(message, repeated, tag="8")]
    pub peer_info_list: ::prost::alloc::vec::Vec<node_info::PeerInfo>,
    #[prost(message, optional, tag="9")]
    pub config_node_info: ::core::option::Option<node_info::ConfigNodeInfo>,
    #[prost(message, optional, tag="10")]
    pub machine_info: ::core::option::Option<node_info::MachineInfo>,
    #[prost(map="string, string", tag="11")]
    pub cheat_witness_info_map: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Nested message and enum types in `NodeInfo`.
pub mod node_info {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PeerInfo {
        #[prost(string, tag="1")]
        pub last_sync_block: ::prost::alloc::string::String,
        #[prost(int64, tag="2")]
        pub remain_num: i64,
        #[prost(int64, tag="3")]
        pub last_block_update_time: i64,
        #[prost(bool, tag="4")]
        pub sync_flag: bool,
        #[prost(int64, tag="5")]
        pub head_block_time_we_both_have: i64,
        #[prost(bool, tag="6")]
        pub need_sync_from_peer: bool,
        #[prost(bool, tag="7")]
        pub need_sync_from_us: bool,
        #[prost(string, tag="8")]
        pub host: ::prost::alloc::string::String,
        #[prost(int32, tag="9")]
        pub port: i32,
        #[prost(string, tag="10")]
        pub node_id: ::prost::alloc::string::String,
        #[prost(int64, tag="11")]
        pub connect_time: i64,
        #[prost(double, tag="12")]
        pub avg_latency: f64,
        #[prost(int32, tag="13")]
        pub sync_to_fetch_size: i32,
        #[prost(int64, tag="14")]
        pub sync_to_fetch_size_peek_num: i64,
        #[prost(int32, tag="15")]
        pub sync_block_requested_size: i32,
        #[prost(int64, tag="16")]
        pub un_fetch_syn_num: i64,
        #[prost(int32, tag="17")]
        pub block_in_porc_size: i32,
        #[prost(string, tag="18")]
        pub head_block_we_both_have: ::prost::alloc::string::String,
        #[prost(bool, tag="19")]
        pub is_active: bool,
        #[prost(int32, tag="20")]
        pub score: i32,
        #[prost(int32, tag="21")]
        pub node_count: i32,
        #[prost(int64, tag="22")]
        pub in_flow: i64,
        #[prost(int32, tag="23")]
        pub disconnect_times: i32,
        #[prost(string, tag="24")]
        pub local_disconnect_reason: ::prost::alloc::string::String,
        #[prost(string, tag="25")]
        pub remote_disconnect_reason: ::prost::alloc::string::String,
    }
impl ::prost::Name for PeerInfo {
const NAME: &'static str = "PeerInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.NodeInfo.{}", Self::NAME)
            }}
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConfigNodeInfo {
        #[prost(string, tag="1")]
        pub code_version: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub p2p_version: ::prost::alloc::string::String,
        #[prost(int32, tag="3")]
        pub listen_port: i32,
        #[prost(bool, tag="4")]
        pub discover_enable: bool,
        #[prost(int32, tag="5")]
        pub active_node_size: i32,
        #[prost(int32, tag="6")]
        pub passive_node_size: i32,
        #[prost(int32, tag="7")]
        pub send_node_size: i32,
        #[prost(int32, tag="8")]
        pub max_connect_count: i32,
        #[prost(int32, tag="9")]
        pub same_ip_max_connect_count: i32,
        #[prost(int32, tag="10")]
        pub backup_listen_port: i32,
        #[prost(int32, tag="11")]
        pub backup_member_size: i32,
        #[prost(int32, tag="12")]
        pub backup_priority: i32,
        #[prost(int32, tag="13")]
        pub db_version: i32,
        #[prost(int32, tag="14")]
        pub min_participation_rate: i32,
        #[prost(bool, tag="15")]
        pub support_constant: bool,
        #[prost(double, tag="16")]
        pub min_time_ratio: f64,
        #[prost(double, tag="17")]
        pub max_time_ratio: f64,
        #[prost(int64, tag="18")]
        pub allow_creation_of_contracts: i64,
        #[prost(int64, tag="19")]
        pub allow_adaptive_energy: i64,
    }
impl ::prost::Name for ConfigNodeInfo {
const NAME: &'static str = "ConfigNodeInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.NodeInfo.{}", Self::NAME)
            }}
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MachineInfo {
        #[prost(int32, tag="1")]
        pub thread_count: i32,
        #[prost(int32, tag="2")]
        pub dead_lock_thread_count: i32,
        #[prost(int32, tag="3")]
        pub cpu_count: i32,
        #[prost(int64, tag="4")]
        pub total_memory: i64,
        #[prost(int64, tag="5")]
        pub free_memory: i64,
        #[prost(double, tag="6")]
        pub cpu_rate: f64,
        #[prost(string, tag="7")]
        pub java_version: ::prost::alloc::string::String,
        #[prost(string, tag="8")]
        pub os_name: ::prost::alloc::string::String,
        #[prost(int64, tag="9")]
        pub jvm_total_memory: i64,
        #[prost(int64, tag="10")]
        pub jvm_free_memory: i64,
        #[prost(double, tag="11")]
        pub process_cpu_rate: f64,
        #[prost(message, repeated, tag="12")]
        pub memory_desc_info_list: ::prost::alloc::vec::Vec<machine_info::MemoryDescInfo>,
        #[prost(message, repeated, tag="13")]
        pub dead_lock_thread_info_list: ::prost::alloc::vec::Vec<machine_info::DeadLockThreadInfo>,
    }
    /// Nested message and enum types in `MachineInfo`.
    pub mod machine_info {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MemoryDescInfo {
            #[prost(string, tag="1")]
            pub name: ::prost::alloc::string::String,
            #[prost(int64, tag="2")]
            pub init_size: i64,
            #[prost(int64, tag="3")]
            pub use_size: i64,
            #[prost(int64, tag="4")]
            pub max_size: i64,
            #[prost(double, tag="5")]
            pub use_rate: f64,
        }
impl ::prost::Name for MemoryDescInfo {
const NAME: &'static str = "MemoryDescInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.NodeInfo.MachineInfo.{}", Self::NAME)
            }}
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DeadLockThreadInfo {
            #[prost(string, tag="1")]
            pub name: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub lock_name: ::prost::alloc::string::String,
            #[prost(string, tag="3")]
            pub lock_owner: ::prost::alloc::string::String,
            #[prost(string, tag="4")]
            pub state: ::prost::alloc::string::String,
            #[prost(int64, tag="5")]
            pub block_time: i64,
            #[prost(int64, tag="6")]
            pub wait_time: i64,
            #[prost(string, tag="7")]
            pub stack_trace: ::prost::alloc::string::String,
        }
impl ::prost::Name for DeadLockThreadInfo {
const NAME: &'static str = "DeadLockThreadInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.NodeInfo.MachineInfo.{}", Self::NAME)
            }}
    }
impl ::prost::Name for MachineInfo {
const NAME: &'static str = "MachineInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.NodeInfo.{}", Self::NAME)
            }}
}
impl ::prost::Name for NodeInfo {
const NAME: &'static str = "NodeInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricsInfo {
    #[prost(int64, tag="1")]
    pub interval: i64,
    #[prost(message, optional, tag="2")]
    pub node: ::core::option::Option<metrics_info::NodeInfo>,
    #[prost(message, optional, tag="3")]
    pub blockchain: ::core::option::Option<metrics_info::BlockChainInfo>,
    #[prost(message, optional, tag="4")]
    pub net: ::core::option::Option<metrics_info::NetInfo>,
}
/// Nested message and enum types in `MetricsInfo`.
pub mod metrics_info {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NodeInfo {
        #[prost(string, tag="1")]
        pub ip: ::prost::alloc::string::String,
        #[prost(int32, tag="2")]
        pub node_type: i32,
        #[prost(string, tag="3")]
        pub version: ::prost::alloc::string::String,
        #[prost(int32, tag="4")]
        pub backup_status: i32,
    }
impl ::prost::Name for NodeInfo {
const NAME: &'static str = "NodeInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.MetricsInfo.{}", Self::NAME)
            }}
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlockChainInfo {
        #[prost(int64, tag="1")]
        pub head_block_num: i64,
        #[prost(int64, tag="2")]
        pub head_block_timestamp: i64,
        #[prost(string, tag="3")]
        pub head_block_hash: ::prost::alloc::string::String,
        #[prost(int32, tag="4")]
        pub fork_count: i32,
        #[prost(int32, tag="5")]
        pub fail_fork_count: i32,
        #[prost(message, optional, tag="6")]
        pub block_process_time: ::core::option::Option<RateInfo>,
        #[prost(message, optional, tag="7")]
        pub tps: ::core::option::Option<RateInfo>,
        #[prost(int32, tag="8")]
        pub transaction_cache_size: i32,
        #[prost(message, optional, tag="9")]
        pub missed_transaction: ::core::option::Option<RateInfo>,
        #[prost(message, repeated, tag="10")]
        pub witnesses: ::prost::alloc::vec::Vec<block_chain_info::Witness>,
        #[prost(int64, tag="11")]
        pub fail_process_block_num: i64,
        #[prost(string, tag="12")]
        pub fail_process_block_reason: ::prost::alloc::string::String,
        #[prost(message, repeated, tag="13")]
        pub dup_witness: ::prost::alloc::vec::Vec<block_chain_info::DupWitness>,
    }
    /// Nested message and enum types in `BlockChainInfo`.
    pub mod block_chain_info {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Witness {
            #[prost(string, tag="1")]
            pub address: ::prost::alloc::string::String,
            #[prost(int32, tag="2")]
            pub version: i32,
        }
impl ::prost::Name for Witness {
const NAME: &'static str = "Witness";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.MetricsInfo.BlockChainInfo.{}", Self::NAME)
            }}
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DupWitness {
            #[prost(string, tag="1")]
            pub address: ::prost::alloc::string::String,
            #[prost(int64, tag="2")]
            pub block_num: i64,
            #[prost(int32, tag="3")]
            pub count: i32,
        }
impl ::prost::Name for DupWitness {
const NAME: &'static str = "DupWitness";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.MetricsInfo.BlockChainInfo.{}", Self::NAME)
            }}
    }
impl ::prost::Name for BlockChainInfo {
const NAME: &'static str = "BlockChainInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.MetricsInfo.{}", Self::NAME)
            }}
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RateInfo {
        #[prost(int64, tag="1")]
        pub count: i64,
        #[prost(double, tag="2")]
        pub mean_rate: f64,
        #[prost(double, tag="3")]
        pub one_minute_rate: f64,
        #[prost(double, tag="4")]
        pub five_minute_rate: f64,
        #[prost(double, tag="5")]
        pub fifteen_minute_rate: f64,
    }
impl ::prost::Name for RateInfo {
const NAME: &'static str = "RateInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.MetricsInfo.{}", Self::NAME)
            }}
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NetInfo {
        #[prost(int32, tag="1")]
        pub error_proto_count: i32,
        #[prost(message, optional, tag="2")]
        pub api: ::core::option::Option<net_info::ApiInfo>,
        #[prost(int32, tag="3")]
        pub connection_count: i32,
        #[prost(int32, tag="4")]
        pub valid_connection_count: i32,
        #[prost(message, optional, tag="5")]
        pub tcp_in_traffic: ::core::option::Option<RateInfo>,
        #[prost(message, optional, tag="6")]
        pub tcp_out_traffic: ::core::option::Option<RateInfo>,
        #[prost(int32, tag="7")]
        pub disconnection_count: i32,
        #[prost(message, repeated, tag="8")]
        pub disconnection_detail: ::prost::alloc::vec::Vec<net_info::DisconnectionDetailInfo>,
        #[prost(message, optional, tag="9")]
        pub udp_in_traffic: ::core::option::Option<RateInfo>,
        #[prost(message, optional, tag="10")]
        pub udp_out_traffic: ::core::option::Option<RateInfo>,
        #[prost(message, optional, tag="11")]
        pub latency: ::core::option::Option<net_info::LatencyInfo>,
    }
    /// Nested message and enum types in `NetInfo`.
    pub mod net_info {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ApiInfo {
            #[prost(message, optional, tag="1")]
            pub qps: ::core::option::Option<super::RateInfo>,
            #[prost(message, optional, tag="2")]
            pub fail_qps: ::core::option::Option<super::RateInfo>,
            #[prost(message, optional, tag="3")]
            pub out_traffic: ::core::option::Option<super::RateInfo>,
            #[prost(message, repeated, tag="4")]
            pub detail: ::prost::alloc::vec::Vec<api_info::ApiDetailInfo>,
        }
        /// Nested message and enum types in `ApiInfo`.
        pub mod api_info {
            #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ApiDetailInfo {
                #[prost(string, tag="1")]
                pub name: ::prost::alloc::string::String,
                #[prost(message, optional, tag="2")]
                pub qps: ::core::option::Option<super::super::RateInfo>,
                #[prost(message, optional, tag="3")]
                pub fail_qps: ::core::option::Option<super::super::RateInfo>,
                #[prost(message, optional, tag="4")]
                pub out_traffic: ::core::option::Option<super::super::RateInfo>,
            }
impl ::prost::Name for ApiDetailInfo {
const NAME: &'static str = "ApiDetailInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.MetricsInfo.NetInfo.ApiInfo.{}", Self::NAME)
            }}
        }
impl ::prost::Name for ApiInfo {
const NAME: &'static str = "ApiInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.MetricsInfo.NetInfo.{}", Self::NAME)
            }}
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DisconnectionDetailInfo {
            #[prost(string, tag="1")]
            pub reason: ::prost::alloc::string::String,
            #[prost(int32, tag="2")]
            pub count: i32,
        }
impl ::prost::Name for DisconnectionDetailInfo {
const NAME: &'static str = "DisconnectionDetailInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.MetricsInfo.NetInfo.{}", Self::NAME)
            }}
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct LatencyInfo {
            #[prost(int32, tag="1")]
            pub top99: i32,
            #[prost(int32, tag="2")]
            pub top95: i32,
            #[prost(int32, tag="3")]
            pub top75: i32,
            #[prost(int32, tag="4")]
            pub total_count: i32,
            #[prost(int32, tag="5")]
            pub delay1_s: i32,
            #[prost(int32, tag="6")]
            pub delay2_s: i32,
            #[prost(int32, tag="7")]
            pub delay3_s: i32,
            #[prost(message, repeated, tag="8")]
            pub detail: ::prost::alloc::vec::Vec<latency_info::LatencyDetailInfo>,
        }
        /// Nested message and enum types in `LatencyInfo`.
        pub mod latency_info {
            #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
            pub struct LatencyDetailInfo {
                #[prost(string, tag="1")]
                pub witness: ::prost::alloc::string::String,
                #[prost(int32, tag="2")]
                pub top99: i32,
                #[prost(int32, tag="3")]
                pub top95: i32,
                #[prost(int32, tag="4")]
                pub top75: i32,
                #[prost(int32, tag="5")]
                pub count: i32,
                #[prost(int32, tag="6")]
                pub delay1_s: i32,
                #[prost(int32, tag="7")]
                pub delay2_s: i32,
                #[prost(int32, tag="8")]
                pub delay3_s: i32,
            }
impl ::prost::Name for LatencyDetailInfo {
const NAME: &'static str = "LatencyDetailInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.MetricsInfo.NetInfo.LatencyInfo.{}", Self::NAME)
            }}
        }
impl ::prost::Name for LatencyInfo {
const NAME: &'static str = "LatencyInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.MetricsInfo.NetInfo.{}", Self::NAME)
            }}
    }
impl ::prost::Name for NetInfo {
const NAME: &'static str = "NetInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.MetricsInfo.{}", Self::NAME)
            }}
}
impl ::prost::Name for MetricsInfo {
const NAME: &'static str = "MetricsInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbftMessage {
    #[prost(message, optional, tag="1")]
    pub raw_data: ::core::option::Option<pbft_message::Raw>,
    #[prost(bytes="vec", tag="2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `PBFTMessage`.
pub mod pbft_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Raw {
        #[prost(enumeration="MsgType", tag="1")]
        pub msg_type: i32,
        #[prost(enumeration="DataType", tag="2")]
        pub data_type: i32,
        #[prost(int64, tag="3")]
        pub view_n: i64,
        #[prost(int64, tag="4")]
        pub epoch: i64,
        #[prost(bytes="vec", tag="5")]
        pub data: ::prost::alloc::vec::Vec<u8>,
    }
impl ::prost::Name for Raw {
const NAME: &'static str = "Raw";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.PBFTMessage.{}", Self::NAME)
            }}
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MsgType {
        ViewChange = 0,
        Request = 1,
        Preprepare = 2,
        Prepare = 3,
        Commit = 4,
    }
    impl MsgType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MsgType::ViewChange => "VIEW_CHANGE",
                MsgType::Request => "REQUEST",
                MsgType::Preprepare => "PREPREPARE",
                MsgType::Prepare => "PREPARE",
                MsgType::Commit => "COMMIT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VIEW_CHANGE" => Some(Self::ViewChange),
                "REQUEST" => Some(Self::Request),
                "PREPREPARE" => Some(Self::Preprepare),
                "PREPARE" => Some(Self::Prepare),
                "COMMIT" => Some(Self::Commit),
                _ => None,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataType {
        Block = 0,
        Srl = 1,
    }
    impl DataType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DataType::Block => "BLOCK",
                DataType::Srl => "SRL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BLOCK" => Some(Self::Block),
                "SRL" => Some(Self::Srl),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for PbftMessage {
const NAME: &'static str = "PBFTMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbftCommitResult {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub signature: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
impl ::prost::Name for PbftCommitResult {
const NAME: &'static str = "PBFTCommitResult";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Srl {
    #[prost(bytes="vec", repeated, tag="1")]
    pub sr_address: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
impl ::prost::Name for Srl {
const NAME: &'static str = "SRL";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountType {
    Normal = 0,
    AssetIssue = 1,
    Contract = 2,
}
impl AccountType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccountType::Normal => "Normal",
            AccountType::AssetIssue => "AssetIssue",
            AccountType::Contract => "Contract",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Normal" => Some(Self::Normal),
            "AssetIssue" => Some(Self::AssetIssue),
            "Contract" => Some(Self::Contract),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReasonCode {
    Requested = 0,
    BadProtocol = 2,
    TooManyPeers = 4,
    DuplicatePeer = 5,
    IncompatibleProtocol = 6,
    RandomElimination = 7,
    PeerQuiting = 8,
    UnexpectedIdentity = 9,
    LocalIdentity = 10,
    PingTimeout = 11,
    UserReason = 16,
    Reset = 17,
    SyncFail = 18,
    FetchFail = 19,
    BadTx = 20,
    BadBlock = 21,
    Forked = 22,
    Unlinkable = 23,
    IncompatibleVersion = 24,
    IncompatibleChain = 25,
    TimeOut = 32,
    ConnectFail = 33,
    TooManyPeersWithSameIp = 34,
    LightNodeSyncFail = 35,
    BelowThanMe = 36,
    NotWitness = 37,
    NoSuchMessage = 38,
    Unknown = 255,
}
impl ReasonCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReasonCode::Requested => "REQUESTED",
            ReasonCode::BadProtocol => "BAD_PROTOCOL",
            ReasonCode::TooManyPeers => "TOO_MANY_PEERS",
            ReasonCode::DuplicatePeer => "DUPLICATE_PEER",
            ReasonCode::IncompatibleProtocol => "INCOMPATIBLE_PROTOCOL",
            ReasonCode::RandomElimination => "RANDOM_ELIMINATION",
            ReasonCode::PeerQuiting => "PEER_QUITING",
            ReasonCode::UnexpectedIdentity => "UNEXPECTED_IDENTITY",
            ReasonCode::LocalIdentity => "LOCAL_IDENTITY",
            ReasonCode::PingTimeout => "PING_TIMEOUT",
            ReasonCode::UserReason => "USER_REASON",
            ReasonCode::Reset => "RESET",
            ReasonCode::SyncFail => "SYNC_FAIL",
            ReasonCode::FetchFail => "FETCH_FAIL",
            ReasonCode::BadTx => "BAD_TX",
            ReasonCode::BadBlock => "BAD_BLOCK",
            ReasonCode::Forked => "FORKED",
            ReasonCode::Unlinkable => "UNLINKABLE",
            ReasonCode::IncompatibleVersion => "INCOMPATIBLE_VERSION",
            ReasonCode::IncompatibleChain => "INCOMPATIBLE_CHAIN",
            ReasonCode::TimeOut => "TIME_OUT",
            ReasonCode::ConnectFail => "CONNECT_FAIL",
            ReasonCode::TooManyPeersWithSameIp => "TOO_MANY_PEERS_WITH_SAME_IP",
            ReasonCode::LightNodeSyncFail => "LIGHT_NODE_SYNC_FAIL",
            ReasonCode::BelowThanMe => "BELOW_THAN_ME",
            ReasonCode::NotWitness => "NOT_WITNESS",
            ReasonCode::NoSuchMessage => "NO_SUCH_MESSAGE",
            ReasonCode::Unknown => "UNKNOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REQUESTED" => Some(Self::Requested),
            "BAD_PROTOCOL" => Some(Self::BadProtocol),
            "TOO_MANY_PEERS" => Some(Self::TooManyPeers),
            "DUPLICATE_PEER" => Some(Self::DuplicatePeer),
            "INCOMPATIBLE_PROTOCOL" => Some(Self::IncompatibleProtocol),
            "RANDOM_ELIMINATION" => Some(Self::RandomElimination),
            "PEER_QUITING" => Some(Self::PeerQuiting),
            "UNEXPECTED_IDENTITY" => Some(Self::UnexpectedIdentity),
            "LOCAL_IDENTITY" => Some(Self::LocalIdentity),
            "PING_TIMEOUT" => Some(Self::PingTimeout),
            "USER_REASON" => Some(Self::UserReason),
            "RESET" => Some(Self::Reset),
            "SYNC_FAIL" => Some(Self::SyncFail),
            "FETCH_FAIL" => Some(Self::FetchFail),
            "BAD_TX" => Some(Self::BadTx),
            "BAD_BLOCK" => Some(Self::BadBlock),
            "FORKED" => Some(Self::Forked),
            "UNLINKABLE" => Some(Self::Unlinkable),
            "INCOMPATIBLE_VERSION" => Some(Self::IncompatibleVersion),
            "INCOMPATIBLE_CHAIN" => Some(Self::IncompatibleChain),
            "TIME_OUT" => Some(Self::TimeOut),
            "CONNECT_FAIL" => Some(Self::ConnectFail),
            "TOO_MANY_PEERS_WITH_SAME_IP" => Some(Self::TooManyPeersWithSameIp),
            "LIGHT_NODE_SYNC_FAIL" => Some(Self::LightNodeSyncFail),
            "BELOW_THAN_ME" => Some(Self::BelowThanMe),
            "NOT_WITNESS" => Some(Self::NotWitness),
            "NO_SUCH_MESSAGE" => Some(Self::NoSuchMessage),
            "UNKNOWN" => Some(Self::Unknown),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetIssueContract {
    #[prost(string, tag="41")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub abbr: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="4")]
    pub total_supply: i64,
    #[prost(message, repeated, tag="5")]
    pub frozen_supply: ::prost::alloc::vec::Vec<asset_issue_contract::FrozenSupply>,
    #[prost(int32, tag="6")]
    pub trx_num: i32,
    #[prost(int32, tag="7")]
    pub precision: i32,
    #[prost(int32, tag="8")]
    pub num: i32,
    #[prost(int64, tag="9")]
    pub start_time: i64,
    #[prost(int64, tag="10")]
    pub end_time: i64,
    /// useless
    #[prost(int64, tag="11")]
    pub order: i64,
    #[prost(int32, tag="16")]
    pub vote_score: i32,
    #[prost(bytes="vec", tag="20")]
    pub description: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="21")]
    pub url: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="22")]
    pub free_asset_net_limit: i64,
    #[prost(int64, tag="23")]
    pub public_free_asset_net_limit: i64,
    #[prost(int64, tag="24")]
    pub public_free_asset_net_usage: i64,
    #[prost(int64, tag="25")]
    pub public_latest_free_net_time: i64,
}
/// Nested message and enum types in `AssetIssueContract`.
pub mod asset_issue_contract {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FrozenSupply {
        #[prost(int64, tag="1")]
        pub frozen_amount: i64,
        #[prost(int64, tag="2")]
        pub frozen_days: i64,
    }
impl ::prost::Name for FrozenSupply {
const NAME: &'static str = "FrozenSupply";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.AssetIssueContract.{}", Self::NAME)
            }}
}
impl ::prost::Name for AssetIssueContract {
const NAME: &'static str = "AssetIssueContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferAssetContract {
    /// this field is token name before the proposal ALLOW_SAME_TOKEN_NAME is active, otherwise it is token id and token is should be in string format.
    #[prost(bytes="vec", tag="1")]
    pub asset_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="4")]
    pub amount: i64,
}
impl ::prost::Name for TransferAssetContract {
const NAME: &'static str = "TransferAssetContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnfreezeAssetContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for UnfreezeAssetContract {
const NAME: &'static str = "UnfreezeAssetContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAssetContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub description: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub url: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="4")]
    pub new_limit: i64,
    #[prost(int64, tag="5")]
    pub new_public_limit: i64,
}
impl ::prost::Name for UpdateAssetContract {
const NAME: &'static str = "UpdateAssetContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipateAssetIssueContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub to_address: ::prost::alloc::vec::Vec<u8>,
    /// this field is token name before the proposal ALLOW_SAME_TOKEN_NAME is active, otherwise it is token id and token is should be in string format.
    #[prost(bytes="vec", tag="3")]
    pub asset_name: ::prost::alloc::vec::Vec<u8>,
    /// the amount of drops
    #[prost(int64, tag="4")]
    pub amount: i64,
}
impl ::prost::Name for ParticipateAssetIssueContract {
const NAME: &'static str = "ParticipateAssetIssueContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountCreateContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub account_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="AccountType", tag="3")]
    pub r#type: i32,
}
impl ::prost::Name for AccountCreateContract {
const NAME: &'static str = "AccountCreateContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
/// Update account name. Account name is not unique now.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUpdateContract {
    #[prost(bytes="vec", tag="1")]
    pub account_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for AccountUpdateContract {
const NAME: &'static str = "AccountUpdateContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
/// Set account id if the account has no id. Account id is unique and case insensitive.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAccountIdContract {
    #[prost(bytes="vec", tag="1")]
    pub account_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for SetAccountIdContract {
const NAME: &'static str = "SetAccountIdContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountPermissionUpdateContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    /// Empty is invalidate
    #[prost(message, optional, tag="2")]
    pub owner: ::core::option::Option<Permission>,
    /// Can be empty
    #[prost(message, optional, tag="3")]
    pub witness: ::core::option::Option<Permission>,
    /// Empty is invalidate
    #[prost(message, repeated, tag="4")]
    pub actives: ::prost::alloc::vec::Vec<Permission>,
}
impl ::prost::Name for AccountPermissionUpdateContract {
const NAME: &'static str = "AccountPermissionUpdateContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WitnessCreateContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub url: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for WitnessCreateContract {
const NAME: &'static str = "WitnessCreateContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WitnessUpdateContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="12")]
    pub update_url: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for WitnessUpdateContract {
const NAME: &'static str = "WitnessUpdateContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteWitnessContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="2")]
    pub votes: ::prost::alloc::vec::Vec<vote_witness_contract::Vote>,
    #[prost(bool, tag="3")]
    pub support: bool,
}
/// Nested message and enum types in `VoteWitnessContract`.
pub mod vote_witness_contract {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Vote {
        #[prost(bytes="vec", tag="1")]
        pub vote_address: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="2")]
        pub vote_count: i64,
    }
impl ::prost::Name for Vote {
const NAME: &'static str = "Vote";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.VoteWitnessContract.{}", Self::NAME)
            }}
}
impl ::prost::Name for VoteWitnessContract {
const NAME: &'static str = "VoteWitnessContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreezeBalanceContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub frozen_balance: i64,
    #[prost(int64, tag="3")]
    pub frozen_duration: i64,
    #[prost(enumeration="ResourceCode", tag="10")]
    pub resource: i32,
    #[prost(bytes="vec", tag="15")]
    pub receiver_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for FreezeBalanceContract {
const NAME: &'static str = "FreezeBalanceContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnfreezeBalanceContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="ResourceCode", tag="10")]
    pub resource: i32,
    #[prost(bytes="vec", tag="15")]
    pub receiver_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for UnfreezeBalanceContract {
const NAME: &'static str = "UnfreezeBalanceContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawBalanceContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for WithdrawBalanceContract {
const NAME: &'static str = "WithdrawBalanceContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub amount: i64,
}
impl ::prost::Name for TransferContract {
const NAME: &'static str = "TransferContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionBalanceTrace {
    #[prost(bytes="vec", tag="1")]
    pub transaction_identifier: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="2")]
    pub operation: ::prost::alloc::vec::Vec<transaction_balance_trace::Operation>,
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub status: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TransactionBalanceTrace`.
pub mod transaction_balance_trace {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Operation {
        #[prost(int64, tag="1")]
        pub operation_identifier: i64,
        #[prost(bytes="vec", tag="2")]
        pub address: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="3")]
        pub amount: i64,
    }
impl ::prost::Name for Operation {
const NAME: &'static str = "Operation";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.TransactionBalanceTrace.{}", Self::NAME)
            }}
}
impl ::prost::Name for TransactionBalanceTrace {
const NAME: &'static str = "TransactionBalanceTrace";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockBalanceTrace {
    #[prost(message, optional, tag="1")]
    pub block_identifier: ::core::option::Option<block_balance_trace::BlockIdentifier>,
    #[prost(int64, tag="2")]
    pub timestamp: i64,
    ///   BlockIdentifier parent_block_identifier = 4;
    #[prost(message, repeated, tag="3")]
    pub transaction_balance_trace: ::prost::alloc::vec::Vec<TransactionBalanceTrace>,
}
/// Nested message and enum types in `BlockBalanceTrace`.
pub mod block_balance_trace {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlockIdentifier {
        #[prost(bytes="vec", tag="1")]
        pub hash: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="2")]
        pub number: i64,
    }
impl ::prost::Name for BlockIdentifier {
const NAME: &'static str = "BlockIdentifier";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.BlockBalanceTrace.{}", Self::NAME)
            }}
}
impl ::prost::Name for BlockBalanceTrace {
const NAME: &'static str = "BlockBalanceTrace";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountTrace {
    #[prost(int64, tag="1")]
    pub balance: i64,
    #[prost(int64, tag="99")]
    pub placeholder: i64,
}
impl ::prost::Name for AccountTrace {
const NAME: &'static str = "AccountTrace";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountIdentifier {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for AccountIdentifier {
const NAME: &'static str = "AccountIdentifier";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountBalanceRequest {
    #[prost(message, optional, tag="1")]
    pub account_identifier: ::core::option::Option<AccountIdentifier>,
    #[prost(message, optional, tag="2")]
    pub block_identifier: ::core::option::Option<block_balance_trace::BlockIdentifier>,
}
impl ::prost::Name for AccountBalanceRequest {
const NAME: &'static str = "AccountBalanceRequest";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountBalanceResponse {
    #[prost(int64, tag="1")]
    pub balance: i64,
    #[prost(message, optional, tag="2")]
    pub block_identifier: ::core::option::Option<block_balance_trace::BlockIdentifier>,
}
impl ::prost::Name for AccountBalanceResponse {
const NAME: &'static str = "AccountBalanceResponse";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreezeBalanceV2Contract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub frozen_balance: i64,
    #[prost(enumeration="ResourceCode", tag="3")]
    pub resource: i32,
}
impl ::prost::Name for FreezeBalanceV2Contract {
const NAME: &'static str = "FreezeBalanceV2Contract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnfreezeBalanceV2Contract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub unfreeze_balance: i64,
    #[prost(enumeration="ResourceCode", tag="3")]
    pub resource: i32,
}
impl ::prost::Name for UnfreezeBalanceV2Contract {
const NAME: &'static str = "UnfreezeBalanceV2Contract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawExpireUnfreezeContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for WithdrawExpireUnfreezeContract {
const NAME: &'static str = "WithdrawExpireUnfreezeContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegateResourceContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="ResourceCode", tag="2")]
    pub resource: i32,
    #[prost(int64, tag="3")]
    pub balance: i64,
    #[prost(bytes="vec", tag="4")]
    pub receiver_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="5")]
    pub lock: bool,
    #[prost(int64, tag="6")]
    pub lock_period: i64,
}
impl ::prost::Name for DelegateResourceContract {
const NAME: &'static str = "DelegateResourceContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnDelegateResourceContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="ResourceCode", tag="2")]
    pub resource: i32,
    #[prost(int64, tag="3")]
    pub balance: i64,
    #[prost(bytes="vec", tag="4")]
    pub receiver_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for UnDelegateResourceContract {
const NAME: &'static str = "UnDelegateResourceContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelAllUnfreezeV2Contract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for CancelAllUnfreezeV2Contract {
const NAME: &'static str = "CancelAllUnfreezeV2Contract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalApproveContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub proposal_id: i64,
    /// add or remove approval
    #[prost(bool, tag="3")]
    pub is_add_approval: bool,
}
impl ::prost::Name for ProposalApproveContract {
const NAME: &'static str = "ProposalApproveContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalCreateContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(map="int64, int64", tag="2")]
    pub parameters: ::std::collections::HashMap<i64, i64>,
}
impl ::prost::Name for ProposalCreateContract {
const NAME: &'static str = "ProposalCreateContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalDeleteContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub proposal_id: i64,
}
impl ::prost::Name for ProposalDeleteContract {
const NAME: &'static str = "ProposalDeleteContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyStorageBytesContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    /// storage bytes for buy
    #[prost(int64, tag="2")]
    pub bytes: i64,
}
impl ::prost::Name for BuyStorageBytesContract {
const NAME: &'static str = "BuyStorageBytesContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyStorageContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    /// trx quantity for buy storage (sun)
    #[prost(int64, tag="2")]
    pub quant: i64,
}
impl ::prost::Name for BuyStorageContract {
const NAME: &'static str = "BuyStorageContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SellStorageContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub storage_bytes: i64,
}
impl ::prost::Name for SellStorageContract {
const NAME: &'static str = "SellStorageContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBrokerageContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    /// 1 mean 1%
    #[prost(int32, tag="2")]
    pub brokerage: i32,
}
impl ::prost::Name for UpdateBrokerageContract {
const NAME: &'static str = "UpdateBrokerageContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeCreateContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub first_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub first_token_balance: i64,
    #[prost(bytes="vec", tag="4")]
    pub second_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub second_token_balance: i64,
}
impl ::prost::Name for ExchangeCreateContract {
const NAME: &'static str = "ExchangeCreateContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeInjectContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub exchange_id: i64,
    #[prost(bytes="vec", tag="3")]
    pub token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="4")]
    pub quant: i64,
}
impl ::prost::Name for ExchangeInjectContract {
const NAME: &'static str = "ExchangeInjectContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeWithdrawContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub exchange_id: i64,
    #[prost(bytes="vec", tag="3")]
    pub token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="4")]
    pub quant: i64,
}
impl ::prost::Name for ExchangeWithdrawContract {
const NAME: &'static str = "ExchangeWithdrawContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeTransactionContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub exchange_id: i64,
    #[prost(bytes="vec", tag="3")]
    pub token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="4")]
    pub quant: i64,
    #[prost(int64, tag="5")]
    pub expected: i64,
}
impl ::prost::Name for ExchangeTransactionContract {
const NAME: &'static str = "ExchangeTransactionContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketSellAssetContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub sell_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub sell_token_quantity: i64,
    #[prost(bytes="vec", tag="4")]
    pub buy_token_id: ::prost::alloc::vec::Vec<u8>,
    /// min to receive
    #[prost(int64, tag="5")]
    pub buy_token_quantity: i64,
}
impl ::prost::Name for MarketSellAssetContract {
const NAME: &'static str = "MarketSellAssetContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketCancelOrderContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub order_id: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for MarketCancelOrderContract {
const NAME: &'static str = "MarketCancelOrderContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmartContract {
    #[prost(bytes="vec", tag="1")]
    pub origin_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub abi: ::core::option::Option<smart_contract::Abi>,
    #[prost(bytes="vec", tag="4")]
    pub bytecode: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub call_value: i64,
    #[prost(int64, tag="6")]
    pub consume_user_resource_percent: i64,
    #[prost(string, tag="7")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag="8")]
    pub origin_energy_limit: i64,
    #[prost(bytes="vec", tag="9")]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="10")]
    pub trx_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="11")]
    pub version: i32,
}
/// Nested message and enum types in `SmartContract`.
pub mod smart_contract {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Abi {
        #[prost(message, repeated, tag="1")]
        pub entrys: ::prost::alloc::vec::Vec<abi::Entry>,
    }
    /// Nested message and enum types in `ABI`.
    pub mod abi {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Entry {
            #[prost(bool, tag="1")]
            pub anonymous: bool,
            #[prost(bool, tag="2")]
            pub constant: bool,
            #[prost(string, tag="3")]
            pub name: ::prost::alloc::string::String,
            #[prost(message, repeated, tag="4")]
            pub inputs: ::prost::alloc::vec::Vec<entry::Param>,
            #[prost(message, repeated, tag="5")]
            pub outputs: ::prost::alloc::vec::Vec<entry::Param>,
            #[prost(enumeration="entry::EntryType", tag="6")]
            pub r#type: i32,
            #[prost(bool, tag="7")]
            pub payable: bool,
            #[prost(enumeration="entry::StateMutabilityType", tag="8")]
            pub state_mutability: i32,
        }
        /// Nested message and enum types in `Entry`.
        pub mod entry {
            #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Param {
                #[prost(bool, tag="1")]
                pub indexed: bool,
                #[prost(string, tag="2")]
                pub name: ::prost::alloc::string::String,
                /// SolidityType type = 3;
                #[prost(string, tag="3")]
                pub r#type: ::prost::alloc::string::String,
            }
impl ::prost::Name for Param {
const NAME: &'static str = "Param";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.SmartContract.ABI.Entry.{}", Self::NAME)
            }}
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
            #[repr(i32)]
            pub enum EntryType {
                UnknownEntryType = 0,
                Constructor = 1,
                Function = 2,
                Event = 3,
                Fallback = 4,
                Receive = 5,
                Error = 6,
            }
            impl EntryType {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        EntryType::UnknownEntryType => "UnknownEntryType",
                        EntryType::Constructor => "Constructor",
                        EntryType::Function => "Function",
                        EntryType::Event => "Event",
                        EntryType::Fallback => "Fallback",
                        EntryType::Receive => "Receive",
                        EntryType::Error => "Error",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "UnknownEntryType" => Some(Self::UnknownEntryType),
                        "Constructor" => Some(Self::Constructor),
                        "Function" => Some(Self::Function),
                        "Event" => Some(Self::Event),
                        "Fallback" => Some(Self::Fallback),
                        "Receive" => Some(Self::Receive),
                        "Error" => Some(Self::Error),
                        _ => None,
                    }
                }
            }
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
            #[repr(i32)]
            pub enum StateMutabilityType {
                UnknownMutabilityType = 0,
                Pure = 1,
                View = 2,
                Nonpayable = 3,
                Payable = 4,
            }
            impl StateMutabilityType {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        StateMutabilityType::UnknownMutabilityType => "UnknownMutabilityType",
                        StateMutabilityType::Pure => "Pure",
                        StateMutabilityType::View => "View",
                        StateMutabilityType::Nonpayable => "Nonpayable",
                        StateMutabilityType::Payable => "Payable",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "UnknownMutabilityType" => Some(Self::UnknownMutabilityType),
                        "Pure" => Some(Self::Pure),
                        "View" => Some(Self::View),
                        "Nonpayable" => Some(Self::Nonpayable),
                        "Payable" => Some(Self::Payable),
                        _ => None,
                    }
                }
            }
        }
impl ::prost::Name for Entry {
const NAME: &'static str = "Entry";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.SmartContract.ABI.{}", Self::NAME)
            }}
    }
impl ::prost::Name for Abi {
const NAME: &'static str = "ABI";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.SmartContract.{}", Self::NAME)
            }}
}
impl ::prost::Name for SmartContract {
const NAME: &'static str = "SmartContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractState {
    #[prost(int64, tag="1")]
    pub energy_usage: i64,
    #[prost(int64, tag="2")]
    pub energy_factor: i64,
    #[prost(int64, tag="3")]
    pub update_cycle: i64,
}
impl ::prost::Name for ContractState {
const NAME: &'static str = "ContractState";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSmartContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub new_contract: ::core::option::Option<SmartContract>,
    #[prost(int64, tag="3")]
    pub call_token_value: i64,
    #[prost(int64, tag="4")]
    pub token_id: i64,
}
impl ::prost::Name for CreateSmartContract {
const NAME: &'static str = "CreateSmartContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerSmartContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub call_value: i64,
    #[prost(bytes="vec", tag="4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub call_token_value: i64,
    #[prost(int64, tag="6")]
    pub token_id: i64,
}
impl ::prost::Name for TriggerSmartContract {
const NAME: &'static str = "TriggerSmartContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearAbiContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ClearAbiContract {
const NAME: &'static str = "ClearABIContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSettingContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub consume_user_resource_percent: i64,
}
impl ::prost::Name for UpdateSettingContract {
const NAME: &'static str = "UpdateSettingContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEnergyLimitContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub origin_energy_limit: i64,
}
impl ::prost::Name for UpdateEnergyLimitContract {
const NAME: &'static str = "UpdateEnergyLimitContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmartContractDataWrapper {
    #[prost(message, optional, tag="1")]
    pub smart_contract: ::core::option::Option<SmartContract>,
    #[prost(bytes="vec", tag="2")]
    pub runtimecode: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub contract_state: ::core::option::Option<ContractState>,
}
impl ::prost::Name for SmartContractDataWrapper {
const NAME: &'static str = "SmartContractDataWrapper";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
// for shielded transaction

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationPath {
    #[prost(bool, repeated, tag="1")]
    pub value: ::prost::alloc::vec::Vec<bool>,
}
impl ::prost::Name for AuthenticationPath {
const NAME: &'static str = "AuthenticationPath";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerklePath {
    #[prost(message, repeated, tag="1")]
    pub authentication_paths: ::prost::alloc::vec::Vec<AuthenticationPath>,
    #[prost(bool, repeated, tag="2")]
    pub index: ::prost::alloc::vec::Vec<bool>,
    #[prost(bytes="vec", tag="3")]
    pub rt: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for MerklePath {
const NAME: &'static str = "MerklePath";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputPoint {
    #[prost(bytes="vec", tag="1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="2")]
    pub index: i32,
}
impl ::prost::Name for OutputPoint {
const NAME: &'static str = "OutputPoint";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputPointInfo {
    #[prost(message, repeated, tag="1")]
    pub out_points: ::prost::alloc::vec::Vec<OutputPoint>,
    #[prost(int32, tag="2")]
    pub block_num: i32,
}
impl ::prost::Name for OutputPointInfo {
const NAME: &'static str = "OutputPointInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PedersenHash {
    #[prost(bytes="vec", tag="1")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for PedersenHash {
const NAME: &'static str = "PedersenHash";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncrementalMerkleTree {
    #[prost(message, optional, tag="1")]
    pub left: ::core::option::Option<PedersenHash>,
    #[prost(message, optional, tag="2")]
    pub right: ::core::option::Option<PedersenHash>,
    #[prost(message, repeated, tag="3")]
    pub parents: ::prost::alloc::vec::Vec<PedersenHash>,
}
impl ::prost::Name for IncrementalMerkleTree {
const NAME: &'static str = "IncrementalMerkleTree";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncrementalMerkleVoucher {
    #[prost(message, optional, tag="1")]
    pub tree: ::core::option::Option<IncrementalMerkleTree>,
    #[prost(message, repeated, tag="2")]
    pub filled: ::prost::alloc::vec::Vec<PedersenHash>,
    #[prost(message, optional, tag="3")]
    pub cursor: ::core::option::Option<IncrementalMerkleTree>,
    #[prost(int64, tag="4")]
    pub cursor_depth: i64,
    #[prost(bytes="vec", tag="5")]
    pub rt: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="10")]
    pub output_point: ::core::option::Option<OutputPoint>,
}
impl ::prost::Name for IncrementalMerkleVoucher {
const NAME: &'static str = "IncrementalMerkleVoucher";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncrementalMerkleVoucherInfo {
    #[prost(message, repeated, tag="1")]
    pub vouchers: ::prost::alloc::vec::Vec<IncrementalMerkleVoucher>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
impl ::prost::Name for IncrementalMerkleVoucherInfo {
const NAME: &'static str = "IncrementalMerkleVoucherInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendDescription {
    #[prost(bytes="vec", tag="1")]
    pub value_commitment: ::prost::alloc::vec::Vec<u8>,
    /// merkle root
    #[prost(bytes="vec", tag="2")]
    pub anchor: ::prost::alloc::vec::Vec<u8>,
    /// used for check double spend
    #[prost(bytes="vec", tag="3")]
    pub nullifier: ::prost::alloc::vec::Vec<u8>,
    /// used for check spend authority signature
    #[prost(bytes="vec", tag="4")]
    pub rk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub zkproof: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub spend_authority_signature: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for SpendDescription {
const NAME: &'static str = "SpendDescription";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveDescription {
    #[prost(bytes="vec", tag="1")]
    pub value_commitment: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub note_commitment: ::prost::alloc::vec::Vec<u8>,
    /// for Encryption
    #[prost(bytes="vec", tag="3")]
    pub epk: ::prost::alloc::vec::Vec<u8>,
    /// Encryption for incoming, decrypt it with ivk
    #[prost(bytes="vec", tag="4")]
    pub c_enc: ::prost::alloc::vec::Vec<u8>,
    /// Encryption for audit, decrypt it with ovk
    #[prost(bytes="vec", tag="5")]
    pub c_out: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub zkproof: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ReceiveDescription {
const NAME: &'static str = "ReceiveDescription";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShieldedTransferContract {
    /// transparent address
    #[prost(bytes="vec", tag="1")]
    pub transparent_from_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub from_amount: i64,
    #[prost(message, repeated, tag="3")]
    pub spend_description: ::prost::alloc::vec::Vec<SpendDescription>,
    #[prost(message, repeated, tag="4")]
    pub receive_description: ::prost::alloc::vec::Vec<ReceiveDescription>,
    #[prost(bytes="vec", tag="5")]
    pub binding_signature: ::prost::alloc::vec::Vec<u8>,
    /// transparent address
    #[prost(bytes="vec", tag="6")]
    pub transparent_to_address: ::prost::alloc::vec::Vec<u8>,
    /// the amount to transparent to_address
    #[prost(int64, tag="7")]
    pub to_amount: i64,
}
impl ::prost::Name for ShieldedTransferContract {
const NAME: &'static str = "ShieldedTransferContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Return {
    #[prost(bool, tag="1")]
    pub result: bool,
    #[prost(enumeration="r#return::ResponseCode", tag="2")]
    pub code: i32,
    #[prost(bytes="vec", tag="3")]
    pub message: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `Return`.
pub mod r#return {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResponseCode {
        Success = 0,
        /// error in signature
        Sigerror = 1,
        ContractValidateError = 2,
        ContractExeError = 3,
        BandwithError = 4,
        DupTransactionError = 5,
        TaposError = 6,
        TooBigTransactionError = 7,
        TransactionExpirationError = 8,
        ServerBusy = 9,
        NoConnection = 10,
        NotEnoughEffectiveConnection = 11,
        BlockUnsolidified = 12,
        OtherError = 20,
    }
    impl ResponseCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ResponseCode::Success => "SUCCESS",
                ResponseCode::Sigerror => "SIGERROR",
                ResponseCode::ContractValidateError => "CONTRACT_VALIDATE_ERROR",
                ResponseCode::ContractExeError => "CONTRACT_EXE_ERROR",
                ResponseCode::BandwithError => "BANDWITH_ERROR",
                ResponseCode::DupTransactionError => "DUP_TRANSACTION_ERROR",
                ResponseCode::TaposError => "TAPOS_ERROR",
                ResponseCode::TooBigTransactionError => "TOO_BIG_TRANSACTION_ERROR",
                ResponseCode::TransactionExpirationError => "TRANSACTION_EXPIRATION_ERROR",
                ResponseCode::ServerBusy => "SERVER_BUSY",
                ResponseCode::NoConnection => "NO_CONNECTION",
                ResponseCode::NotEnoughEffectiveConnection => "NOT_ENOUGH_EFFECTIVE_CONNECTION",
                ResponseCode::BlockUnsolidified => "BLOCK_UNSOLIDIFIED",
                ResponseCode::OtherError => "OTHER_ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUCCESS" => Some(Self::Success),
                "SIGERROR" => Some(Self::Sigerror),
                "CONTRACT_VALIDATE_ERROR" => Some(Self::ContractValidateError),
                "CONTRACT_EXE_ERROR" => Some(Self::ContractExeError),
                "BANDWITH_ERROR" => Some(Self::BandwithError),
                "DUP_TRANSACTION_ERROR" => Some(Self::DupTransactionError),
                "TAPOS_ERROR" => Some(Self::TaposError),
                "TOO_BIG_TRANSACTION_ERROR" => Some(Self::TooBigTransactionError),
                "TRANSACTION_EXPIRATION_ERROR" => Some(Self::TransactionExpirationError),
                "SERVER_BUSY" => Some(Self::ServerBusy),
                "NO_CONNECTION" => Some(Self::NoConnection),
                "NOT_ENOUGH_EFFECTIVE_CONNECTION" => Some(Self::NotEnoughEffectiveConnection),
                "BLOCK_UNSOLIDIFIED" => Some(Self::BlockUnsolidified),
                "OTHER_ERROR" => Some(Self::OtherError),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for Return {
const NAME: &'static str = "Return";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockReference {
    #[prost(int64, tag="1")]
    pub block_num: i64,
    #[prost(bytes="vec", tag="2")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for BlockReference {
const NAME: &'static str = "BlockReference";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WitnessList {
    #[prost(message, repeated, tag="1")]
    pub witnesses: ::prost::alloc::vec::Vec<Witness>,
}
impl ::prost::Name for WitnessList {
const NAME: &'static str = "WitnessList";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalList {
    #[prost(message, repeated, tag="1")]
    pub proposals: ::prost::alloc::vec::Vec<Proposal>,
}
impl ::prost::Name for ProposalList {
const NAME: &'static str = "ProposalList";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeList {
    #[prost(message, repeated, tag="1")]
    pub exchanges: ::prost::alloc::vec::Vec<Exchange>,
}
impl ::prost::Name for ExchangeList {
const NAME: &'static str = "ExchangeList";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetIssueList {
    #[prost(message, repeated, tag="1")]
    pub asset_issue: ::prost::alloc::vec::Vec<AssetIssueContract>,
}
impl ::prost::Name for AssetIssueList {
const NAME: &'static str = "AssetIssueList";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockList {
    #[prost(message, repeated, tag="1")]
    pub block: ::prost::alloc::vec::Vec<Block>,
}
impl ::prost::Name for BlockList {
const NAME: &'static str = "BlockList";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionList {
    #[prost(message, repeated, tag="1")]
    pub transaction: ::prost::alloc::vec::Vec<Transaction>,
}
impl ::prost::Name for TransactionList {
const NAME: &'static str = "TransactionList";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionIdList {
    #[prost(string, repeated, tag="1")]
    pub tx_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for TransactionIdList {
const NAME: &'static str = "TransactionIdList";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatedResourceMessage {
    #[prost(bytes="vec", tag="1")]
    pub from_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub to_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for DelegatedResourceMessage {
const NAME: &'static str = "DelegatedResourceMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatedResourceList {
    #[prost(message, repeated, tag="1")]
    pub delegated_resource: ::prost::alloc::vec::Vec<DelegatedResource>,
}
impl ::prost::Name for DelegatedResourceList {
const NAME: &'static str = "DelegatedResourceList";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAvailableUnfreezeCountRequestMessage {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for GetAvailableUnfreezeCountRequestMessage {
const NAME: &'static str = "GetAvailableUnfreezeCountRequestMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAvailableUnfreezeCountResponseMessage {
    #[prost(int64, tag="1")]
    pub count: i64,
}
impl ::prost::Name for GetAvailableUnfreezeCountResponseMessage {
const NAME: &'static str = "GetAvailableUnfreezeCountResponseMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanDelegatedMaxSizeRequestMessage {
    #[prost(int32, tag="1")]
    pub r#type: i32,
    #[prost(bytes="vec", tag="2")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for CanDelegatedMaxSizeRequestMessage {
const NAME: &'static str = "CanDelegatedMaxSizeRequestMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanDelegatedMaxSizeResponseMessage {
    #[prost(int64, tag="1")]
    pub max_size: i64,
}
impl ::prost::Name for CanDelegatedMaxSizeResponseMessage {
const NAME: &'static str = "CanDelegatedMaxSizeResponseMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanWithdrawUnfreezeAmountRequestMessage {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub timestamp: i64,
}
impl ::prost::Name for CanWithdrawUnfreezeAmountRequestMessage {
const NAME: &'static str = "CanWithdrawUnfreezeAmountRequestMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanWithdrawUnfreezeAmountResponseMessage {
    #[prost(int64, tag="1")]
    pub amount: i64,
}
impl ::prost::Name for CanWithdrawUnfreezeAmountResponseMessage {
const NAME: &'static str = "CanWithdrawUnfreezeAmountResponseMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PricesResponseMessage {
    #[prost(string, tag="1")]
    pub prices: ::prost::alloc::string::String,
}
impl ::prost::Name for PricesResponseMessage {
const NAME: &'static str = "PricesResponseMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
/// Gossip node list
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeList {
    #[prost(message, repeated, tag="1")]
    pub nodes: ::prost::alloc::vec::Vec<Node>,
}
impl ::prost::Name for NodeList {
const NAME: &'static str = "NodeList";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
/// Gossip node
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    #[prost(message, optional, tag="1")]
    pub address: ::core::option::Option<Address>,
}
impl ::prost::Name for Node {
const NAME: &'static str = "Node";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
/// Gossip node address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(bytes="vec", tag="1")]
    pub host: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="2")]
    pub port: i32,
}
impl ::prost::Name for Address {
const NAME: &'static str = "Address";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyMessage {
}
impl ::prost::Name for EmptyMessage {
const NAME: &'static str = "EmptyMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumberMessage {
    #[prost(int64, tag="1")]
    pub num: i64,
}
impl ::prost::Name for NumberMessage {
const NAME: &'static str = "NumberMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesMessage {
    #[prost(bytes="vec", tag="1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for BytesMessage {
const NAME: &'static str = "BytesMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeMessage {
    #[prost(int64, tag="1")]
    pub begin_in_milliseconds: i64,
    #[prost(int64, tag="2")]
    pub end_in_milliseconds: i64,
}
impl ::prost::Name for TimeMessage {
const NAME: &'static str = "TimeMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockReq {
    #[prost(string, tag="1")]
    pub id_or_num: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub detail: bool,
}
impl ::prost::Name for BlockReq {
const NAME: &'static str = "BlockReq";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockLimit {
    #[prost(int64, tag="1")]
    pub start_num: i64,
    #[prost(int64, tag="2")]
    pub end_num: i64,
}
impl ::prost::Name for BlockLimit {
const NAME: &'static str = "BlockLimit";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionLimit {
    #[prost(bytes="vec", tag="1")]
    pub transaction_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub limit_num: i64,
}
impl ::prost::Name for TransactionLimit {
const NAME: &'static str = "TransactionLimit";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountPaginated {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<Account>,
    #[prost(int64, tag="2")]
    pub offset: i64,
    #[prost(int64, tag="3")]
    pub limit: i64,
}
impl ::prost::Name for AccountPaginated {
const NAME: &'static str = "AccountPaginated";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimePaginatedMessage {
    #[prost(message, optional, tag="1")]
    pub time_message: ::core::option::Option<TimeMessage>,
    #[prost(int64, tag="2")]
    pub offset: i64,
    #[prost(int64, tag="3")]
    pub limit: i64,
}
impl ::prost::Name for TimePaginatedMessage {
const NAME: &'static str = "TimePaginatedMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
/// deprecated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountNetMessage {
    #[prost(int64, tag="1")]
    pub free_net_used: i64,
    #[prost(int64, tag="2")]
    pub free_net_limit: i64,
    #[prost(int64, tag="3")]
    pub net_used: i64,
    #[prost(int64, tag="4")]
    pub net_limit: i64,
    #[prost(map="string, int64", tag="5")]
    pub asset_net_used: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    #[prost(map="string, int64", tag="6")]
    pub asset_net_limit: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    #[prost(int64, tag="7")]
    pub total_net_limit: i64,
    #[prost(int64, tag="8")]
    pub total_net_weight: i64,
}
impl ::prost::Name for AccountNetMessage {
const NAME: &'static str = "AccountNetMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountResourceMessage {
    #[prost(int64, tag="1")]
    pub free_net_used: i64,
    #[prost(int64, tag="2")]
    pub free_net_limit: i64,
    #[prost(int64, tag="3")]
    pub net_used: i64,
    #[prost(int64, tag="4")]
    pub net_limit: i64,
    #[prost(map="string, int64", tag="5")]
    pub asset_net_used: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    #[prost(map="string, int64", tag="6")]
    pub asset_net_limit: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    #[prost(int64, tag="7")]
    pub total_net_limit: i64,
    #[prost(int64, tag="8")]
    pub total_net_weight: i64,
    #[prost(int64, tag="9")]
    pub total_tron_power_weight: i64,
    #[prost(int64, tag="10")]
    pub tron_power_used: i64,
    #[prost(int64, tag="11")]
    pub tron_power_limit: i64,
    #[prost(int64, tag="13")]
    pub energy_used: i64,
    #[prost(int64, tag="14")]
    pub energy_limit: i64,
    #[prost(int64, tag="15")]
    pub total_energy_limit: i64,
    #[prost(int64, tag="16")]
    pub total_energy_weight: i64,
    #[prost(int64, tag="21")]
    pub storage_used: i64,
    #[prost(int64, tag="22")]
    pub storage_limit: i64,
}
impl ::prost::Name for AccountResourceMessage {
const NAME: &'static str = "AccountResourceMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaginatedMessage {
    #[prost(int64, tag="1")]
    pub offset: i64,
    #[prost(int64, tag="2")]
    pub limit: i64,
}
impl ::prost::Name for PaginatedMessage {
const NAME: &'static str = "PaginatedMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionExtention {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<Transaction>,
    /// transaction id =  sha256(transaction.rowdata)
    #[prost(bytes="vec", tag="2")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub constant_result: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="4")]
    pub result: ::core::option::Option<Return>,
    #[prost(int64, tag="5")]
    pub energy_used: i64,
    #[prost(message, repeated, tag="6")]
    pub logs: ::prost::alloc::vec::Vec<transaction_info::Log>,
    #[prost(message, repeated, tag="7")]
    pub internal_transactions: ::prost::alloc::vec::Vec<InternalTransaction>,
    #[prost(int64, tag="8")]
    pub energy_penalty: i64,
}
impl ::prost::Name for TransactionExtention {
const NAME: &'static str = "TransactionExtention";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateEnergyMessage {
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<Return>,
    #[prost(int64, tag="2")]
    pub energy_required: i64,
}
impl ::prost::Name for EstimateEnergyMessage {
const NAME: &'static str = "EstimateEnergyMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockExtention {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<TransactionExtention>,
    #[prost(message, optional, tag="2")]
    pub block_header: ::core::option::Option<BlockHeader>,
    #[prost(bytes="vec", tag="3")]
    pub blockid: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for BlockExtention {
const NAME: &'static str = "BlockExtention";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockListExtention {
    #[prost(message, repeated, tag="1")]
    pub block: ::prost::alloc::vec::Vec<BlockExtention>,
}
impl ::prost::Name for BlockListExtention {
const NAME: &'static str = "BlockListExtention";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionListExtention {
    #[prost(message, repeated, tag="1")]
    pub transaction: ::prost::alloc::vec::Vec<TransactionExtention>,
}
impl ::prost::Name for TransactionListExtention {
const NAME: &'static str = "TransactionListExtention";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockIncrementalMerkleTree {
    #[prost(int64, tag="1")]
    pub number: i64,
    #[prost(message, optional, tag="2")]
    pub merkle_tree: ::core::option::Option<IncrementalMerkleTree>,
}
impl ::prost::Name for BlockIncrementalMerkleTree {
const NAME: &'static str = "BlockIncrementalMerkleTree";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionSignWeight {
    #[prost(message, optional, tag="1")]
    pub permission: ::core::option::Option<Permission>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub approved_list: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(int64, tag="3")]
    pub current_weight: i64,
    #[prost(message, optional, tag="4")]
    pub result: ::core::option::Option<transaction_sign_weight::Result>,
    #[prost(message, optional, tag="5")]
    pub transaction: ::core::option::Option<TransactionExtention>,
}
/// Nested message and enum types in `TransactionSignWeight`.
pub mod transaction_sign_weight {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        #[prost(enumeration="result::ResponseCode", tag="1")]
        pub code: i32,
        #[prost(string, tag="2")]
        pub message: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Result`.
    pub mod result {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ResponseCode {
            EnoughPermission = 0,
            /// error in
            NotEnoughPermission = 1,
            SignatureFormatError = 2,
            ComputeAddressError = 3,
            /// The key is not in permission
            PermissionError = 4,
            OtherError = 20,
        }
        impl ResponseCode {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ResponseCode::EnoughPermission => "ENOUGH_PERMISSION",
                    ResponseCode::NotEnoughPermission => "NOT_ENOUGH_PERMISSION",
                    ResponseCode::SignatureFormatError => "SIGNATURE_FORMAT_ERROR",
                    ResponseCode::ComputeAddressError => "COMPUTE_ADDRESS_ERROR",
                    ResponseCode::PermissionError => "PERMISSION_ERROR",
                    ResponseCode::OtherError => "OTHER_ERROR",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "ENOUGH_PERMISSION" => Some(Self::EnoughPermission),
                    "NOT_ENOUGH_PERMISSION" => Some(Self::NotEnoughPermission),
                    "SIGNATURE_FORMAT_ERROR" => Some(Self::SignatureFormatError),
                    "COMPUTE_ADDRESS_ERROR" => Some(Self::ComputeAddressError),
                    "PERMISSION_ERROR" => Some(Self::PermissionError),
                    "OTHER_ERROR" => Some(Self::OtherError),
                    _ => None,
                }
            }
        }
    }
impl ::prost::Name for Result {
const NAME: &'static str = "Result";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.TransactionSignWeight.{}", Self::NAME)
            }}
}
impl ::prost::Name for TransactionSignWeight {
const NAME: &'static str = "TransactionSignWeight";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionApprovedList {
    #[prost(bytes="vec", repeated, tag="2")]
    pub approved_list: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="4")]
    pub result: ::core::option::Option<transaction_approved_list::Result>,
    #[prost(message, optional, tag="5")]
    pub transaction: ::core::option::Option<TransactionExtention>,
}
/// Nested message and enum types in `TransactionApprovedList`.
pub mod transaction_approved_list {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        #[prost(enumeration="result::ResponseCode", tag="1")]
        pub code: i32,
        #[prost(string, tag="2")]
        pub message: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Result`.
    pub mod result {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ResponseCode {
            Success = 0,
            SignatureFormatError = 1,
            ComputeAddressError = 2,
            OtherError = 20,
        }
        impl ResponseCode {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ResponseCode::Success => "SUCCESS",
                    ResponseCode::SignatureFormatError => "SIGNATURE_FORMAT_ERROR",
                    ResponseCode::ComputeAddressError => "COMPUTE_ADDRESS_ERROR",
                    ResponseCode::OtherError => "OTHER_ERROR",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "SUCCESS" => Some(Self::Success),
                    "SIGNATURE_FORMAT_ERROR" => Some(Self::SignatureFormatError),
                    "COMPUTE_ADDRESS_ERROR" => Some(Self::ComputeAddressError),
                    "OTHER_ERROR" => Some(Self::OtherError),
                    _ => None,
                }
            }
        }
    }
impl ::prost::Name for Result {
const NAME: &'static str = "Result";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.TransactionApprovedList.{}", Self::NAME)
            }}
}
impl ::prost::Name for TransactionApprovedList {
const NAME: &'static str = "TransactionApprovedList";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IvkDecryptParameters {
    #[prost(int64, tag="1")]
    pub start_block_index: i64,
    #[prost(int64, tag="2")]
    pub end_block_index: i64,
    #[prost(bytes="vec", tag="3")]
    pub ivk: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for IvkDecryptParameters {
const NAME: &'static str = "IvkDecryptParameters";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IvkDecryptAndMarkParameters {
    #[prost(int64, tag="1")]
    pub start_block_index: i64,
    #[prost(int64, tag="2")]
    pub end_block_index: i64,
    #[prost(bytes="vec", tag="5")]
    pub ivk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub nk: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for IvkDecryptAndMarkParameters {
const NAME: &'static str = "IvkDecryptAndMarkParameters";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OvkDecryptParameters {
    #[prost(int64, tag="1")]
    pub start_block_index: i64,
    #[prost(int64, tag="2")]
    pub end_block_index: i64,
    #[prost(bytes="vec", tag="3")]
    pub ovk: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for OvkDecryptParameters {
const NAME: &'static str = "OvkDecryptParameters";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptNotes {
    #[prost(message, repeated, tag="1")]
    pub note_txs: ::prost::alloc::vec::Vec<decrypt_notes::NoteTx>,
}
/// Nested message and enum types in `DecryptNotes`.
pub mod decrypt_notes {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NoteTx {
        #[prost(message, optional, tag="1")]
        pub note: ::core::option::Option<super::Note>,
        /// transaction id =  sha256(transaction.rowdata)
        #[prost(bytes="vec", tag="2")]
        pub txid: ::prost::alloc::vec::Vec<u8>,
        /// the index of note in receive
        #[prost(int32, tag="3")]
        pub index: i32,
    }
impl ::prost::Name for NoteTx {
const NAME: &'static str = "NoteTx";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.DecryptNotes.{}", Self::NAME)
            }}
}
impl ::prost::Name for DecryptNotes {
const NAME: &'static str = "DecryptNotes";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptNotesMarked {
    #[prost(message, repeated, tag="1")]
    pub note_txs: ::prost::alloc::vec::Vec<decrypt_notes_marked::NoteTx>,
}
/// Nested message and enum types in `DecryptNotesMarked`.
pub mod decrypt_notes_marked {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NoteTx {
        #[prost(message, optional, tag="1")]
        pub note: ::core::option::Option<super::Note>,
        /// transaction id =  sha256(transaction.rowdata)
        #[prost(bytes="vec", tag="2")]
        pub txid: ::prost::alloc::vec::Vec<u8>,
        /// the index of note in receive
        #[prost(int32, tag="3")]
        pub index: i32,
        #[prost(bool, tag="4")]
        pub is_spend: bool,
    }
impl ::prost::Name for NoteTx {
const NAME: &'static str = "NoteTx";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.DecryptNotesMarked.{}", Self::NAME)
            }}
}
impl ::prost::Name for DecryptNotesMarked {
const NAME: &'static str = "DecryptNotesMarked";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Note {
    #[prost(int64, tag="1")]
    pub value: i64,
    #[prost(string, tag="2")]
    pub payment_address: ::prost::alloc::string::String,
    /// random 32
    #[prost(bytes="vec", tag="3")]
    pub rcm: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub memo: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for Note {
const NAME: &'static str = "Note";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendNote {
    #[prost(message, optional, tag="3")]
    pub note: ::core::option::Option<Note>,
    /// random number for spend authority signature
    #[prost(bytes="vec", tag="4")]
    pub alpha: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="5")]
    pub voucher: ::core::option::Option<IncrementalMerkleVoucher>,
    /// path for cm from leaf to root in merkle tree
    #[prost(bytes="vec", tag="6")]
    pub path: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for SpendNote {
const NAME: &'static str = "SpendNote";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveNote {
    #[prost(message, optional, tag="1")]
    pub note: ::core::option::Option<Note>,
}
impl ::prost::Name for ReceiveNote {
const NAME: &'static str = "ReceiveNote";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateParameters {
    #[prost(bytes="vec", tag="1")]
    pub transparent_from_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub ask: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub nsk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub ovk: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub from_amount: i64,
    #[prost(message, repeated, tag="6")]
    pub shielded_spends: ::prost::alloc::vec::Vec<SpendNote>,
    #[prost(message, repeated, tag="7")]
    pub shielded_receives: ::prost::alloc::vec::Vec<ReceiveNote>,
    #[prost(bytes="vec", tag="8")]
    pub transparent_to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="9")]
    pub to_amount: i64,
    /// timeout in seconds, it works only when it bigger than 0
    #[prost(int64, tag="10")]
    pub timeout: i64,
}
impl ::prost::Name for PrivateParameters {
const NAME: &'static str = "PrivateParameters";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateParametersWithoutAsk {
    #[prost(bytes="vec", tag="1")]
    pub transparent_from_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub nsk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub ovk: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub from_amount: i64,
    #[prost(message, repeated, tag="6")]
    pub shielded_spends: ::prost::alloc::vec::Vec<SpendNote>,
    #[prost(message, repeated, tag="7")]
    pub shielded_receives: ::prost::alloc::vec::Vec<ReceiveNote>,
    #[prost(bytes="vec", tag="8")]
    pub transparent_to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="9")]
    pub to_amount: i64,
    /// timeout in seconds, it works only when it bigger than 0
    #[prost(int64, tag="10")]
    pub timeout: i64,
}
impl ::prost::Name for PrivateParametersWithoutAsk {
const NAME: &'static str = "PrivateParametersWithoutAsk";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendAuthSigParameters {
    #[prost(bytes="vec", tag="1")]
    pub ask: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub tx_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub alpha: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for SpendAuthSigParameters {
const NAME: &'static str = "SpendAuthSigParameters";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NfParameters {
    #[prost(message, optional, tag="1")]
    pub note: ::core::option::Option<Note>,
    #[prost(message, optional, tag="2")]
    pub voucher: ::core::option::Option<IncrementalMerkleVoucher>,
    #[prost(bytes="vec", tag="3")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub nk: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for NfParameters {
const NAME: &'static str = "NfParameters";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandedSpendingKeyMessage {
    #[prost(bytes="vec", tag="1")]
    pub ask: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub nsk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub ovk: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ExpandedSpendingKeyMessage {
const NAME: &'static str = "ExpandedSpendingKeyMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewingKeyMessage {
    #[prost(bytes="vec", tag="1")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub nk: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ViewingKeyMessage {
const NAME: &'static str = "ViewingKeyMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncomingViewingKeyMessage {
    #[prost(bytes="vec", tag="1")]
    pub ivk: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for IncomingViewingKeyMessage {
const NAME: &'static str = "IncomingViewingKeyMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiversifierMessage {
    #[prost(bytes="vec", tag="1")]
    pub d: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for DiversifierMessage {
const NAME: &'static str = "DiversifierMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncomingViewingKeyDiversifierMessage {
    #[prost(message, optional, tag="1")]
    pub ivk: ::core::option::Option<IncomingViewingKeyMessage>,
    #[prost(message, optional, tag="2")]
    pub d: ::core::option::Option<DiversifierMessage>,
}
impl ::prost::Name for IncomingViewingKeyDiversifierMessage {
const NAME: &'static str = "IncomingViewingKeyDiversifierMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentAddressMessage {
    #[prost(message, optional, tag="1")]
    pub d: ::core::option::Option<DiversifierMessage>,
    #[prost(bytes="vec", tag="2")]
    pub pk_d: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub payment_address: ::prost::alloc::string::String,
}
impl ::prost::Name for PaymentAddressMessage {
const NAME: &'static str = "PaymentAddressMessage";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShieldedAddressInfo {
    #[prost(bytes="vec", tag="1")]
    pub sk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub ask: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub nsk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub ovk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub nk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub ivk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub d: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub pk_d: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="10")]
    pub payment_address: ::prost::alloc::string::String,
}
impl ::prost::Name for ShieldedAddressInfo {
const NAME: &'static str = "ShieldedAddressInfo";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoteParameters {
    #[prost(bytes="vec", tag="1")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub nk: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub note: ::core::option::Option<Note>,
    #[prost(bytes="vec", tag="4")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="5")]
    pub index: i32,
}
impl ::prost::Name for NoteParameters {
const NAME: &'static str = "NoteParameters";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendResult {
    #[prost(bool, tag="1")]
    pub result: bool,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
impl ::prost::Name for SpendResult {
const NAME: &'static str = "SpendResult";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfoList {
    #[prost(message, repeated, tag="1")]
    pub transaction_info: ::prost::alloc::vec::Vec<TransactionInfo>,
}
impl ::prost::Name for TransactionInfoList {
const NAME: &'static str = "TransactionInfoList";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendNoteTrc20 {
    #[prost(message, optional, tag="1")]
    pub note: ::core::option::Option<Note>,
    #[prost(bytes="vec", tag="2")]
    pub alpha: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub path: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub pos: i64,
}
impl ::prost::Name for SpendNoteTrc20 {
const NAME: &'static str = "SpendNoteTRC20";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateShieldedTrc20Parameters {
    #[prost(bytes="vec", tag="1")]
    pub ask: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub nsk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub ovk: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub from_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="5")]
    pub shielded_spends: ::prost::alloc::vec::Vec<SpendNoteTrc20>,
    #[prost(message, repeated, tag="6")]
    pub shielded_receives: ::prost::alloc::vec::Vec<ReceiveNote>,
    #[prost(bytes="vec", tag="7")]
    pub transparent_to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub to_amount: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="9")]
    pub shielded_trc20_contract_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for PrivateShieldedTrc20Parameters {
const NAME: &'static str = "PrivateShieldedTRC20Parameters";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateShieldedTrc20ParametersWithoutAsk {
    #[prost(bytes="vec", tag="1")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub nsk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub ovk: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub from_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="5")]
    pub shielded_spends: ::prost::alloc::vec::Vec<SpendNoteTrc20>,
    #[prost(message, repeated, tag="6")]
    pub shielded_receives: ::prost::alloc::vec::Vec<ReceiveNote>,
    #[prost(bytes="vec", tag="7")]
    pub transparent_to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub to_amount: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="9")]
    pub shielded_trc20_contract_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for PrivateShieldedTrc20ParametersWithoutAsk {
const NAME: &'static str = "PrivateShieldedTRC20ParametersWithoutAsk";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShieldedTrc20Parameters {
    #[prost(message, repeated, tag="1")]
    pub spend_description: ::prost::alloc::vec::Vec<SpendDescription>,
    #[prost(message, repeated, tag="2")]
    pub receive_description: ::prost::alloc::vec::Vec<ReceiveDescription>,
    #[prost(bytes="vec", tag="3")]
    pub binding_signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub message_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub trigger_contract_input: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub parameter_type: ::prost::alloc::string::String,
}
impl ::prost::Name for ShieldedTrc20Parameters {
const NAME: &'static str = "ShieldedTRC20Parameters";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IvkDecryptTrc20Parameters {
    #[prost(int64, tag="1")]
    pub start_block_index: i64,
    #[prost(int64, tag="2")]
    pub end_block_index: i64,
    #[prost(bytes="vec", tag="3")]
    pub shielded_trc20_contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub ivk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub nk: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="7")]
    pub events: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for IvkDecryptTrc20Parameters {
const NAME: &'static str = "IvkDecryptTRC20Parameters";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OvkDecryptTrc20Parameters {
    #[prost(int64, tag="1")]
    pub start_block_index: i64,
    #[prost(int64, tag="2")]
    pub end_block_index: i64,
    #[prost(bytes="vec", tag="3")]
    pub ovk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub shielded_trc20_contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="5")]
    pub events: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for OvkDecryptTrc20Parameters {
const NAME: &'static str = "OvkDecryptTRC20Parameters";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptNotesTrc20 {
    #[prost(message, repeated, tag="1")]
    pub note_txs: ::prost::alloc::vec::Vec<decrypt_notes_trc20::NoteTx>,
}
/// Nested message and enum types in `DecryptNotesTRC20`.
pub mod decrypt_notes_trc20 {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NoteTx {
        #[prost(message, optional, tag="1")]
        pub note: ::core::option::Option<super::Note>,
        #[prost(int64, tag="2")]
        pub position: i64,
        #[prost(bool, tag="3")]
        pub is_spent: bool,
        #[prost(bytes="vec", tag="4")]
        pub txid: ::prost::alloc::vec::Vec<u8>,
        /// the index of note in txid
        #[prost(int32, tag="5")]
        pub index: i32,
        #[prost(string, tag="6")]
        pub to_amount: ::prost::alloc::string::String,
        #[prost(bytes="vec", tag="7")]
        pub transparent_to_address: ::prost::alloc::vec::Vec<u8>,
    }
impl ::prost::Name for NoteTx {
const NAME: &'static str = "NoteTx";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.DecryptNotesTRC20.{}", Self::NAME)
            }}
}
impl ::prost::Name for DecryptNotesTrc20 {
const NAME: &'static str = "DecryptNotesTRC20";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NfTrc20Parameters {
    #[prost(message, optional, tag="1")]
    pub note: ::core::option::Option<Note>,
    #[prost(bytes="vec", tag="2")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub nk: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="4")]
    pub position: i64,
    #[prost(bytes="vec", tag="5")]
    pub shielded_trc20_contract_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for NfTrc20Parameters {
const NAME: &'static str = "NfTRC20Parameters";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NullifierResult {
    #[prost(bool, tag="1")]
    pub is_spent: bool,
}
impl ::prost::Name for NullifierResult {
const NAME: &'static str = "NullifierResult";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShieldedTrc20TriggerContractParameters {
    #[prost(message, optional, tag="1")]
    pub shielded_trc20_parameters: ::core::option::Option<ShieldedTrc20Parameters>,
    #[prost(message, repeated, tag="2")]
    pub spend_authority_signature: ::prost::alloc::vec::Vec<BytesMessage>,
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub transparent_to_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ShieldedTrc20TriggerContractParameters {
const NAME: &'static str = "ShieldedTRC20TriggerContractParameters";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZksnarkRequest {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<Transaction>,
    #[prost(bytes="vec", tag="2")]
    pub sighash: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub value_balance: i64,
    #[prost(string, tag="4")]
    pub tx_id: ::prost::alloc::string::String,
}
impl ::prost::Name for ZksnarkRequest {
const NAME: &'static str = "ZksnarkRequest";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZksnarkResponse {
    #[prost(enumeration="zksnark_response::Code", tag="1")]
    pub code: i32,
}
/// Nested message and enum types in `ZksnarkResponse`.
pub mod zksnark_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        Success = 0,
        Failed = 1,
    }
    impl Code {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Code::Success => "SUCCESS",
                Code::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUCCESS" => Some(Self::Success),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for ZksnarkResponse {
const NAME: &'static str = "ZksnarkResponse";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InventoryItems {
    #[prost(int32, tag="1")]
    pub r#type: i32,
    #[prost(bytes="vec", repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
impl ::prost::Name for InventoryItems {
const NAME: &'static str = "InventoryItems";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteAssetContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub vote_address: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, tag="3")]
    pub support: bool,
    #[prost(int32, tag="5")]
    pub count: i32,
}
impl ::prost::Name for VoteAssetContract {
const NAME: &'static str = "VoteAssetContract";
const PACKAGE: &'static str = "protocol";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("protocol.{}", Self::NAME)
            }}
// @@protoc_insertion_point(module)