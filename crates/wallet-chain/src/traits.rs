use async_trait::async_trait;
use wallet_error::AppResult;
use wallet_types::{
    Address, Amount, GasEstimate, GasEstimateRequest, NormalizedTx, TxHash,
};

#[async_trait]
pub trait BalanceReader: Send + Sync {
    async fn native_balance(&self, addr: &Address) -> AppResult<Amount>;
}

#[async_trait]
pub trait TokenBalance: Send + Sync {
    async fn token_balance(&self, wallet: &Address, token: &Address) -> AppResult<Amount>;
}

#[async_trait]
pub trait TxBroadcaster: Send + Sync {
    async fn send_raw(&self, raw: &[u8]) -> AppResult<TxHash>;
}

#[async_trait]
pub trait BlockSource: Send + Sync {
    async fn tip(&self) -> AppResult<u64>;
    async fn fetch_block_txs(&self, height: u64) -> AppResult<Vec<NormalizedTx>>;
}

#[async_trait]
pub trait GasEstimator: Send + Sync {
    async fn estimate_gas(&self, tx: &GasEstimateRequest) -> AppResult<GasEstimate>;
}

#[async_trait]
pub trait NonceProvider: Send + Sync {
    async fn nonce(&self, addr: &Address) -> AppResult<u64>;
}
