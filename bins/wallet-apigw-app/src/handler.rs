use salvo::oapi::extract::{JsonBody, PathParam, QueryParam};
use salvo::prelude::*;
use std::sync::Arc;
use wallet_error::AppError;
use wallet_proto::wallet::v1::{
    BroadcastRequest, EstimateEnergyRequest, EstimateGasRequest, GetAppConfigRequest,
    GetBalancesRequest, GetKlinesRequest, GetPoolRequest, GetPriceRequest, ListAddressesRequest,
    ListDappsRequest, ListGuidesRequest, ListNetworksRequest, ListTokensRequest,
    ListTransactionsRequest, Pagination, QuoteRequest, RegisterRequest, SponsorRequest,
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

/// Get service metrics (for monitoring)
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
