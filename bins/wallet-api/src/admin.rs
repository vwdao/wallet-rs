use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use wallet_db::{DappRow, NetworkRepo, NetworkRow, TokenRow, UserRepo};
use wallet_domain::AppState;
use wallet_proto::wallet::v1::admin::admin_cms_service_server::AdminCmsService;
use wallet_proto::wallet::v1::admin::admin_dapp_service_server::AdminDappService;
use wallet_proto::wallet::v1::admin::admin_gas_pool_service_server::AdminGasPoolService;
use wallet_proto::wallet::v1::admin::admin_network_service_server::AdminNetworkService;
use wallet_proto::wallet::v1::admin::admin_rpc_endpoint_service_server::AdminRpcEndpointService;
use wallet_proto::wallet::v1::admin::admin_swap_service_server::AdminSwapService;
use wallet_proto::wallet::v1::admin::admin_token_service_server::AdminTokenService;
use wallet_proto::wallet::v1::admin::admin_transaction_service_server::AdminTransactionService;
use wallet_proto::wallet::v1::admin::admin_user_service_server::AdminUserService;
use wallet_proto::wallet::v1::admin::*;
use wallet_proto::wallet::v1::{
    AppConfig, Dapp, Empty, GetAppConfigRequest, Guide, ListDappsRequest, ListDappsResponse,
    ListGuidesRequest, ListGuidesResponse, ListNetworksRequest, ListNetworksResponse,
    ListTokensRequest, ListTokensResponse, Network, PageMeta, Token,
};
use wallet_types::ChainIndex;

use crate::service::{dapp_from_row, network_from_row, token_from_row};

pub struct AdminUserSvc(pub Arc<AppState>);
pub struct AdminNetworkSvc(pub Arc<AppState>);
pub struct AdminTokenSvc(pub Arc<AppState>);
pub struct AdminDappSvc(pub Arc<AppState>);
pub struct AdminRpcSvc(pub Arc<AppState>);
pub struct AdminGasSvc(pub Arc<AppState>);
pub struct AdminSwapSvc(pub Arc<AppState>);
pub struct AdminTxSvc(pub Arc<AppState>);
pub struct AdminCmsSvc(pub Arc<AppState>);

#[tonic::async_trait]
impl AdminUserService for AdminUserSvc {
    async fn list_users(
        &self,
        req: Request<ListUsersRequest>,
    ) -> Result<Response<ListUsersResponse>, Status> {
        let p = req
            .into_inner()
            .pagination
            .unwrap_or(wallet_proto::wallet::v1::Pagination {
                page: 1,
                page_size: 20,
            });
        let page = p.page.max(1);
        let page_size = p.page_size.clamp(1, 100);
        let (rows, total) = UserRepo::new(&self.0.db)
            .list_users(page_size as i64, ((page - 1) * page_size) as i64)
            .await?;
        Ok(Response::new(ListUsersResponse {
            items: rows
                .into_iter()
                .map(|u| AdminUser {
                    id: u.id.to_string(),
                    external_id: u.external_id,
                })
                .collect(),
            meta: Some(PageMeta {
                page,
                page_size,
                total: total as u64,
            }),
        }))
    }
}

#[tonic::async_trait]
impl AdminNetworkService for AdminNetworkSvc {
    async fn upsert_network(&self, req: Request<Network>) -> Result<Response<Network>, Status> {
        let n = req.into_inner();
        let row = NetworkRow {
            chain_index: n.chain_index,
            name: n.name,
            family: n.family,
            evm_chain_id: if n.evm_chain_id == 0 {
                None
            } else {
                Some(n.evm_chain_id as i64)
            },
            enabled: n.enabled,
            created_at: jiff::Timestamp::now(),
        };
        let saved = wallet_domain::network::NetworkService::new(&self.0)
            .upsert(row)
            .await?;
        Ok(Response::new(network_from_row(saved)))
    }

    async fn list_networks(
        &self,
        req: Request<ListNetworksRequest>,
    ) -> Result<Response<ListNetworksResponse>, Status> {
        let items = wallet_domain::network::NetworkService::new(&self.0)
            .list(req.into_inner().enabled_only)
            .await?
            .into_iter()
            .map(network_from_row)
            .collect();
        Ok(Response::new(ListNetworksResponse { items }))
    }
}

#[tonic::async_trait]
impl AdminTokenService for AdminTokenSvc {
    async fn upsert_token(&self, req: Request<Token>) -> Result<Response<Token>, Status> {
        let t = req.into_inner();
        let row = TokenRow {
            id: Uuid::parse_str(&t.id).unwrap_or_else(|_| Uuid::new_v4()),
            chain_index: t.chain_index,
            address: t.address,
            symbol: t.symbol,
            name: t.name,
            decimals: t.decimals as i32,
            logo_url: if t.logo_url.is_empty() {
                None
            } else {
                Some(t.logo_url)
            },
            created_at: jiff::Timestamp::now(),
        };
        let saved = wallet_domain::token::TokenService::new(&self.0)
            .upsert(row)
            .await?;
        Ok(Response::new(token_from_row(saved)))
    }

    async fn list_tokens(
        &self,
        req: Request<ListTokensRequest>,
    ) -> Result<Response<ListTokensResponse>, Status> {
        let r = req.into_inner();
        let p = r
            .pagination
            .unwrap_or(wallet_proto::wallet::v1::Pagination {
                page: 1,
                page_size: 20,
            });
        let (items, total) = wallet_domain::token::TokenService::new(&self.0)
            .list(ChainIndex(r.chain_index), p.page.max(1), p.page_size.max(1))
            .await?;
        Ok(Response::new(ListTokensResponse {
            items: items.into_iter().map(token_from_row).collect(),
            meta: Some(PageMeta {
                page: p.page.max(1),
                page_size: p.page_size.max(1),
                total: total as u64,
            }),
        }))
    }
}

#[tonic::async_trait]
impl AdminDappService for AdminDappSvc {
    async fn upsert_dapp(&self, req: Request<Dapp>) -> Result<Response<Dapp>, Status> {
        let d = req.into_inner();
        let row = DappRow {
            id: Uuid::parse_str(&d.id).unwrap_or_else(|_| Uuid::new_v4()),
            name: d.name,
            url: d.url,
            logo_url: if d.logo_url.is_empty() {
                None
            } else {
                Some(d.logo_url)
            },
            chain_indexes: d.chain_indexes,
            created_at: jiff::Timestamp::now(),
        };
        let saved = wallet_domain::dapp::DappService::new(&self.0)
            .upsert(row)
            .await?;
        Ok(Response::new(dapp_from_row(saved)))
    }

    async fn list_dapps(
        &self,
        req: Request<ListDappsRequest>,
    ) -> Result<Response<ListDappsResponse>, Status> {
        let p = req
            .into_inner()
            .pagination
            .unwrap_or(wallet_proto::wallet::v1::Pagination {
                page: 1,
                page_size: 20,
            });
        let (items, total) = wallet_domain::dapp::DappService::new(&self.0)
            .list(p.page.max(1), p.page_size.max(1))
            .await?;
        Ok(Response::new(ListDappsResponse {
            items: items.into_iter().map(dapp_from_row).collect(),
            meta: Some(PageMeta {
                page: p.page.max(1),
                page_size: p.page_size.max(1),
                total: total as u64,
            }),
        }))
    }
}

#[tonic::async_trait]
impl AdminRpcEndpointService for AdminRpcSvc {
    async fn list_endpoints(
        &self,
        req: Request<ListEndpointsRequest>,
    ) -> Result<Response<ListEndpointsResponse>, Status> {
        let items = NetworkRepo::new(&self.0.db)
            .list_endpoints(ChainIndex(req.into_inner().chain_index))
            .await?
            .into_iter()
            .map(|e| RpcEndpoint {
                id: e.id.to_string(),
                chain_index: e.chain_index,
                url: e.url,
                weight: e.weight as u32,
                enabled: e.enabled,
                headers: serde_json::from_value(e.headers).unwrap_or_default(),
                protocol: e.protocol,
            })
            .collect();
        Ok(Response::new(ListEndpointsResponse { items }))
    }

    async fn upsert_endpoint(
        &self,
        req: Request<RpcEndpoint>,
    ) -> Result<Response<RpcEndpoint>, Status> {
        let e = req.into_inner();
        let headers = serde_json::to_value(&e.headers)
            .map_err(|err| Status::invalid_argument(format!("invalid headers: {err}")))?;
        let row = wallet_db::RpcEndpointRow {
            id: Uuid::parse_str(&e.id).unwrap_or_else(|_| Uuid::new_v4()),
            chain_index: e.chain_index,
            url: e.url.clone(),
            protocol: e.protocol.clone(),
            weight: e.weight as i32,
            enabled: e.enabled,
            tier: String::from("free"),
            is_archive: false,
            priority: 0,
            headers,
            last_health_check: None,
            healthy: true,
            archive_checked_at: None,
            avg_latency_ms: None,
            block_height: None,
            error_count: 0,
            created_at: jiff::Timestamp::now(),
        };
        let saved = NetworkRepo::new(&self.0.db).upsert_endpoint(&row).await?;
        Ok(Response::new(RpcEndpoint {
            id: saved.id.to_string(),
            chain_index: saved.chain_index,
            url: saved.url,
            weight: saved.weight as u32,
            enabled: saved.enabled,
            headers: serde_json::from_value(saved.headers).unwrap_or_default(),
            protocol: saved.protocol,
        }))
    }

    async fn delete_endpoint(
        &self,
        req: Request<DeleteEndpointRequest>,
    ) -> Result<Response<Empty>, Status> {
        let r = req.into_inner();
        let id = Uuid::parse_str(&r.id)
            .map_err(|e| Status::invalid_argument(format!("invalid endpoint id: {e}")))?;
        NetworkRepo::new(&self.0.db)
            .delete_endpoint(id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(Empty {}))
    }
}

#[tonic::async_trait]
impl AdminGasPoolService for AdminGasSvc {
    async fn set_enabled(
        &self,
        req: Request<SetGasPoolEnabledRequest>,
    ) -> Result<Response<Empty>, Status> {
        let r = req.into_inner();
        wallet_domain::gaspool::GasPoolService::new(&self.0)
            .set_enabled(ChainIndex(r.chain_index), r.enabled)
            .await?;
        Ok(Response::new(Empty {}))
    }
}

#[tonic::async_trait]
impl AdminSwapService for AdminSwapSvc {
    async fn upsert_provider(
        &self,
        req: Request<SwapProvider>,
    ) -> Result<Response<SwapProvider>, Status> {
        let p = req.into_inner();
        let mut db = self.0.db.clone_inner();
        let config_str = if p.config_json.is_empty() {
            "{}".to_string()
        } else {
            p.config_json.clone()
        };
        // Upsert swap provider using raw SQL via toasty
        toasty::sql::statement(
            r#"
            INSERT INTO swap_providers (name, enabled, config_json)
            VALUES ($1, $2, $3::jsonb)
            ON CONFLICT (name) DO UPDATE SET enabled = EXCLUDED.enabled, config_json = EXCLUDED.config_json
            "#,
        )
        .bind(&p.name)
        .bind(p.enabled)
        .bind(&config_str)
        .exec(&mut db)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(p))
    }
}

#[tonic::async_trait]
impl AdminTransactionService for AdminTxSvc {
    async fn reindex(&self, req: Request<ReindexRequest>) -> Result<Response<Empty>, Status> {
        let r = req.into_inner();
        tracing::info!(
            chain_index = r.chain_index,
            from = r.from_block,
            to = r.to_block,
            "admin reindex requested"
        );
        self.0
            .events
            .publish(
                "wallet.sync.reindex",
                serde_json::json!({
                    "chain_index": r.chain_index,
                    "from_block": r.from_block,
                    "to_block": r.to_block,
                }),
            )
            .await
            .map_err(|e| Status::internal(format!("publish reindex event: {e}")))?;
        Ok(Response::new(Empty {}))
    }
}

#[tonic::async_trait]
impl AdminCmsService for AdminCmsSvc {
    async fn upsert_app_config(
        &self,
        req: Request<AppConfig>,
    ) -> Result<Response<AppConfig>, Status> {
        let cfg = req.into_inner();
        let mut db = self.0.db.clone_inner();
        toasty::sql::statement(
            r#"
            INSERT INTO app_configs (platform, min_version, latest_version, force_update_url, features_json)
            VALUES ($1, $2, $3, $4, $5::jsonb)
            ON CONFLICT (platform) DO UPDATE
            SET min_version = EXCLUDED.min_version,
                latest_version = EXCLUDED.latest_version,
                force_update_url = EXCLUDED.force_update_url,
                features_json = EXCLUDED.features_json
            "#,
        )
        .bind(&cfg.platform)
        .bind(&cfg.min_version)
        .bind(&cfg.latest_version)
        .bind(&cfg.force_update_url)
        .bind(&cfg.features_json)
        .exec(&mut db)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(cfg))
    }

    async fn get_app_config(
        &self,
        req: Request<GetAppConfigRequest>,
    ) -> Result<Response<AppConfig>, Status> {
        let platform = req.into_inner().platform;
        let mut db = self.0.db.clone_inner();
        let rows: Vec<wallet_db::AppConfig> =
            wallet_db::AppConfig::filter(wallet_db::AppConfig::fields().platform().eq(&platform))
                .exec(&mut db)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

        let row = rows
            .into_iter()
            .next()
            .ok_or_else(|| Status::not_found("app config not found"))?;

        Ok(Response::new(AppConfig {
            platform: row.platform,
            min_version: row.min_version,
            latest_version: row.latest_version,
            force_update_url: row.force_update_url.unwrap_or_default(),
            features_json: row.features_json,
        }))
    }

    async fn upsert_guide(&self, req: Request<Guide>) -> Result<Response<Guide>, Status> {
        let g = req.into_inner();
        let id = Uuid::parse_str(&g.id).unwrap_or_else(|_| Uuid::new_v4());
        let mut db = self.0.db.clone_inner();
        toasty::sql::statement(
            r#"
            INSERT INTO guides (id, locale, title, body)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE SET locale = EXCLUDED.locale, title = EXCLUDED.title, body = EXCLUDED.body
            "#,
        )
        .bind(id)
        .bind(&g.locale)
        .bind(&g.title)
        .bind(&g.body)
        .exec(&mut db)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(Guide {
            id: id.to_string(),
            title: g.title,
            body: g.body,
            locale: g.locale,
        }))
    }

    async fn delete_guide(
        &self,
        req: Request<DeleteGuideRequest>,
    ) -> Result<Response<Empty>, Status> {
        let r = req.into_inner();
        let id = Uuid::parse_str(&r.id)
            .map_err(|e| Status::invalid_argument(format!("invalid guide id: {e}")))?;
        let mut db = self.0.db.clone_inner();
        toasty::sql::statement("DELETE FROM guides WHERE id = $1")
            .bind(id)
            .exec(&mut db)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(Empty {}))
    }

    async fn list_guides(
        &self,
        req: Request<ListGuidesRequest>,
    ) -> Result<Response<ListGuidesResponse>, Status> {
        let r = req.into_inner();
        let locale = r.locale;
        let p = r
            .pagination
            .unwrap_or(wallet_proto::wallet::v1::Pagination {
                page: 1,
                page_size: 20,
            });
        let page = p.page.max(1);
        let page_size = p.page_size.clamp(1, 100);
        let limit = page_size as usize;
        let offset = ((page - 1) * page_size) as usize;

        let mut db = self.0.db.clone_inner();
        let rows: Vec<wallet_db::Guide> =
            wallet_db::Guide::filter(wallet_db::Guide::fields().locale().eq(&locale))
                .limit(limit)
                .offset(offset)
                .exec(&mut db)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

        let total: u64 = wallet_db::Guide::filter(wallet_db::Guide::fields().locale().eq(&locale))
            .count()
            .exec(&mut db)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListGuidesResponse {
            items: rows
                .into_iter()
                .map(|g| Guide {
                    id: g.id.to_string(),
                    title: g.title,
                    body: g.body,
                    locale: g.locale,
                })
                .collect(),
            meta: Some(PageMeta {
                page,
                page_size,
                total: total as u64,
            }),
        }))
    }
}
