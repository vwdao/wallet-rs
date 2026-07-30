use wallet_error::{AppError, AppResult};
use wallet_types::ChainIndex;

use crate::models::{Address, User};
use crate::Db;

pub struct UserRepo<'a> {
    db: &'a Db,
}

impl<'a> UserRepo<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub async fn get(&self, id: uuid::Uuid) -> AppResult<Option<User>> {
        let mut db = self.db.clone_inner();
        match User::get_by_id(&mut db, &id).await {
            Ok(user) => Ok(Some(user)),
            Err(_) => Ok(None),
        }
    }

    pub async fn register(&self, external_id: &str) -> AppResult<User> {
        let mut db = self.db.clone_inner();
        if let Ok(existing) = User::filter_by_external_id(external_id).get(&mut db).await {
            return Ok(existing);
        }
        toasty::create!(User {
            external_id: external_id,
        })
        .exec(&mut db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))
    }

    pub async fn list_addresses(
        &self,
        user_id: uuid::Uuid,
        chain_index: Option<ChainIndex>,
    ) -> AppResult<Vec<Address>> {
        let mut db = self.db.clone_inner();
        let query = Address::filter(Address::fields().user_id().eq(user_id));
        let query = if let Some(ci) = chain_index {
            query.filter(Address::fields().chain_index().eq(ci.as_i64()))
        } else {
            query
        };
        query
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))
    }

    pub async fn list_users(&self, limit: i64, offset: i64) -> AppResult<(Vec<User>, i64)> {
        let mut db = self.db.clone_inner();
        let rows: Vec<User> = User::all()
            .limit(limit as usize)
            .offset(offset as usize)
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;
        let total = User::all()
            .count()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;
        Ok((rows, total as i64))
    }
}
