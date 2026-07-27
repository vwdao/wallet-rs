//! Shared newtypes and chain identity.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// SLIP-44 style chain index used across the platform (matches Go constants).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChainIndex(pub i64);

impl ChainIndex {
    pub const BTC: Self = Self(0);
    pub const DOGE: Self = Self(3);
    pub const ETH: Self = Self(60);
    pub const SOL: Self = Self(501);
    pub const BSC: Self = Self(20000714);
    pub const POL: Self = Self(966);
    pub const BASE: Self = Self(8453);
    pub const ARB: Self = Self(10042221);
    pub const OP: Self = Self(10000070);
    pub const AVAX: Self = Self(10009000);
    pub const TRON: Self = Self(195);
    pub const ZCASH: Self = Self(133);
    pub const HYPERLIQUID: Self = Self(10000999);
    pub const ROBINHOOD: Self = Self(10004663);

    pub fn as_i64(self) -> i64 {
        self.0
    }
}

impl fmt::Display for ChainIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChainFamily {
    Evm,
    Solana,
    Bitcoin,
    Tron,
    UtxoOther,
}

impl ChainFamily {
    pub fn for_index(index: ChainIndex) -> Option<Self> {
        match index {
            ChainIndex::ETH
            | ChainIndex::BSC
            | ChainIndex::POL
            | ChainIndex::BASE
            | ChainIndex::ARB
            | ChainIndex::OP
            | ChainIndex::AVAX
            | ChainIndex::HYPERLIQUID
            | ChainIndex::ROBINHOOD => Some(Self::Evm),
            ChainIndex::SOL => Some(Self::Solana),
            ChainIndex::BTC => Some(Self::Bitcoin),
            ChainIndex::DOGE | ChainIndex::ZCASH => Some(Self::UtxoOther),
            ChainIndex::TRON => Some(Self::Tron),
            _ => None,
        }
    }
}

/// Opaque chain address (hex / base58 / bech32 string).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Address(pub String);

impl Address {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TxHash(pub String);

impl TxHash {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TxHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Amount {
    pub raw: Decimal,
    pub decimals: u32,
}

impl Amount {
    pub fn new(raw: Decimal, decimals: u32) -> Self {
        Self { raw, decimals }
    }

    pub fn zero(decimals: u32) -> Self {
        Self {
            raw: Decimal::ZERO,
            decimals,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TxStatus {
    Pending,
    Success,
    Failed,
    Dropped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedTx {
    pub hash: TxHash,
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub value: Amount,
    pub block_number: u64,
    pub status: TxStatus,
    pub raw: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasEstimateRequest {
    pub from: Address,
    pub to: Option<Address>,
    pub data: Option<Vec<u8>>,
    pub value: Option<Amount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasEstimate {
    pub gas_limit: u64,
    pub max_fee_per_gas: Option<u128>,
    pub max_priority_fee_per_gas: Option<u128>,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    #[error("invalid chain index: {0}")]
    ChainIndex(String),
}

impl FromStr for ChainIndex {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i64>()
            .map(ChainIndex)
            .map_err(|_| ParseError::ChainIndex(s.to_string()))
    }
}
