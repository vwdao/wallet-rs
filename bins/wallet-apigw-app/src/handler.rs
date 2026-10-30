use salvo::oapi::extract::{JsonBody, PathParam, QueryParam};
use salvo::prelude::*;
use std::sync::Arc;
use wallet_error::AppError;
use wallet_proto::wallet::v1::{
    BroadcastRequest, CheckAppVersionRequest, CheckGeoRequest, CheckRecipientRequest,
    EstimateEnergyRequest, EstimateGasRequest, GetAppConfigRequest, GetBalancesRequest,
    GetKlinesRequest, GetPoolRequest, GetPriceRequest, GetTransactionDetailRequest,
    ListAddressesRequest, ListApprovalByUserAddressRequest, ListDappsRequest, ListGuidesRequest,
    ListNetworksRequest, ListTokenBalanceRequest, ListTokensRequest, ListTransactionsRequest,
    Pagination, QuoteRequest, RegisterRequest, SearchDappRequest,
    SponsorRequest, TransactionsByFilterRequest, WalletAddressWithChains,
};

use crate::types::*;
use crate::GwState;

type ApiResult<T> = Result<T, AppError>;

fn require_state(depot: &mut Depot) -> Result<Arc<GwState>, AppError> {
    depot
        .get_typed::<Arc<GwState>>()
        .cloned()
        .map_err(|_| AppError::internal("state not initialized"))
}

#[handler]
pub async fn healthz(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Text::Plain("ok"));
}

/// List supported blockchain networks
#[endpoint]
pub async fn list_networks(depot: &mut Depot) -> ApiResult<Json<ListNetworksResponse>> {
    let st = require_state(depot)?;
    let mut c = st.network.clone();
    let resp = c
        .list_networks(ListNetworksRequest { enabled_only: true })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(ListNetworksResponse {
        items: inner.items.into_iter().map(NetworkInfo::from).collect(),
    }))
}

/// List tokens on a chain
#[endpoint]
pub async fn list_tokens(
    depot: &mut Depot,
    chain_index: QueryParam<i64, false>,
    page: QueryParam<u32, false>,
    page_size: QueryParam<u32, false>,
) -> ApiResult<Json<ListTokensResponse>> {
    let st = require_state(depot)?;
    let mut c = st.token.clone();
    let resp = c
        .list_tokens(ListTokensRequest {
            chain_index: chain_index.into_inner().unwrap_or(0),
            pagination: Some(Pagination {
                page: page.into_inner().unwrap_or(1),
                page_size: page_size.into_inner().unwrap_or(20),
            }),
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(ListTokensResponse {
        items: inner.items.into_iter().map(TokenInfo::from).collect(),
        meta: inner.meta.map(PageMeta::from).unwrap_or(PageMeta {
            page: 1,
            page_size: 20,
            total: 0,
        }),
    }))
}

/// Get token balances for a wallet
#[endpoint]
pub async fn get_balances(
    depot: &mut Depot,
    chain_index: QueryParam<i64, false>,
    wallet: QueryParam<String, false>,
    tokens: QueryParam<String, false>,
) -> ApiResult<Json<GetBalancesResponse>> {
    let st = require_state(depot)?;
    let token_list = tokens
        .into_inner()
        .unwrap_or_default()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    let mut c = st.token.clone();
    let resp = c
        .get_balances(GetBalancesRequest {
            chain_index: chain_index.into_inner().unwrap_or(0),
            wallet: wallet.into_inner().unwrap_or_default(),
            tokens: token_list,
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(GetBalancesResponse {
        items: inner.items.into_iter().map(BalanceItem::from).collect(),
    }))
}

/// List transactions for an address
#[endpoint]
pub async fn list_txs(
    depot: &mut Depot,
    chain_index: QueryParam<i64, false>,
    address: QueryParam<String, false>,
    page: QueryParam<u32, false>,
    page_size: QueryParam<u32, false>,
) -> ApiResult<Json<ListTransactionsResponse>> {
    let st = require_state(depot)?;
    let mut c = st.tx.clone();
    let resp = c
        .list_transactions(ListTransactionsRequest {
            chain_index: chain_index.into_inner().unwrap_or(0),
            address: address.into_inner().unwrap_or_default(),
            pagination: Some(Pagination {
                page: page.into_inner().unwrap_or(1),
                page_size: page_size.into_inner().unwrap_or(20),
            }),
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(ListTransactionsResponse {
        items: inner.items.into_iter().map(TransactionInfo::from).collect(),
        meta: inner.meta.map(PageMeta::from).unwrap_or(PageMeta {
            page: 1,
            page_size: 20,
            total: 0,
        }),
    }))
}

/// Broadcast a signed transaction
#[endpoint]
pub async fn broadcast(
    depot: &mut Depot,
    body: JsonBody<BroadcastBody>,
) -> ApiResult<Json<BroadcastResponse>> {
    let st = require_state(depot)?;
    let body = body.into_inner();
    let raw = hex::decode(body.raw_tx_hex.trim_start_matches("0x"))
        .map_err(|e| AppError::InvalidArgument(e.to_string()))?;
    let mut c = st.tx.clone();
    let resp = c
        .broadcast(BroadcastRequest {
            chain_index: body.chain_index,
            raw_tx: raw,
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(BroadcastResponse { hash: inner.hash }))
}

/// Get a swap quote from DEX aggregators
#[endpoint]
pub async fn swap_quote(
    depot: &mut Depot,
    body: JsonBody<QuoteBody>,
) -> ApiResult<Json<QuoteResponse>> {
    let st = require_state(depot)?;
    let body = body.into_inner();
    let mut c = st.swap.clone();
    let resp = c
        .quote(QuoteRequest {
            chain_index: body.chain_index,
            from_token: body.from_token,
            to_token: body.to_token,
            amount: body.amount,
            provider: body.provider.unwrap_or_else(|| "1inch".into()),
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(QuoteResponse {
        provider: inner.provider,
        amount_out: inner.amount_out,
        route_json: inner.route_json,
    }))
}

/// Get token price
#[endpoint]
pub async fn price(
    depot: &mut Depot,
    symbol: QueryParam<String, false>,
) -> ApiResult<Json<PriceInfo>> {
    let st = require_state(depot)?;
    let mut c = st.market.clone();
    let resp = c
        .get_price(GetPriceRequest {
            symbol: symbol.into_inner().unwrap_or_else(|| "ETHUSDT".into()),
        })
        .await
        .map_err(AppError::from)?;
    Ok(Json(PriceInfo::from(resp.into_inner())))
}

/// Get kline/candlestick data
#[endpoint]
pub async fn klines(
    depot: &mut Depot,
    symbol: QueryParam<String, false>,
    interval: QueryParam<String, false>,
    page: QueryParam<u32, false>,
    page_size: QueryParam<u32, false>,
) -> ApiResult<Json<GetKlinesResponse>> {
    let st = require_state(depot)?;
    let mut c = st.market.clone();
    let resp = c
        .get_klines(GetKlinesRequest {
            symbol: symbol.into_inner().unwrap_or_else(|| "ETHUSDT".into()),
            interval: interval.into_inner().unwrap_or_else(|| "1m".into()),
            pagination: Some(Pagination {
                page: page.into_inner().unwrap_or(1),
                page_size: page_size.into_inner().unwrap_or(100),
            }),
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(GetKlinesResponse {
        items: inner.items.into_iter().map(KlineInfo::from).collect(),
    }))
}

/// List available dapps
#[endpoint]
pub async fn list_dapps(depot: &mut Depot) -> ApiResult<Json<ListDappsResponse>> {
    let st = require_state(depot)?;
    let mut c = st.dapp.clone();
    let resp = c
        .list_dapps(ListDappsRequest {
            pagination: Some(Pagination {
                page: 1,
                page_size: 50,
            }),
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(ListDappsResponse {
        items: inner.items.into_iter().map(DappInfo::from).collect(),
        meta: inner.meta.map(PageMeta::from).unwrap_or(PageMeta {
            page: 1,
            page_size: 50,
            total: 0,
        }),
    }))
}

/// Estimate energy rental cost on TRON
#[endpoint]
pub async fn rent_estimate(
    depot: &mut Depot,
    body: JsonBody<RentBody>,
) -> ApiResult<Json<EstimateEnergyResponse>> {
    let st = require_state(depot)?;
    let body = body.into_inner();
    let mut c = st.rent.clone();
    let resp = c
        .estimate_energy(EstimateEnergyRequest {
            address: body.address,
            energy: body.energy,
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(EstimateEnergyResponse {
        price: inner.price,
        duration_hours: inner.duration_hours,
    }))
}

/// Get app configuration (min version, features, etc.)
#[endpoint]
pub async fn app_config(
    depot: &mut Depot,
    platform: QueryParam<String, false>,
) -> ApiResult<Json<AppConfigInfo>> {
    let st = require_state(depot)?;
    let mut c = st.cms.clone();
    let app_version = std::env::var("APP_VERSION").unwrap_or_else(|_| "0.1.0".into());
    let resp = c
        .get_app_config(GetAppConfigRequest {
            platform: platform.into_inner().unwrap_or_else(|| "ios".into()),
            version: app_version,
        })
        .await
        .map_err(AppError::from)?;
    Ok(Json(AppConfigInfo::from(resp.into_inner())))
}

/// List help guides
#[endpoint]
pub async fn list_guides(
    depot: &mut Depot,
    locale: QueryParam<String, false>,
    page: QueryParam<u32, false>,
    page_size: QueryParam<u32, false>,
) -> ApiResult<Json<ListGuidesResponse>> {
    let st = require_state(depot)?;
    let mut c = st.cms.clone();
    let resp = c
        .list_guides(ListGuidesRequest {
            locale: locale.into_inner().unwrap_or_else(|| "en".into()),
            pagination: Some(Pagination {
                page: page.into_inner().unwrap_or(1),
                page_size: page_size.into_inner().unwrap_or(20),
            }),
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(ListGuidesResponse {
        items: inner.items.into_iter().map(GuideInfo::from).collect(),
        meta: inner.meta.map(PageMeta::from).unwrap_or(PageMeta {
            page: 1,
            page_size: 20,
            total: 0,
        }),
    }))
}

/// Get gas pool balance and status
#[endpoint]
pub async fn gas_pool(
    depot: &mut Depot,
    chain_index: PathParam<i64>,
) -> ApiResult<Json<GasPoolInfo>> {
    let st = require_state(depot)?;
    let mut c = st.gaspool.clone();
    let resp = c
        .get_pool(GetPoolRequest {
            chain_index: chain_index.into_inner(),
        })
        .await
        .map_err(AppError::from)?;
    Ok(Json(GasPoolInfo::from(resp.into_inner())))
}

/// Sponsor gas fee for a user operation
#[endpoint]
pub async fn gas_sponsor(
    depot: &mut Depot,
    body: JsonBody<SponsorBody>,
) -> ApiResult<Json<SponsorResponse>> {
    let st = require_state(depot)?;
    let body = body.into_inner();
    let raw = hex::decode(body.user_op_or_tx.trim_start_matches("0x"))
        .map_err(|e| AppError::InvalidArgument(e.to_string()))?;
    let mut c = st.gaspool.clone();
    let resp = c
        .sponsor(SponsorRequest {
            chain_index: body.chain_index,
            user_address: body.user_address,
            user_op_or_tx: raw,
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(SponsorResponse {
        status: inner.status,
        tx_hash: inner.tx_hash,
    }))
}

/// Estimate gas for a transaction
#[endpoint]
pub async fn estimate_gas(
    depot: &mut Depot,
    body: JsonBody<EstimateGasBody>,
) -> ApiResult<Json<EstimateGasResponse>> {
    let st = require_state(depot)?;
    let body = body.into_inner();
    let data = body
        .data
        .and_then(|d| hex::decode(d.trim_start_matches("0x")).ok())
        .unwrap_or_default();
    let mut c = st.tx.clone();
    let resp = c
        .estimate_gas(EstimateGasRequest {
            chain_index: body.chain_index,
            from_address: body.from_address,
            to_address: body.to_address.unwrap_or_default(),
            data,
            value: String::new(),
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(EstimateGasResponse {
        gas_limit: inner.gas_limit,
        max_fee_per_gas: inner.max_fee_per_gas,
        max_priority_fee_per_gas: inner.max_priority_fee_per_gas,
    }))
}

/// Register a new user
#[endpoint]
pub async fn register(
    depot: &mut Depot,
    body: JsonBody<RegisterBody>,
) -> ApiResult<Json<UserInfo>> {
    let st = require_state(depot)?;
    let body = body.into_inner();
    let mut c = st.user.clone();
    let resp = c
        .register(RegisterRequest {
            external_id: body.external_id,
        })
        .await
        .map_err(AppError::from)?;
    Ok(Json(UserInfo::from(resp.into_inner())))
}

/// List wallet addresses for a user
#[endpoint]
pub async fn list_addresses(
    depot: &mut Depot,
    user_id: QueryParam<String, false>,
    chain_index: QueryParam<i64, false>,
) -> ApiResult<Json<ListAddressesResponse>> {
    let st = require_state(depot)?;
    let user_id = user_id
        .into_inner()
        .ok_or_else(|| AppError::InvalidArgument("user_id is required".into()))?;
    let mut c = st.user.clone();
    let resp = c
        .list_addresses(ListAddressesRequest {
            user_id,
            chain_index: chain_index.into_inner().unwrap_or(0),
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(ListAddressesResponse {
        items: inner.items.into_iter().map(AddressInfo::from).collect(),
    }))
}

/// Get service metrics (for monitoring JSON)
#[endpoint]
pub async fn metrics(depot: &mut Depot) -> ApiResult<Json<MetricsResponse>> {
    let st = require_state(depot)?;
    let snapshot = st.metrics.snapshot();
    Ok(Json(MetricsResponse {
        requests_total: snapshot.requests_total,
        requests_success: snapshot.requests_success,
        requests_error: snapshot.requests_error,
        db_queries_total: snapshot.db_queries_total,
        db_queries_error: snapshot.db_queries_error,
        rpc_calls_total: snapshot.rpc_calls_total,
        rpc_calls_error: snapshot.rpc_calls_error,
        events_published: snapshot.events_published,
    }))
}

/// Prometheus metrics scrape endpoint (text/plain)
#[handler]
pub async fn metrics_prometheus(depot: &mut Depot, res: &mut Response) {
    let st = match require_state(depot) {
        Ok(s) => s,
        Err(_) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            return;
        }
    };
    let text = st.metrics.prometheus_text();
    res.headers_mut().insert(
        salvo::http::header::CONTENT_TYPE,
        "text/plain; version=0.0.4; charset=utf-8".parse().unwrap(),
    );
    res.render(Text::Plain(text));
}

/// Check recipient address safety and risk
#[endpoint]
pub async fn check_recipient(
    body: JsonBody<CheckRecipientBody>,
    depot: &mut Depot,
) -> ApiResult<Json<CheckRecipientResponse>> {
    let st = require_state(depot)?;
    let b = body.into_inner();
    let mut c = st.token.clone();
    let resp = c
        .check_recipient(CheckRecipientRequest {
            chain_index: b.chain_index,
            address: b.address,
            sender_address: b.sender_address,
            current_token_contract: b.current_token_contract.unwrap_or_default(),
            reference_addresses: b.reference_addresses.unwrap_or_default(),
            trusted_recipient: b.trusted_recipient.unwrap_or(false),
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(CheckRecipientResponse {
        action: inner.action,
        reason: inner.reason,
        complete: inner.complete,
        rules_version: inner.rules_version,
        valid_until_ms: inner.valid_until_ms,
        items: inner
            .items
            .into_iter()
            .map(|i| RecipientCheckItemDto {
                kind: i.kind,
                outcome: i.outcome,
                action: i.action,
                source: i.source,
                checked_at_ms: i.checked_at_ms,
                expires_at_ms: i.expires_at_ms,
            })
            .collect(),
        similarity_prefix: inner.similarity_prefix,
        similarity_suffix: inner.similarity_suffix,
        black_hole_addresses: inner.black_hole_addresses,
    }))
}

/// Batch fetch multi-wallet, multi-chain balances
#[endpoint]
pub async fn list_token_balances_batch(
    body: JsonBody<BatchBalancesBody>,
    depot: &mut Depot,
) -> ApiResult<Json<BatchBalancesResponse>> {
    let st = require_state(depot)?;
    let b = body.into_inner();
    let mut c = st.token.clone();
    let req = ListTokenBalanceRequest {
        addresses: b
            .addresses
            .into_iter()
            .map(|a| WalletAddressWithChains {
                address: a.address,
                chain_indexes: a.chain_indexes,
            })
            .collect(),
    };
    let resp = c.list_token_balance(req).await.map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(BatchBalancesResponse {
        list: inner
            .list
            .into_iter()
            .map(|w| WalletBalanceEntryDto {
                address: w.address,
                balances: w
                    .balances
                    .into_iter()
                    .map(|tb| TokenBalanceDetailDto {
                        chain_index: tb.chain_index,
                        address: tb.address,
                        name: tb.name,
                        symbol: tb.symbol,
                        decimals: tb.decimals,
                        raw_balance: tb.raw_balance,
                        price: tb.price,
                        logo_url: tb.logo_url,
                        is_official: tb.is_official,
                    })
                    .collect(),
            })
            .collect(),
    }))
}

/// Query transactions with advanced multi-address and status filters
#[endpoint]
pub async fn transactions_by_filter(
    addresses: QueryParam<String, false>,
    chain_index: QueryParam<i64, false>,
    page: QueryParam<i64, false>,
    limit: QueryParam<i64, false>,
    depot: &mut Depot,
) -> ApiResult<Json<TransactionsFilterResponse>> {
    let st = require_state(depot)?;
    let mut c = st.tx.clone();
    let resp = c
        .transactions_by_filter(TransactionsByFilterRequest {
            chain_index: chain_index.into_inner().unwrap_or(0),
            addresses: addresses.into_inner().unwrap_or_default(),
            status: None,
            token_address: None,
            keyword: None,
            transfer_type: None,
            page: page.into_inner().unwrap_or(1),
            limit: limit.into_inner().unwrap_or(20),
            start_time: 0,
            end_time: 0,
            include_spam: None,
            is_spam: None,
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(TransactionsFilterResponse {
        count: inner.count,
        list: inner.list.into_iter().map(TransactionInfo::from).collect(),
    }))
}

/// Cross-chain transaction summary
#[endpoint]
pub async fn transactions_from_summary(
    addresses: QueryParam<String, false>,
    page: QueryParam<i64, false>,
    limit: QueryParam<i64, false>,
    depot: &mut Depot,
) -> ApiResult<Json<TransactionsSummaryResponse>> {
    let st = require_state(depot)?;
    let mut c = st.tx.clone();
    let resp = c
        .transactions_from_summary(TransactionsByFilterRequest {
            chain_index: 0,
            addresses: addresses.into_inner().unwrap_or_default(),
            status: None,
            token_address: None,
            keyword: None,
            transfer_type: None,
            page: page.into_inner().unwrap_or(1),
            limit: limit.into_inner().unwrap_or(20),
            start_time: 0,
            end_time: 0,
            include_spam: None,
            is_spam: None,
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(TransactionsSummaryResponse {
        count: inner.count,
        list: inner
            .list
            .into_iter()
            .map(|s| TransactionSummaryDto {
                txn: s.txn,
                chain_index: s.chain_index,
                chain_name: s.chain_name,
                from_address: s.from_address,
                to_address: s.to_address,
                value: s.value,
                fee: s.fee,
                tx_type: s.tx_type,
                token_address: s.token_address,
                timestamp: s.timestamp,
                status: s.status,
                block_number: s.block_number,
                token_decimal: s.token_decimal,
                token_symbol: s.token_symbol,
                token_logo_url: s.token_logo_url,
                token_name: s.token_name,
                risk_level: s.risk_level,
                is_spam: s.is_spam,
                price: s.price,
                native_price: s.native_price,
            })
            .collect(),
    }))
}

/// Get transaction detail
#[endpoint]
pub async fn get_transaction_detail(
    chain_index: PathParam<i64>,
    txn: PathParam<String>,
    depot: &mut Depot,
) -> ApiResult<Json<TransactionDetailResponse>> {
    let st = require_state(depot)?;
    let mut c = st.tx.clone();
    let resp = c
        .get_transaction_detail(GetTransactionDetailRequest {
            chain_index: chain_index.into_inner(),
            txn: txn.into_inner(),
            from_address: None,
            to_address: None,
            token_address: None,
            verify_from_chain: false,
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(TransactionDetailResponse {
        txn: inner.txn,
        from_address: inner.from_address,
        to_address: inner.to_address,
        value: inner.value,
        fee: inner.fee,
        tx_type: inner.tx_type,
        token_address: inner.token_address,
        timestamp: inner.timestamp,
        status: inner.status,
        block_number: inner.block_number,
        chain_index: inner.chain_index,
        use_gaspool: inner.use_gaspool,
        price: inner.price,
        native_price: inner.native_price,
    }))
}

/// Check app version and upgrade requirement
#[endpoint]
pub async fn check_app_version(
    platform: QueryParam<String, false>,
    version: QueryParam<String, false>,
    locale: QueryParam<String, false>,
    depot: &mut Depot,
) -> ApiResult<Json<VersionCheckResponse>> {
    let st = require_state(depot)?;
    let platform = platform
        .into_inner()
        .ok_or_else(|| AppError::InvalidArgument("platform is required".into()))?;
    let version = version
        .into_inner()
        .ok_or_else(|| AppError::InvalidArgument("version is required".into()))?;
    let mut c = st.cms.clone();
    let resp = c
        .check_app_version(CheckAppVersionRequest {
            platform,
            version,
            locale: locale.into_inner().unwrap_or_else(|| "en".to_string()),
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(VersionCheckResponse {
        need_update: inner.need_update,
        force_update: inner.force_update,
        latest_version: inner.latest_version,
        min_version: inner.min_version,
        download_url: inner.download_url,
        title: inner.title,
        description: inner.description,
    }))
}

/// Check geographic region access
#[endpoint]
pub async fn check_geo(
    ip: QueryParam<String, false>,
    depot: &mut Depot,
) -> ApiResult<Json<GeoCheckResponse>> {
    let st = require_state(depot)?;
    let mut c = st.cms.clone();
    let resp = c
        .check_geo(CheckGeoRequest {
            ip: ip.into_inner().unwrap_or_else(|| "127.0.0.1".to_string()),
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(GeoCheckResponse {
        is_allowed: inner.is_allowed,
        country_code: inner.country_code,
        country_name: inner.country_name,
        reason: inner.reason,
    }))
}

/// List DApp token approvals for an address
#[endpoint]
pub async fn list_dapp_approvals(
    chain_index: QueryParam<i64, false>,
    address: QueryParam<String, false>,
    page: QueryParam<u32, false>,
    page_size: QueryParam<u32, false>,
    depot: &mut Depot,
) -> ApiResult<Json<DappApprovalsResponse>> {
    let st = require_state(depot)?;
    let address = address
        .into_inner()
        .ok_or_else(|| AppError::InvalidArgument("address is required".into()))?;
    let page = page.into_inner().unwrap_or(1);
    let page_size = page_size.into_inner().unwrap_or(20);
    let mut c = st.dapp.clone();
    let resp = c
        .list_approval_by_user_address(ListApprovalByUserAddressRequest {
            chain_index: chain_index.into_inner().unwrap_or(0),
            address,
            pagination: Some(Pagination { page, page_size }),
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(DappApprovalsResponse {
        items: inner
            .items
            .into_iter()
            .map(|a| TokenApprovalItemDto {
                chain_index: a.chain_index,
                dapp_name: a.dapp_name,
                dapp_url: a.dapp_url,
                spender_address: a.spender_address,
                token_address: a.token_address,
                token_symbol: a.token_symbol,
                allowance: a.allowance,
                is_unlimited: a.is_unlimited,
                updated_at: a.updated_at,
            })
            .collect(),
        meta: PageMeta {
            page,
            page_size,
            total: 0,
        },
    }))
}

/// Search DApps
#[endpoint]
pub async fn search_dapps(
    query: QueryParam<String, false>,
    chain_index: QueryParam<i64, false>,
    page: QueryParam<u32, false>,
    page_size: QueryParam<u32, false>,
    depot: &mut Depot,
) -> ApiResult<Json<ListDappsResponse>> {
    let st = require_state(depot)?;
    let query = query
        .into_inner()
        .ok_or_else(|| AppError::InvalidArgument("query is required".into()))?;
    let page = page.into_inner().unwrap_or(1);
    let page_size = page_size.into_inner().unwrap_or(20);
    let mut c = st.dapp.clone();
    let resp = c
        .search_dapp(SearchDappRequest {
            query,
            chain_index: chain_index.into_inner(),
            pagination: Some(Pagination { page, page_size }),
        })
        .await
        .map_err(AppError::from)?;
    let inner = resp.into_inner();
    Ok(Json(ListDappsResponse {
        items: inner.items.into_iter().map(DappInfo::from).collect(),
        meta: PageMeta {
            page,
            page_size,
            total: inner.meta.map(|m| m.total).unwrap_or(0),
        },
    }))
}
