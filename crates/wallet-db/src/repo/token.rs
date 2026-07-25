use wallet_error::{AppError, AppResult};
use wallet_types::ChainIndex;

use crate::models::Token;
use crate::Db;

pub struct TokenRepo<'a> {
    db: &'a Db,
}

impl<'a> TokenRepo<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub async fn get(&self, chain_index: ChainIndex, address: &str) -> AppResult<Option<Token>> {
        let mut db = self.db.clone_inner();
        let rows: Vec<Token> = Token::filter(
            Token::fields()
                .chain_index()
                .eq(chain_index.as_i64())
                .and(Token::fields().address().eq(address)),
        )
        .exec(&mut db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;
        Ok(rows.into_iter().next())
    }

    pub async fn list(
        &self,
        chain_index: ChainIndex,
        limit: i64,
        offset: i64,
    ) -> AppResult<(Vec<Token>, i64)> {
        let mut db = self.db.clone_inner();
        let rows: Vec<Token> = Token::filter(Token::fields().chain_index().eq(chain_index.as_i64()))
            .limit(limit as usize)
            .offset(offset as usize)
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;
        let total = Token::filter(Token::fields().chain_index().eq(chain_index.as_i64()))
            .count()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;
        Ok((rows, total as i64))
    }

    pub async fn upsert(&self, row: &Token) -> AppResult<Token> {
        let mut db = self.db.clone_inner();
        let existing: Vec<Token> = Token::filter(
            Token::fields()
                .chain_index()
                .eq(row.chain_index)
                .and(Token::fields().address().eq(&row.address)),
        )
        .exec(&mut db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

        if let Some(mut existing) = existing.into_iter().next() {
            existing
                .update()
                .symbol(&row.symbol)
                .name(&row.name)
                .decimals(row.decimals)
                .logo_url(row.logo_url.clone())
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;
            Token::get_by_id(&mut db, &existing.id)
                .await
                .map_err(|e| AppError::internal(e.to_string()))
        } else {
            toasty::create!(Token {
                id: row.id,
                chain_index: row.chain_index,
                address: &row.address,
                symbol: &row.symbol,
                name: &row.name,
                decimals: row.decimals,
                logo_url: row.logo_url.clone(),
            })
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))
        }
    }
}
