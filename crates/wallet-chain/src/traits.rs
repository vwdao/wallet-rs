use async_trait::async_trait;
use wallet_error::AppResult;
use wallet_types::{Address, Amount, GasEstimate, GasEstimateRequest, NormalizedTx, TxHash};

#[async_trait]
pub trait BalanceReader: Send + Sync {
    async fn native_balance(&self, addr: &Address) -> AppResult<Amount>;
}

#[async_trait]
pub trait TokenBalance: Send + Sync {
    async fn token_balance(&self, wallet: &Address, token: &Address) -> AppResult<Amount>;

    /// Batch query token balances for a wallet in a single or batched RPC call where supported.
    async fn token_balances_batch(
        &self,
        wallet: &Address,
        tokens: &[Address],
    ) -> AppResult<Vec<(Address, Amount)>> {
        let mut out = Vec::with_capacity(tokens.len());
        for t in tokens {
            let bal = self.token_balance(wallet, t).await?;
            out.push((t.clone(), bal));
        }
        Ok(out)
    }
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
