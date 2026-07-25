use crate::AppState;
use wallet_chain::{BalanceReader, TokenBalance};
use wallet_db::{TokenRepo, TokenRow};
use wallet_error::{AppError, AppResult};
use wallet_types::{Address, Amount, ChainIndex};

pub struct TokenService<'a> {
    pub state: &'a AppState,
}

impl<'a> TokenService<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    pub async fn get(&self, chain_index: ChainIndex, address: &str) -> AppResult<TokenRow> {
        TokenRepo::new(&self.state.db)
            .get(chain_index, address)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("token {address}")))
    }

    pub async fn list(
        &self,
        chain_index: ChainIndex,
        page: u32,
        page_size: u32,
    ) -> AppResult<(Vec<TokenRow>, i64)> {
        let limit = page_size.max(1) as i64;
        let offset = ((page.max(1) - 1) * page_size) as i64;
        TokenRepo::new(&self.state.db)
            .list(chain_index, limit, offset)
            .await
    }

    pub async fn balances(
        &self,
        chain_index: ChainIndex,
        wallet: &Address,
        tokens: &[Address],
    ) -> AppResult<Vec<(String, Amount)>> {
        let chain = self.state.chains.get(chain_index)?;
        let mut out = Vec::new();
        if tokens.is_empty() {
            let native = chain.native_balance(wallet).await?;
            out.push(("native".into(), native));
        } else {
            for t in tokens {
                let bal = chain.token_balance(wallet, t).await?;
                out.push((t.as_str().to_string(), bal));
            }
        }
        Ok(out)
    }

    pub async fn upsert(&self, row: TokenRow) -> AppResult<TokenRow> {
        TokenRepo::new(&self.state.db).upsert(&row).await
    }
}
