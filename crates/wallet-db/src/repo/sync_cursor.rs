use wallet_error::{AppError, AppResult};
use wallet_types::ChainIndex;

use crate::models::SyncCursor;
use crate::Db;

pub struct SyncCursorRepo<'a> {
    db: &'a Db,
}

impl<'a> SyncCursorRepo<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub async fn get(&self, chain_index: ChainIndex) -> AppResult<u64> {
        let mut db = self.db.clone_inner();
        let rows: Vec<SyncCursor> =
            SyncCursor::filter(SyncCursor::fields().chain_index().eq(chain_index.as_i64()))
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;
        Ok(rows
            .into_iter()
            .next()
            .map(|r| r.height as u64)
            .unwrap_or(0))
    }

    pub async fn set(&self, chain_index: ChainIndex, height: u64) -> AppResult<()> {
        let mut db = self.db.clone_inner();
        let rows: Vec<SyncCursor> =
            SyncCursor::filter(SyncCursor::fields().chain_index().eq(chain_index.as_i64()))
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;

        match rows.into_iter().next() {
            Some(mut existing) => {
                existing
                    .update()
                    .height(height as i64)
                    .exec(&mut db)
                    .await
                    .map_err(|e| AppError::internal(e.to_string()))?;
            }
            None => {
                toasty::create!(SyncCursor {
                    chain_index: chain_index.as_i64(),
                    height: height as i64,
                })
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;
            }
        }
        Ok(())
    }
}
