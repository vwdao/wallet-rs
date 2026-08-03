use salvo::oapi::extract::{JsonBody, PathParam, QueryParam};
use salvo::prelude::*;
use std::sync::Arc;
use wallet_error::AppError;
use wallet_proto::wallet::v1::admin::{
    DeleteGuideRequest, ListEndpointsRequest, SetGasPoolEnabledRequest,
    SwapProvider as ProtoSwapProvider,
};
use wallet_proto::wallet::v1::{
    AppConfig, Guide, ListDappsRequest, ListGuidesRequest, ListNetworksRequest, ListTokensRequest,
    Pagination,
};

use crate::types::*;
use crate::AdminState;

type ApiResult<T> = Result<T, AppError>;

fn require_state(depot: &mut Depot) -> Result<Arc<AdminState>, AppError> {
    depot
        .get_typed::<Arc<AdminState>>()
        .cloned()
        .map_err(|_| AppError::internal("state not initialized"))
}

/// Health check
#[endpoint]
pub async fn healthz(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Text::Plain("ok"));
}

/// List all networks (including disabled)
#[endpoint]
pub async fn list_networks(depot: &mut Depot) -> ApiResult<Json<ListNetworksResponse>> {
    let st = require_state(depot)?;
    let mut c = st.networks.clone();
    let resp = c
        .list_networks(ListNetworksRequest {
            enabled_only: false,
        })
        .await?;
    let inner = resp.into_inner();
    Ok(Json(ListNetworksResponse {
        items: inner.items.into_iter().map(NetworkInfo::from).collect(),
    }))
}

/// Create or update a network
#[endpoint]
pub async fn upsert_network(
    depot: &mut Depot,
    body: JsonBody<NetworkBody>,
) -> ApiResult<Json<NetworkInfo>> {
    let st = require_state(depot)?;
    let body = body.into_inner();
    let proto = wallet_proto::wallet::v1::Network {
        chain_index: body.chain_index,
        name: body.name,
        family: body.family,
        evm_chain_id: body.evm_chain_id,
        enabled: body.enabled,
    };
    let mut c = st.networks.clone();
    let resp = c.upsert_network(proto).await?;
    Ok(Json(NetworkInfo::from(resp.into_inner())))
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
    let mut c = st.tokens.clone();
    let resp = c
        .list_tokens(ListTokensRequest {
            chain_index: chain_index.into_inner().unwrap_or(60),
            pagination: Some(Pagination {
                page: page.into_inner().unwrap_or(1),
                page_size: page_size.into_inner().unwrap_or(20),
            }),
        })
        .await?;
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

/// Create or update a token
#[endpoint]
pub async fn upsert_token(
    depot: &mut Depot,
    body: JsonBody<TokenBody>,
) -> ApiResult<Json<TokenInfo>> {
    let st = require_state(depot)?;
    let body = body.into_inner();
    let proto = wallet_proto::wallet::v1::Token {
        id: body.id,
        chain_index: body.chain_index,
        address: body.address,
        symbol: body.symbol,
        name: body.name,
        decimals: body.decimals,
        logo_url: body.logo_url,
    };
    let mut c = st.tokens.clone();
    let resp = c.upsert_token(proto).await?;
    Ok(Json(TokenInfo::from(resp.into_inner())))
}

/// List users
#[endpoint]
pub async fn list_users(
    depot: &mut Depot,
    page: QueryParam<u32, false>,
    page_size: QueryParam<u32, false>,
) -> ApiResult<Json<ListUsersResponse>> {
    let st = require_state(depot)?;
    let mut c = st.users.clone();
    let resp = c
        .list_users(wallet_proto::wallet::v1::admin::ListUsersRequest {
            pagination: Some(Pagination {
                page: page.into_inner().unwrap_or(1),
                page_size: page_size.into_inner().unwrap_or(20),
            }),
        })
        .await?;
    let inner = resp.into_inner();
    Ok(Json(ListUsersResponse {
        items: inner.items.into_iter().map(AdminUserInfo::from).collect(),
        meta: inner.meta.map(PageMeta::from).unwrap_or(PageMeta {
            page: 1,
            page_size: 20,
            total: 0,
        }),
    }))
}

/// List dapps
#[endpoint]
pub async fn list_dapps(
    depot: &mut Depot,
    page: QueryParam<u32, false>,
    page_size: QueryParam<u32, false>,
) -> ApiResult<Json<ListDappsResponse>> {
    let st = require_state(depot)?;
    let mut c = st.dapps.clone();
    let resp = c
        .list_dapps(ListDappsRequest {
            pagination: Some(Pagination {
                page: page.into_inner().unwrap_or(1),
                page_size: page_size.into_inner().unwrap_or(20),
            }),
        })
        .await?;
    let inner = resp.into_inner();
    Ok(Json(ListDappsResponse {
        items: inner.items.into_iter().map(DappInfo::from).collect(),
        meta: inner.meta.map(PageMeta::from).unwrap_or(PageMeta {
            page: 1,
            page_size: 20,
            total: 0,
        }),
    }))
}

/// Create or update a dapp
#[endpoint]
pub async fn upsert_dapp(depot: &mut Depot, body: JsonBody<DappBody>) -> ApiResult<Json<DappInfo>> {
    let st = require_state(depot)?;
    let body = body.into_inner();
    let proto = wallet_proto::wallet::v1::Dapp {
        id: body.id,
        name: body.name,
        url: body.url,
        logo_url: body.logo_url,
        chain_indexes: body.chain_indexes,
    };
    let mut c = st.dapps.clone();
    let resp = c.upsert_dapp(proto).await?;
    Ok(Json(DappInfo::from(resp.into_inner())))
}

/// List RPC endpoints
#[endpoint]
pub async fn list_rpc_endpoints(
    depot: &mut Depot,
    chain_index: QueryParam<i64, false>,
) -> ApiResult<Json<ListRpcEndpointsResponse>> {
    let st = require_state(depot)?;
    let mut c = st.rpc_endpoints.clone();
    let resp = c
        .list_endpoints(ListEndpointsRequest {
            chain_index: chain_index.into_inner().unwrap_or(60),
        })
        .await?;
    let inner = resp.into_inner();
    Ok(Json(ListRpcEndpointsResponse {
        items: inner.items.into_iter().map(RpcEndpointInfo::from).collect(),
    }))
}

/// Create or update an RPC endpoint
#[endpoint]
pub async fn upsert_rpc_endpoint(
    depot: &mut Depot,
    body: JsonBody<RpcEndpointBody>,
) -> ApiResult<Json<RpcEndpointInfo>> {
    let st = require_state(depot)?;
    let body = body.into_inner();
    let proto = wallet_proto::wallet::v1::admin::RpcEndpoint {
        id: body.id,
        chain_index: body.chain_index,
        url: body.url,
        weight: body.weight,
        enabled: body.enabled,
        headers: body.headers,
        protocol: body.protocol,
    };
    let mut c = st.rpc_endpoints.clone();
    let resp = c.upsert_endpoint(proto).await?;
    let inner = resp.into_inner();
    Ok(Json(RpcEndpointInfo {
        id: inner.id,
        chain_index: inner.chain_index,
        url: inner.url,
        weight: inner.weight,
        enabled: inner.enabled,
        headers: inner.headers,
        protocol: inner.protocol,
    }))
}

/// Delete an RPC endpoint
#[endpoint]
pub async fn delete_rpc_endpoint(
    depot: &mut Depot,
    id: PathParam<String>,
) -> ApiResult<Json<DeletedResponse>> {
    let st = require_state(depot)?;
    let endpoint_id = id.into_inner();
    let mut c = st.rpc_endpoints.clone();
    c.delete_endpoint(wallet_proto::wallet::v1::admin::DeleteEndpointRequest { id: endpoint_id })
        .await?;
    Ok(Json(DeletedResponse { deleted: true }))
}

/// Enable or disable a gas pool
#[endpoint]
pub async fn set_gas_pool_enabled(
    depot: &mut Depot,
    body: JsonBody<SetGasPoolEnabledBody>,
) -> ApiResult<Json<OkResponse>> {
    let st = require_state(depot)?;
    let body = body.into_inner();
    let mut c = st.gas_pools.clone();
    c.set_enabled(SetGasPoolEnabledRequest {
        chain_index: body.chain_index,
        enabled: body.enabled,
    })
    .await?;
    Ok(Json(OkResponse { ok: true }))
}

/// Create or update a swap provider
#[endpoint]
pub async fn upsert_swap_provider(
    depot: &mut Depot,
    body: JsonBody<SwapProviderBody>,
) -> ApiResult<Json<OkResponse>> {
    let st = require_state(depot)?;
    let body = body.into_inner();
    let proto = ProtoSwapProvider {
        name: body.name,
        enabled: body.enabled,
        config_json: body.config_json,
    };
    let mut c = st.swap.clone();
    c.upsert_provider(proto).await?;
    Ok(Json(OkResponse { ok: true }))
}

/// Reindex transactions in a block range
#[endpoint]
pub async fn reindex(
    depot: &mut Depot,
    body: JsonBody<ReindexBody>,
) -> ApiResult<Json<OkResponse>> {
    let st = require_state(depot)?;
    let body = body.into_inner();
    let mut c = st.transactions.clone();
    c.reindex(wallet_proto::wallet::v1::admin::ReindexRequest {
        chain_index: body.chain_index,
        from_block: body.from_block,
        to_block: body.to_block,
    })
    .await?;
    Ok(Json(OkResponse { ok: true }))
}

/// Get app config for a platform
#[endpoint]
pub async fn get_app_config(
    depot: &mut Depot,
    platform: QueryParam<String, false>,
) -> ApiResult<Json<AppConfigInfo>> {
    let st = require_state(depot)?;
    let mut c = st.cms.clone();
    let resp = c
        .get_app_config(wallet_proto::wallet::v1::GetAppConfigRequest {
            platform: platform.into_inner().unwrap_or_else(|| "ios".into()),
            version: String::new(),
        })
        .await?;
    Ok(Json(AppConfigInfo::from(resp.into_inner())))
}

/// Create or update an app config
#[endpoint]
pub async fn upsert_app_config(
    depot: &mut Depot,
    body: JsonBody<AppConfigBody>,
) -> ApiResult<Json<AppConfigInfo>> {
    let st = require_state(depot)?;
    let body = body.into_inner();
    let proto = AppConfig {
        platform: body.platform,
        min_version: body.min_version,
        latest_version: body.latest_version,
        force_update_url: body.force_update_url,
        features_json: body.features_json,
    };
    let mut c = st.cms.clone();
    let resp = c.upsert_app_config(proto).await?;
    Ok(Json(AppConfigInfo::from(resp.into_inner())))
}

/// List guides by locale
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
        .await?;
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

/// Create or update a guide
#[endpoint]
pub async fn upsert_guide(
    depot: &mut Depot,
    body: JsonBody<GuideBody>,
) -> ApiResult<Json<GuideInfo>> {
    let st = require_state(depot)?;
    let body = body.into_inner();
    let proto = Guide {
        id: body.id,
        title: body.title,
        body: body.body,
        locale: body.locale,
    };
    let mut c = st.cms.clone();
    let resp = c.upsert_guide(proto).await?;
    Ok(Json(GuideInfo::from(resp.into_inner())))
}

/// Delete a guide
#[endpoint]
pub async fn delete_guide(
    depot: &mut Depot,
    id: PathParam<String>,
) -> ApiResult<Json<DeletedResponse>> {
    let st = require_state(depot)?;
    let guide_id = id.into_inner();
    let mut c = st.cms.clone();
    c.delete_guide(DeleteGuideRequest { id: guide_id }).await?;
    Ok(Json(DeletedResponse { deleted: true }))
}
