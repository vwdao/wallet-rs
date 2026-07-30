use wallet_error::{AppError, AppResult};

use crate::models::Dapp;
use crate::Db;

pub struct DappRepo<'a> {
    db: &'a Db,
}

impl<'a> DappRepo<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub async fn get(&self, id: uuid::Uuid) -> AppResult<Option<Dapp>> {
        let mut db = self.db.clone_inner();
        match Dapp::get_by_id(&mut db, &id).await {
            Ok(dapp) => Ok(Some(dapp)),
            Err(_) => Ok(None),
        }
    }

    pub async fn list(&self, limit: i64, offset: i64) -> AppResult<(Vec<Dapp>, i64)> {
        let mut db = self.db.clone_inner();
        let rows: Vec<Dapp> = Dapp::all()
            .limit(limit as usize)
            .offset(offset as usize)
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;
        let total = Dapp::all()
            .count()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;
        Ok((rows, total as i64))
    }

    pub async fn upsert(&self, row: &Dapp) -> AppResult<Dapp> {
        let mut db = self.db.clone_inner();
        match Dapp::get_by_id(&mut db, &row.id).await {
            Ok(mut existing) => {
                existing
                    .update()
                    .name(&row.name)
                    .url(&row.url)
                    .logo_url(row.logo_url.clone())
                    .chain_indexes(row.chain_indexes.clone())
                    .exec(&mut db)
                    .await
                    .map_err(|e| AppError::internal(e.to_string()))?;
                Dapp::get_by_id(&mut db, &row.id)
                    .await
                    .map_err(|e| AppError::internal(e.to_string()))
            }
            Err(_) => toasty::create!(Dapp {
                id: row.id,
                name: &row.name,
                url: &row.url,
                logo_url: row.logo_url.clone(),
                chain_indexes: row.chain_indexes.clone(),
            })
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string())),
        }
    }
}
