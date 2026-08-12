use crate::traits::{
    BalanceReader, BlockSource, GasEstimator, NonceProvider, TokenBalance, TxBroadcaster,
};
use crate::{bitcoin, evm, solana, sui, ton, tron, zcash};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use wallet_error::{AppError, AppResult};
use wallet_types::{
    Address, Amount, ChainIndex, GasEstimate, GasEstimateRequest, NormalizedTx, TxHash,
};

#[derive(Clone)]
pub enum ChainHandle {
    #[cfg(feature = "evm")]
    Evm(Arc<evm::EvmChain>),
    #[cfg(feature = "solana")]
    Solana(Arc<solana::SolanaChain>),
    #[cfg(feature = "bitcoin")]
    Bitcoin(Arc<bitcoin::BitcoinChain>),
    #[cfg(feature = "zcash")]
    Zcash(Arc<zcash::ZcashChain>),
    #[cfg(feature = "tron")]
    Tron(Arc<tron::TronChain>),
    #[cfg(feature = "ton")]
    Ton(Arc<ton::TonChain>),
    #[cfg(feature = "sui")]
    Sui(Arc<sui::SuiChain>),
}

impl ChainHandle {
    pub fn rpc_url(&self) -> AppResult<String> {
        match self {
            #[cfg(feature = "evm")]
            Self::Evm(c) => c.pool.next_url().map(|u| u.to_string()),
            #[cfg(feature = "solana")]
            Self::Solana(c) => c.pool.next_url().map(|u| u.to_string()),
            #[cfg(feature = "bitcoin")]
            Self::Bitcoin(c) => c.pool.next_url().map(|u| u.to_string()),
            #[cfg(feature = "zcash")]
            Self::Zcash(c) => c.pool.next_url().map(|u| u.to_string()),
            #[cfg(feature = "tron")]
            Self::Tron(c) => c.pool.next_url().map(|u| u.to_string()),
            #[cfg(feature = "ton")]
            Self::Ton(c) => c.pool.next_url().map(|u| u.to_string()),
            #[cfg(feature = "sui")]
            Self::Sui(c) => c.pool.next_url().map(|u| u.to_string()),
        }
    }

    pub fn as_balance_reader(&self) -> &dyn BalanceReader {
        match self {
            #[cfg(feature = "evm")]
            Self::Evm(c) => c.as_ref(),
            #[cfg(feature = "solana")]
            Self::Solana(c) => c.as_ref(),
            #[cfg(feature = "bitcoin")]
            Self::Bitcoin(c) => c.as_ref(),
            #[cfg(feature = "zcash")]
            Self::Zcash(c) => c.as_ref(),
            #[cfg(feature = "tron")]
            Self::Tron(c) => c.as_ref(),
            #[cfg(feature = "ton")]
            Self::Ton(c) => c.as_ref(),
            #[cfg(feature = "sui")]
            Self::Sui(c) => c.as_ref(),
        }
    }

    pub fn as_block_source(&self) -> &dyn BlockSource {
        match self {
            #[cfg(feature = "evm")]
            Self::Evm(c) => c.as_ref(),
            #[cfg(feature = "solana")]
            Self::Solana(c) => c.as_ref(),
            #[cfg(feature = "bitcoin")]
            Self::Bitcoin(c) => c.as_ref(),
            #[cfg(feature = "zcash")]
            Self::Zcash(c) => c.as_ref(),
            #[cfg(feature = "tron")]
            Self::Tron(c) => c.as_ref(),
            #[cfg(feature = "ton")]
            Self::Ton(c) => c.as_ref(),
            #[cfg(feature = "sui")]
            Self::Sui(c) => c.as_ref(),
        }
    }

    pub fn as_tx_broadcaster(&self) -> &dyn TxBroadcaster {
        match self {
            #[cfg(feature = "evm")]
            Self::Evm(c) => c.as_ref(),
            #[cfg(feature = "solana")]
            Self::Solana(c) => c.as_ref(),
            #[cfg(feature = "bitcoin")]
            Self::Bitcoin(c) => c.as_ref(),
            #[cfg(feature = "zcash")]
            Self::Zcash(c) => c.as_ref(),
            #[cfg(feature = "tron")]
            Self::Tron(c) => c.as_ref(),
            #[cfg(feature = "ton")]
            Self::Ton(c) => c.as_ref(),
            #[cfg(feature = "sui")]
            Self::Sui(c) => c.as_ref(),
        }
    }

    pub fn as_gas_estimator(&self) -> &dyn GasEstimator {
        match self {
            #[cfg(feature = "evm")]
            Self::Evm(c) => c.as_ref(),
            #[cfg(feature = "solana")]
            Self::Solana(c) => c.as_ref(),
            #[cfg(feature = "bitcoin")]
            Self::Bitcoin(c) => c.as_ref(),
            #[cfg(feature = "zcash")]
            Self::Zcash(c) => c.as_ref(),
            #[cfg(feature = "tron")]
            Self::Tron(c) => c.as_ref(),
            #[cfg(feature = "ton")]
            Self::Ton(c) => c.as_ref(),
            #[cfg(feature = "sui")]
            Self::Sui(c) => c.as_ref(),
        }
    }

    pub async fn ata_address_rpc(
        &self,
        owner: &Address,
        mint: &Address,
    ) -> AppResult<(Address, bool)> {
        match self {
            #[cfg(feature = "solana")]
            Self::Solana(c) => c.ata_address_rpc(owner, mint).await,
            _ => Err(AppError::Unimplemented(
                "ata_address_rpc for non-Solana chain".into(),
            )),
        }
    }

    pub fn ata_address(&self, owner: &Address, mint: &Address) -> AppResult<Address> {
        match self {
            #[cfg(feature = "solana")]
            Self::Solana(_) => Ok(solana::SolanaChain::ata_address(owner, mint)),
            _ => Err(AppError::Unimplemented(
                "ata_address for non-Solana chain".into(),
            )),
        }
    }
}

#[async_trait]
impl BalanceReader for ChainHandle {
    async fn native_balance(&self, addr: &Address) -> AppResult<Amount> {
        self.as_balance_reader().native_balance(addr).await
    }
}

#[async_trait]
impl TokenBalance for ChainHandle {
    async fn token_balance(&self, wallet: &Address, token: &Address) -> AppResult<Amount> {
        match self {
            #[cfg(feature = "evm")]
            Self::Evm(c) => c.token_balance(wallet, token).await,
            #[cfg(feature = "solana")]
            Self::Solana(c) => c.token_balance(wallet, token).await,
            #[cfg(feature = "bitcoin")]
            Self::Bitcoin(_) => Err(AppError::Unimplemented("bitcoin token balance".into())),
            #[cfg(feature = "zcash")]
            Self::Zcash(c) => c.token_balance(wallet, token).await,
            #[cfg(feature = "tron")]
            Self::Tron(c) => c.token_balance(wallet, token).await,
            #[cfg(feature = "ton")]
            Self::Ton(c) => c.token_balance(wallet, token).await,
            #[cfg(feature = "sui")]
            Self::Sui(c) => c.token_balance(wallet, token).await,
        }
    }
}

#[async_trait]
impl TxBroadcaster for ChainHandle {
    async fn send_raw(&self, raw: &[u8]) -> AppResult<TxHash> {
        self.as_tx_broadcaster().send_raw(raw).await
    }
}

#[async_trait]
impl BlockSource for ChainHandle {
    async fn tip(&self) -> AppResult<u64> {
        self.as_block_source().tip().await
    }

    async fn fetch_block_txs(&self, height: u64) -> AppResult<Vec<NormalizedTx>> {
        self.as_block_source().fetch_block_txs(height).await
    }
}

#[async_trait]
impl GasEstimator for ChainHandle {
    async fn estimate_gas(&self, tx: &GasEstimateRequest) -> AppResult<GasEstimate> {
        match self {
            #[cfg(feature = "evm")]
            Self::Evm(c) => c.estimate_gas(tx).await,
            // TON: masterchain gas price × message-size heuristic; see
            // `TonChain::estimate_gas` for the BOC path.
            #[cfg(feature = "ton")]
            Self::Ton(c) => c.estimate_gas(tx).await,
            // Sui: dry-run the signed transaction block.
            #[cfg(feature = "sui")]
            Self::Sui(c) => c.estimate_gas(tx).await,
            // Zcash: zcashd fee estimate (ZEC/kB) converted to zat/vB.
            #[cfg(feature = "zcash")]
            Self::Zcash(c) => c.estimate_gas(tx).await,
            _ => Err(AppError::Unimplemented("estimate_gas".into())),
        }
    }
}

#[async_trait]
impl NonceProvider for ChainHandle {
    async fn nonce(&self, addr: &Address) -> AppResult<u64> {
        match self {
            #[cfg(feature = "evm")]
            Self::Evm(c) => c.nonce(addr).await,
            _ => Err(AppError::Unimplemented("nonce".into())),
        }
    }
}

#[derive(Default)]
pub struct ChainRegistry {
    chains: HashMap<ChainIndex, ChainHandle>,
}

impl ChainRegistry {
    pub fn insert(&mut self, index: ChainIndex, handle: ChainHandle) {
        self.chains.insert(index, handle);
    }

    pub fn get(&self, index: ChainIndex) -> AppResult<&ChainHandle> {
        self.chains
            .get(&index)
            .ok_or(AppError::ChainNotSupported(index))
    }
}
