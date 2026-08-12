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
    /// TON (The Open Network), SLIP-0044 coin type `607`.
    pub const TON: Self = Self(607);
    /// Sui, SLIP-0044 coin type `784`.
    pub const SUI: Self = Self(784);

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
    /// Zcash. JSON-RPC is zcashd/Bitcoin-compatible, but `getblock` only
    /// supports verbosity 2 (no `prevout`), and shielded transactions are not
    /// addressable — only transparent (t-addr) transfers are indexed.
    Zcash,
    /// TON (The Open Network). JSON-RPC uses the v3 API shape.
    Ton,
    /// Sui. JSON-RPC uses the Sui JSON-RPC shape.
    Sui,
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
            ChainIndex::DOGE => Some(Self::UtxoOther),
            ChainIndex::ZCASH => Some(Self::Zcash),
            ChainIndex::TRON => Some(Self::Tron),
            ChainIndex::TON => Some(Self::Ton),
            ChainIndex::SUI => Some(Self::Sui),
            _ => None,
        }
    }

    /// Canonical string form used across the platform (DB rows, gateway
    /// probing, sync parsers). UTXO-family chains other than Bitcoin share the
    /// `bitcoin` protocol semantics (no EVM-style JSON-RPC). Zcash is a
    /// separate family: its zcashd RPC diverges from Bitcoin Core (verbosity-2
    /// `getblock`, no `prevout`, shielded txs).
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Evm => "evm",
            Self::Solana => "solana",
            Self::Bitcoin | Self::UtxoOther => "bitcoin",
            Self::Tron => "tron",
            Self::Zcash => "zcash",
            Self::Ton => "ton",
            Self::Sui => "sui",
        }
    }
}

/// Opaque chain address (hex / base58 / bech32 string).
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// Convert a `u128` to `Decimal`, saturating at `Decimal::MAX` instead of
/// panicking. `Decimal::from(u128)` panics for values above 2^96-1.
pub fn decimal_from_u128(v: u128) -> Decimal {
    if v >> 96 != 0 {
        Decimal::MAX
    } else {
        Decimal::from(v)
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

impl TxStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Success => "success",
            Self::Failed => "failed",
            Self::Dropped => "dropped",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedTx {
    pub hash: TxHash,
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub value: Amount,
    /// Native gas fee paid for the tx (raw integer, decimals per chain).
    pub gas_fee: Option<Amount>,
    pub block_number: u64,
    pub status: TxStatus,
    pub raw: serde_json::Value,
    /// Token contract / mint address for token transfers (ERC20, SPL, TRC20);
    /// `None` for native transfers.
    pub contract_address: Option<Address>,
    /// Index that disambiguates multiple records sharing the same tx hash and
    /// contract. For EVM/TRC20 this is the event `logIndex`. UTXO-family
    /// chains reuse it as the `vout` index for output records and a *negative*
    /// `-(vin_pos + 2)` for input/spend records, so both directions coexist
    /// under the `(chain_index, hash, contract_address, log_index)` unique key.
    pub log_index: Option<i64>,
    /// Method/selector that triggered the transfer (e.g. `transfer`,
    /// `transferFrom`, `approve`, or a raw 4-byte selector).
    pub method: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GasEstimateRequest {
    pub from: Address,
    pub to: Option<Address>,
    pub data: Option<Vec<u8>>,
    pub value: Option<Amount>,
    /// Family-specific extensions. The shape is family-defined:
    ///
    /// - `sui`: `{"txBytes": "<base64>", "signatures": ["<base64>", ...]}`
    ///   — required for `suix_dryRunTransactionBlock` (a fully-formed
    ///   transaction block, not just a payload).
    /// - `ton`: `{"messageBoc": "<base64>"}` — the message BOC to estimate
    ///   fees for. When omitted, the estimator falls back to a heuristic
    ///   from the wallet address alone.
    /// - `evm`/`bitcoin`/`solana`/`tron`: ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GasEstimate {
    pub gas_limit: u64,
    pub max_fee_per_gas: Option<u128>,
    pub max_priority_fee_per_gas: Option<u128>,
    /// Reference gas price (lowest unit, e.g. MIST on Sui, nanoTON on TON).
    /// For EVM, this is `max_fee_per_gas` in wei; on Sui it is the
    /// `suix_getReferenceGasPrice` value; on TON it is the workchain
    /// `gas_price` from `getConfigParam 18`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<u128>,
    /// Total estimated fee in the chain's lowest unit. On Sui this is
    /// `computationCost + storageCost - storageRebate`; on TON this is the
    /// `runGetMethod`/config-derivation result in nanotons.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_native: Option<u128>,
    /// Number of gas units the execution is expected to consume (Sui
    /// computation units, or TON message-gas units). EVM uses
    /// `gas_limit` instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gas_used: Option<u64>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_from_u128_saturates_above_2_96() {
        assert_eq!(decimal_from_u128(0), Decimal::ZERO);
        assert_eq!(decimal_from_u128(1 << 96), Decimal::MAX);
        assert_eq!(decimal_from_u128(u128::MAX), Decimal::MAX);
        let max = (1u128 << 96) - 1;
        assert_eq!(decimal_from_u128(max), Decimal::from(max));
    }
}
