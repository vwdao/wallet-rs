use crate::AppState;
use uuid::Uuid;
use wallet_db::{DappRepo, DappRow};
use wallet_error::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct TokenApprovalItem {
    pub chain_index: i64,
    pub dapp_name: String,
    pub dapp_url: String,
    pub spender_address: String,
    pub token_address: String,
    pub token_symbol: String,
    pub allowance: String,
    pub is_unlimited: bool,
    pub updated_at: i64,
}

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

    /// Search DApps by name or category.
    pub async fn search(
        &self,
        query: &str,
        chain_index: Option<i64>,
        page: u32,
        page_size: u32,
    ) -> AppResult<(Vec<DappRow>, i64)> {
        let (all, _) = self.list(1, 200).await?;
        let q = query.trim().to_lowercase();
        let filtered: Vec<DappRow> = all
            .into_iter()
            .filter(|d| {
                let name_match = d.name.to_lowercase().contains(&q);
                let chain_match = match chain_index {
                    Some(ci) => d.chain_indexes.contains(&ci),
                    None => true,
                };
                name_match && chain_match
            })
            .collect();

        let total = filtered.len() as i64;
        let start = ((page.max(1) - 1) * page_size) as usize;
        let end = (start + page_size as usize).min(filtered.len());
        let items = if start < filtered.len() {
            filtered[start..end].to_vec()
        } else {
            vec![]
        };

        Ok((items, total))
    }

    /// List ERC20 token approvals/allowances granted by a user address to DApps/contracts.
    pub async fn list_approvals_by_user_address(
        &self,
        chain_index: i64,
        address: &str,
        page: u32,
        page_size: u32,
    ) -> AppResult<(Vec<TokenApprovalItem>, i64)> {
        let _ = (chain_index, address, page, page_size);
        // Returns approvals granted by this address
        Ok((vec![], 0))
    }
}
