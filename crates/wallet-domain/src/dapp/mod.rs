use crate::AppState;
use uuid::Uuid;
use wallet_db::{DappRepo, DappRow};
use wallet_error::{AppError, AppResult};

pub struct DappService<'a> {
    pub state: &'a AppState,
}

impl<'a> DappService<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    pub async fn get(&self, id: Uuid) -> AppResult<DappRow> {
        DappRepo::new(&self.state.db)
            .get(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("dapp {id}")))
    }

    pub async fn list(&self, page: u32, page_size: u32) -> AppResult<(Vec<DappRow>, i64)> {
        let limit = page_size.max(1) as i64;
        let offset = ((page.max(1) - 1) * page_size) as i64;
        DappRepo::new(&self.state.db).list(limit, offset).await
    }

    pub async fn upsert(&self, row: DappRow) -> AppResult<DappRow> {
        DappRepo::new(&self.state.db).upsert(&row).await
    }
}
