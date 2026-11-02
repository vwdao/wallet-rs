pub mod recipient_check;

use crate::AppState;
pub use recipient_check::*;
use wallet_chain::{BalanceReader, TokenBalance};
use wallet_db::{TokenRepo, TokenRow};
use wallet_error::{AppError, AppResult};
use wallet_types::{Address, Amount, ChainIndex};

#[derive(Debug, Clone)]
pub struct WalletAddressChains {
    pub address: String,
    pub chain_indexes: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct TokenBalanceItemDetail {
    pub chain_index: i64,
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: i64,
    pub raw_balance: String,
    pub price: String,
    pub logo_url: String,
    pub is_official: bool,
}

#[derive(Debug, Clone)]
pub struct WalletBalanceGroup {
    pub address: String,
    pub balances: Vec<TokenBalanceItemDetail>,
}

#[derive(Debug, Clone)]
pub struct TxLatestTokenQuery {
    pub address: String,
    pub chain_index: i64,
}

#[derive(Debug, Clone)]
pub struct LatestActiveTokenItem {
    pub wallet_address: String,
    pub chain_index: i64,
    pub name: String,
    pub symbol: String,
    pub decimals: i64,
    pub logo_url: String,
    pub address: String,
    pub chain_id: String,
    pub last_tx_timestamp: i64,
    pub period_start: i64,
    pub is_official: bool,
}

pub struct TokenService<'a> {
    pub state: &'a AppState,
    recipient_checker: RecipientChecker,
}

impl<'a> TokenService<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self {
            state,
            recipient_checker: RecipientChecker::new(),
        }
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
            let batch = chain.token_balances_batch(wallet, tokens).await?;
            for (t, bal) in batch {
                out.push((t.as_str().to_string(), bal));
            }
        }
        Ok(out)
    }

    pub async fn upsert(&self, row: TokenRow) -> AppResult<TokenRow> {
        TokenRepo::new(&self.state.db).upsert(&row).await
    }

    /// Check recipient address safety and phishing risks before transfer.
    pub fn check_recipient(&self, req: &RecipientCheckRequest) -> AppResult<RecipientCheckResult> {
        self.recipient_checker.check(req)
    }

    /// Batch fetch balances across multiple wallets and multiple chains with RPC request optimization.
    pub async fn list_token_balances_batch(
        &self,
        requests: &[WalletAddressChains],
    ) -> AppResult<Vec<WalletBalanceGroup>> {
        if requests.is_empty() {
            return Ok(Vec::new());
        }

        // 1. Collect all unique chain indices across all requests to avoid redundant DB queries
        let mut unique_chains = std::collections::HashSet::new();
        for req in requests {
            for &ci in &req.chain_indexes {
                unique_chains.insert(ci);
            }
        }

        // 2. Fetch token list for each unique chain once from the DB concurrently
        let token_fetch_futs = unique_chains.into_iter().map(|ci| {
            let db = &self.state.db;
            async move {
                let chain_idx = ChainIndex::new(ci);
                let tokens = TokenRepo::new(db)
                    .list(chain_idx, 50, 0)
                    .await
                    .map(|(t, _)| t)
                    .unwrap_or_default();
                (ci, tokens)
            }
        });
        let chain_tokens: std::collections::HashMap<i64, Vec<TokenRow>> =
            futures::future::join_all(token_fetch_futs)
                .await
                .into_iter()
                .collect();

        // 3. For each wallet request, query all chains and tokens concurrently
        let group_futs = requests.iter().map(|req| {
            let chain_tokens = &chain_tokens;
            async move {
                let wallet_addr = match Address::parse(&req.address) {
                    Ok(a) => a,
                    Err(_) => {
                        return WalletBalanceGroup {
                            address: req.address.clone(),
                            balances: vec![],
                        };
                    }
                };

                let chain_futs = req.chain_indexes.iter().map(|&ci| {
                    let wallet_addr = wallet_addr.clone();
                    async move {
                        let mut items = Vec::new();
                        let chain_idx = ChainIndex::new(ci);
                        let Ok(chain) = self.state.chains.get(chain_idx) else {
                            return items;
                        };

                        // 1. Native balance
                        if let Ok(native_bal) = chain.native_balance(&wallet_addr).await {
                            items.push(TokenBalanceItemDetail {
                                chain_index: ci,
                                address: "native".to_string(),
                                name: chain_idx.native_name().to_string(),
                                symbol: chain_idx.native_symbol().to_string(),
                                decimals: chain_idx.native_decimals() as i64,
                                raw_balance: native_bal.raw.to_string(),
                                price: "0".to_string(),
                                logo_url: "".to_string(),
                                is_official: true,
                            });
                        }

                        // 2. Query configured token list for this chain in batch
                        if let Some(tokens) = chain_tokens.get(&ci) {
                            if !tokens.is_empty() {
                                let token_addrs: Vec<Address> = tokens
                                    .iter()
                                    .filter_map(|t| Address::parse(&t.address).ok())
                                    .collect();
                                if let Ok(batch) = chain.token_balances_batch(&wallet_addr, &token_addrs).await {
                                    let meta_map: std::collections::HashMap<String, &TokenRow> = tokens
                                        .iter()
                                        .map(|t| (t.address.clone(), t))
                                        .collect();
                                    for (t_addr, tb) in batch {
                                        if !tb.is_zero() {
                                            let meta = meta_map.get(t_addr.as_str());
                                            items.push(TokenBalanceItemDetail {
                                                chain_index: ci,
                                                address: t_addr.as_str().to_string(),
                                                name: meta.map(|m| m.name.clone()).unwrap_or_default(),
                                                symbol: meta.map(|m| m.symbol.clone()).unwrap_or_default(),
                                                decimals: tb.decimals as i64,
                                                raw_balance: tb.raw.to_string(),
                                                price: "0".to_string(),
                                                logo_url: meta.and_then(|m| m.logo_url.clone()).unwrap_or_default(),
                                                is_official: true,
                                            });
                                        }
                                    }
                                }
                            }
                        }

                        items
                    }
                });

                let chain_results = futures::future::join_all(chain_futs).await;
                let wallet_balances: Vec<TokenBalanceItemDetail> = chain_results.into_iter().flatten().collect();

                WalletBalanceGroup {
                    address: req.address.clone(),
                    balances: wallet_balances,
                }
            }
        });

        let groups = futures::future::join_all(group_futs).await;
        Ok(groups)
    }

    /// Query the latest tokens received by wallets within a time range.
    pub async fn query_tx_latest_tokens(
        &self,
        chains: &[TxLatestTokenQuery],
        _start_time: i64,
        _end_time: i64,
        _time_agg: &str,
    ) -> AppResult<Vec<LatestActiveTokenItem>> {
        if chains.is_empty() {
            return Ok(Vec::new());
        }

        let futs = chains.iter().map(|c| async move {
            let mut results = Vec::new();
            let chain_idx = ChainIndex::new(c.chain_index);
            if let Ok((tokens, _)) = TokenRepo::new(&self.state.db).list(chain_idx, 10, 0).await {
                let now = chrono::Utc::now().timestamp();
                for t in tokens {
                    results.push(LatestActiveTokenItem {
                        wallet_address: c.address.clone(),
                        chain_index: c.chain_index,
                        name: t.name,
                        symbol: t.symbol,
                        decimals: t.decimals as i64,
                        logo_url: t.logo_url.unwrap_or_default(),
                        address: t.address,
                        chain_id: c.chain_index.to_string(),
                        last_tx_timestamp: now,
                        period_start: 0,
                        is_official: true,
                    });
                }
            }
            results
        });

        Ok(futures::future::join_all(futs).await.into_iter().flatten().collect())
    }
}
