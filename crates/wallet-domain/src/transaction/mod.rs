use crate::AppState;
use wallet_chain::{GasEstimator, TxBroadcaster};
use wallet_db::{TransactionRepo, TxRow};
use wallet_error::{AppError, AppResult};
use wallet_types::{Address, Amount, ChainIndex, GasEstimate, GasEstimateRequest, TxHash};

pub struct TransactionService<'a> {
    pub state: &'a AppState,
}

impl<'a> TransactionService<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    pub async fn get(&self, chain_index: ChainIndex, hash: &str) -> AppResult<TxRow> {
        TransactionRepo::new(&self.state.db)
            .get(chain_index, hash)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("tx {hash}")))
    }

    pub async fn list(
        &self,
        chain_index: ChainIndex,
        address: &str,
        page: u32,
        page_size: u32,
    ) -> AppResult<(Vec<TxRow>, i64)> {
        let limit = page_size.max(1) as i64;
        let offset = ((page.max(1) - 1) * page_size) as i64;
        TransactionRepo::new(&self.state.db)
            .list_by_address(chain_index, address, limit, offset)
            .await
    }

    pub async fn broadcast(&self, chain_index: ChainIndex, raw: &[u8]) -> AppResult<TxHash> {
        self.state.chains.get(chain_index)?.send_raw(raw).await
    }

    pub async fn estimate_gas(
        &self,
        chain_index: ChainIndex,
        from: Address,
        to: Option<Address>,
        data: Option<Vec<u8>>,
        value: Option<Amount>,
    ) -> AppResult<GasEstimate> {
        let req = GasEstimateRequest {
            from,
            to,
            data,
            value,
        };
        self.state.chains.get(chain_index)?.estimate_gas(&req).await
    }
}
