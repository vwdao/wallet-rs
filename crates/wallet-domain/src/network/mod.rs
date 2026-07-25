use crate::AppState;
use wallet_chain::BlockSource;
use wallet_db::{NetworkRepo, NetworkRow};
use wallet_error::{AppError, AppResult};
use wallet_types::ChainIndex;

pub struct NetworkService<'a> {
    pub state: &'a AppState,
}

impl<'a> NetworkService<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    pub async fn list(&self, enabled_only: bool) -> AppResult<Vec<NetworkRow>> {
        NetworkRepo::new(&self.state.db).list(enabled_only).await
    }

    pub async fn get(&self, chain_index: ChainIndex) -> AppResult<NetworkRow> {
        NetworkRepo::new(&self.state.db)
            .get(chain_index)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("network {chain_index}")))
    }

    pub async fn tip(&self, chain_index: ChainIndex) -> AppResult<u64> {
        self.state.chains.get(chain_index)?.tip().await
    }

    pub async fn upsert(&self, row: NetworkRow) -> AppResult<NetworkRow> {
        NetworkRepo::new(&self.state.db).upsert(&row).await
    }
}
