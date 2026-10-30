use crate::AppState;
use wallet_chain::{GasEstimator, TxBroadcaster};
use wallet_db::{TransactionFilter, TransactionRepo, TxRow};
use wallet_error::{AppError, AppResult};
use wallet_types::{Address, Amount, ChainIndex, GasEstimate, GasEstimateRequest, TxHash};

#[derive(Debug, Clone)]
pub struct TransactionFilterOptions {
    pub chain_index: i64,
    pub addresses: Vec<String>,
    pub status: Option<i64>,
    pub token_address: Option<String>,
    pub keyword: Option<String>,
    pub transfer_type: Option<String>,
    pub start_time: i64,
    pub end_time: i64,
    pub include_spam: Option<bool>,
    pub is_spam: Option<bool>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Clone)]
pub struct TransactionSummaryItem {
    pub txn: String,
    pub chain_index: i64,
    pub chain_name: String,
    pub from_address: String,
    pub to_address: String,
    pub value: String,
    pub fee: String,
    pub tx_type: String,
    pub token_address: String,
    pub timestamp: i64,
    pub status: String,
    pub block_number: i64,
    pub token_decimal: i64,
    pub token_symbol: String,
    pub token_logo_url: String,
    pub token_name: String,
    pub risk_level: String,
    pub is_spam: bool,
    pub price: String,
    pub native_price: String,
}

#[derive(Debug, Clone)]
pub struct GasPoolDetailSnapshot {
    pub gas: String,
    pub gas_usdt: String,
    pub native_tx_hash: String,
    pub native_tx_value: String,
    pub native_tx_value_usdt: String,
    pub native_tx_gas: String,
    pub native_tx_gas_usdt: String,
    pub status: String,
    pub price: String,
    pub activate_fee: String,
    pub activate_fee_usdt: String,
    pub energy: String,
    pub energy_trx: String,
    pub energy_usdt: String,
    pub bandwidth: String,
    pub bandwidth_trx: String,
    pub bandwidth_usdt: String,
    pub bandwidth_price: String,
}

#[derive(Debug, Clone)]
pub struct TransactionDetailResult {
    pub txn: String,
    pub from_address: String,
    pub to_address: String,
    pub value: String,
    pub fee: String,
    pub tx_type: String,
    pub token_address: String,
    pub timestamp: i64,
    pub status: String,
    pub block_number: i64,
    pub chain_index: i64,
    pub use_gaspool: bool,
    pub gaspool_detail: Option<GasPoolDetailSnapshot>,
    pub price: String,
    pub native_price: String,
}

pub struct TransactionService<'a> {
    pub state: &'a AppState,
}

impl<'a> TransactionService<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    pub async fn get(&self, chain_index: ChainIndex, hash: &str) -> AppResult<TxRow> {
        TransactionRepo::new(&self.state.db)
            .get(chain_index, hash)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("tx {hash}")))
    }

    pub async fn list(
        &self,
        chain_index: ChainIndex,
        address: &str,
        page: u32,
        page_size: u32,
    ) -> AppResult<(Vec<TxRow>, i64)> {
        let limit = page_size.max(1) as i64;
        let offset = ((page.max(1) - 1) * page_size) as i64;
        TransactionRepo::new(&self.state.db)
            .list_by_address(chain_index, address, limit, offset)
            .await
    }

    pub async fn list_by_filter(
        &self,
        opts: &TransactionFilterOptions,
    ) -> AppResult<(Vec<TxRow>, i64)> {
        let limit = opts.page_size.max(1) as i64;
        let offset = ((opts.page.max(1) - 1) * opts.page_size) as i64;
        let filter = TransactionFilter {
            chain_index: if opts.chain_index != 0 {
                Some(opts.chain_index)
            } else {
                None
            },
            addresses: opts.addresses.clone(),
            status: opts.status.map(|s| s.to_string()),
            token_address: opts.token_address.clone(),
            keyword: opts.keyword.clone(),
            transfer_type: opts.transfer_type.clone(),
            start_time: if opts.start_time > 0 {
                Some(opts.start_time)
            } else {
                None
            },
            end_time: if opts.end_time > 0 {
                Some(opts.end_time)
            } else {
                None
            },
            include_spam: opts.include_spam,
            is_spam: opts.is_spam,
            limit,
            offset,
        };

        TransactionRepo::new(&self.state.db).list_by_filter(&filter).await
    }

    pub async fn list_summary(
        &self,
        opts: &TransactionFilterOptions,
    ) -> AppResult<(Vec<TransactionSummaryItem>, i64)> {
        let (rows, total) = self.list_by_filter(opts).await?;
        let items = rows
            .into_iter()
            .map(|r| TransactionSummaryItem {
                txn: r.hash,
                chain_index: r.chain_index,
                chain_name: format!("Chain {}", r.chain_index),
                from_address: r.from_address.unwrap_or_default(),
                to_address: r.to_address.unwrap_or_default(),
                value: r.value.to_string(),
                fee: r.gas_fee.map(|g| g.to_string()).unwrap_or_default(),
                tx_type: r.method.unwrap_or_else(|| "transfer".to_string()),
                token_address: r.contract_address.unwrap_or_default(),
                timestamp: 0,
                status: r.status,
                block_number: r.block_number,
                token_decimal: 18,
                token_symbol: String::new(),
                token_logo_url: String::new(),
                token_name: String::new(),
                risk_level: "low".to_string(),
                is_spam: false,
                price: "0".to_string(),
                native_price: "0".to_string(),
            })
            .collect();
        Ok((items, total))
    }

    pub async fn get_detail(
        &self,
        chain_index: ChainIndex,
        hash: &str,
    ) -> AppResult<TransactionDetailResult> {
        let row = self.get(chain_index, hash).await?;
        Ok(TransactionDetailResult {
            txn: row.hash,
            from_address: row.from_address.unwrap_or_default(),
            to_address: row.to_address.unwrap_or_default(),
            value: row.value.to_string(),
            fee: row.gas_fee.map(|g| g.to_string()).unwrap_or_default(),
            tx_type: row.method.unwrap_or_else(|| "transfer".to_string()),
            token_address: row.contract_address.unwrap_or_default(),
            timestamp: 0,
            status: row.status,
            block_number: row.block_number,
            chain_index: row.chain_index,
            use_gaspool: false,
            gaspool_detail: None,
            price: "0".to_string(),
            native_price: "0".to_string(),
        })
    }

    pub async fn broadcast(&self, chain_index: ChainIndex, raw: &[u8]) -> AppResult<TxHash> {
        self.state.chains.get(chain_index)?.send_raw(raw).await
    }

    pub async fn estimate_gas(
        &self,
        chain_index: ChainIndex,
        from: Address,
        to: Option<Address>,
        data: Option<Vec<u8>>,
        value: Option<Amount>,
    ) -> AppResult<GasEstimate> {
        let req = GasEstimateRequest {
            from,
            to,
            data,
            value,
            extras: None,
        };
        self.state.chains.get(chain_index)?.estimate_gas(&req).await
    }
}
