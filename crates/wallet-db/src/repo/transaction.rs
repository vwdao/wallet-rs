use wallet_error::{AppError, AppResult};
use wallet_types::{ChainIndex, NormalizedTx};

use crate::models::Tx;
use crate::Db;

pub struct TransactionRepo<'a> {
    db: &'a Db,
}

impl<'a> TransactionRepo<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub async fn get(&self, chain_index: ChainIndex, hash: &str) -> AppResult<Option<Tx>> {
        let mut db = self.db.clone_inner();
        let rows: Vec<Tx> = Tx::filter(
            Tx::fields()
                .chain_index()
                .eq(chain_index.as_i64())
                .and(Tx::fields().hash().eq(hash)),
        )
        .exec(&mut db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;
        Ok(rows.into_iter().next())
    }

    pub async fn list_by_address(
        &self,
        chain_index: ChainIndex,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> AppResult<(Vec<Tx>, i64)> {
        let mut db = self.db.clone_inner();
        let rows: Vec<Tx> = Tx::filter(
            Tx::fields().chain_index().eq(chain_index.as_i64()).and(
                Tx::fields()
                    .from_address()
                    .eq(address)
                    .or(Tx::fields().to_address().eq(address)),
            ),
        )
        .limit(limit as usize)
        .offset(offset as usize)
        .exec(&mut db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

        let total = Tx::filter(
            Tx::fields().chain_index().eq(chain_index.as_i64()).and(
                Tx::fields()
                    .from_address()
                    .eq(address)
                    .or(Tx::fields().to_address().eq(address)),
            ),
        )
        .count()
        .exec(&mut db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

        Ok((rows, total as i64))
    }

    pub async fn insert_normalized(
        &self,
        chain_index: ChainIndex,
        tx: &NormalizedTx,
    ) -> AppResult<()> {
        let mut db = self.db.clone_inner();
        let status = tx.status.as_str();

        let existing: Vec<Tx> = Tx::filter(
            Tx::fields()
                .chain_index()
                .eq(chain_index.as_i64())
                .and(Tx::fields().hash().eq(tx.hash.as_str())),
        )
        .exec(&mut db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

        if existing.is_empty() {
            toasty::create!(Tx {
                chain_index: chain_index.as_i64(),
                hash: tx.hash.as_str(),
                from_address: tx.from.as_ref().map(|a| a.as_str().to_string()),
                to_address: tx.to.as_ref().map(|a| a.as_str().to_string()),
                value: tx.value.raw,
                block_number: tx.block_number as i64,
                status: status,
                raw: tx.raw.clone(),
            })
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;
        } else {
            let mut existing = existing.into_iter().next().unwrap();
            existing
                .update()
                .from_address(tx.from.as_ref().map(|a| a.as_str().to_string()))
                .to_address(tx.to.as_ref().map(|a| a.as_str().to_string()))
                .value(tx.value.raw)
                .block_number(tx.block_number as i64)
                .status(status)
                .raw(tx.raw.clone())
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;
        }
        Ok(())
    }
}
