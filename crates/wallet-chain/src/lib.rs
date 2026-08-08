//! Multi-chain capability traits and family implementations.

mod provider;
mod registry;
mod traits;

#[cfg(feature = "bitcoin")]
pub mod bitcoin;
#[cfg(feature = "evm")]
pub mod evm;
#[cfg(feature = "solana")]
pub mod solana;
#[cfg(feature = "tron")]
pub mod tron;
#[cfg(feature = "tron")]
mod ton;

pub use provider::{CircuitBreaker, RpcPool};
pub use registry::{ChainHandle, ChainRegistry};
pub use traits::{
    BalanceReader, BlockSource, GasEstimator, NonceProvider, TokenBalance, TxBroadcaster,
};

use wallet_config::ChainRuntimeConfig;
use wallet_error::{AppError, AppResult};
use wallet_types::{ChainFamily, ChainIndex};

pub fn build_registry(chains: &[ChainRuntimeConfig]) -> AppResult<ChainRegistry> {
    let mut registry = ChainRegistry::default();
    for cfg in chains {
        let index = ChainIndex(cfg.chain_index);
        let family = match cfg.family.as_str() {
            "evm" => ChainFamily::Evm,
            "solana" => ChainFamily::Solana,
            "bitcoin" => ChainFamily::Bitcoin,
            "tron" => ChainFamily::Tron,
            other => {
                return Err(AppError::InvalidArgument(format!(
                    "unknown chain family: {other}"
                )))
            }
        };
        let handle = match family {
            #[cfg(feature = "evm")]
            ChainFamily::Evm => ChainHandle::Evm(evm::EvmChain::new(cfg)?),
            #[cfg(feature = "solana")]
            ChainFamily::Solana => ChainHandle::Solana(solana::SolanaChain::new(cfg)?),
            #[cfg(feature = "bitcoin")]
            ChainFamily::Bitcoin => ChainHandle::Bitcoin(bitcoin::BitcoinChain::new(cfg)?),
            #[cfg(feature = "tron")]
            ChainFamily::Tron => ChainHandle::Tron(tron::TronChain::new(cfg)?),
            _ => {
                return Err(AppError::ChainNotSupported(index));
            }
        };
        registry.insert(index, handle);
    }
    Ok(registry)
}
