use crate::AppState;
use uuid::Uuid;
use wallet_db::{AddressRow, UserRepo, UserRow};
use wallet_error::{AppError, AppResult};
use wallet_events::SUBJECT_USER_REGISTER;
use wallet_types::ChainIndex;

pub struct UserService<'a> {
    pub state: &'a AppState,
}

impl<'a> UserService<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    pub async fn get(&self, id: Uuid) -> AppResult<UserRow> {
        UserRepo::new(&self.state.db)
            .get(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("user {id}")))
    }

    pub async fn register(&self, external_id: &str) -> AppResult<UserRow> {
        let user = UserRepo::new(&self.state.db)
            .register(external_id)
            .await?;
        let _ = self
            .state
            .events
            .publish(
                SUBJECT_USER_REGISTER,
                serde_json::json!({ "user_id": user.id, "external_id": user.external_id }),
            )
            .await;
        Ok(user)
    }

    pub async fn list_addresses(
        &self,
        user_id: Uuid,
        chain_index: Option<ChainIndex>,
    ) -> AppResult<Vec<AddressRow>> {
        UserRepo::new(&self.state.db)
            .list_addresses(user_id, chain_index)
            .await
    }
}
