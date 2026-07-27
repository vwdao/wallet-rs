use rust_decimal::Decimal;
use wallet_error::{AppError, AppResult};
use wallet_types::ChainIndex;

use crate::models::GasPool;
use crate::Db;

pub struct GasPoolRepo<'a> {
    db: &'a Db,
}

impl<'a> GasPoolRepo<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub async fn get(&self, chain_index: ChainIndex) -> AppResult<Option<GasPool>> {
        let mut db = self.db.clone_inner();
        let rows: Vec<GasPool> =
            GasPool::filter(GasPool::fields().chain_index().eq(chain_index.as_i64()))
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;
        Ok(rows.into_iter().next())
    }

    pub async fn set_enabled(&self, chain_index: ChainIndex, enabled: bool) -> AppResult<()> {
        let mut db = self.db.clone_inner();
        let rows: Vec<GasPool> =
            GasPool::filter(GasPool::fields().chain_index().eq(chain_index.as_i64()))
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;

        match rows.into_iter().next() {
            Some(mut existing) => {
                existing
                    .update()
                    .enabled(enabled)
                    .exec(&mut db)
                    .await
                    .map_err(|e| AppError::internal(e.to_string()))?;
            }
            None => {
                toasty::create!(GasPool {
                    chain_index: chain_index.as_i64(),
                    balance: Decimal::ZERO,
                    enabled: enabled,
                    hot_wallet: "",
                    cold_wallet: "",
                })
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;
            }
        }
        Ok(())
    }

    pub async fn update_balance(
        &self,
        chain_index: ChainIndex,
        balance: &Decimal,
    ) -> AppResult<()> {
        let mut db = self.db.clone_inner();
        let rows: Vec<GasPool> =
            GasPool::filter(GasPool::fields().chain_index().eq(chain_index.as_i64()))
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;

        if let Some(mut existing) = rows.into_iter().next() {
            existing
                .update()
                .balance(*balance)
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;
        }
        Ok(())
    }
}
