use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use wallet_domain::AppState;
use wallet_proto::wallet::v1::cms_service_server::CmsService;
use wallet_proto::wallet::v1::dapp_service_server::DappService;
use wallet_proto::wallet::v1::gas_pool_service_server::GasPoolService;
use wallet_proto::wallet::v1::market_service_server::MarketService;
use wallet_proto::wallet::v1::network_service_server::NetworkService;
use wallet_proto::wallet::v1::rent_service_server::RentService;
use wallet_proto::wallet::v1::solana_service_server::SolanaService;
use wallet_proto::wallet::v1::swap_service_server::SwapService;
use wallet_proto::wallet::v1::token_service_server::TokenService;
use wallet_proto::wallet::v1::transaction_service_server::TransactionService;
use wallet_proto::wallet::v1::user_service_server::UserService;
use wallet_proto::wallet::v1::*;
use wallet_types::{Address, ChainIndex};

pub struct UserSvc(pub Arc<AppState>);
pub struct TokenSvc(pub Arc<AppState>);
pub struct TxSvc(pub Arc<AppState>);
pub struct NetworkSvc(pub Arc<AppState>);
pub struct SwapSvc(pub Arc<AppState>);
pub struct GasPoolSvc(pub Arc<AppState>);
pub struct MarketSvc(pub Arc<AppState>);
pub struct DappSvc(pub Arc<AppState>);
pub struct RentSvc;
pub struct SolanaSvc(pub Arc<AppState>);
pub struct CmsSvc(pub Arc<AppState>);

#[tonic::async_trait]
impl UserService for UserSvc {
    async fn get_user(&self, req: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        let id = Uuid::parse_str(&req.into_inner().id)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let u = wallet_domain::user::UserService::new(&self.0)
            .get(id)
            .await?;
        Ok(Response::new(User {
            id: u.id.to_string(),
            external_id: u.external_id,
            created_at_unix: u.created_at.as_second(),
        }))
    }

    async fn register(&self, req: Request<RegisterRequest>) -> Result<Response<User>, Status> {
        let r = req.into_inner();
        let u = wallet_domain::user::UserService::new(&self.0)
            .register(&r.external_id)
            .await?;
        Ok(Response::new(User {
            id: u.id.to_string(),
            external_id: u.external_id,
            created_at_unix: u.created_at.as_second(),
        }))
    }

    async fn list_addresses(
        &self,
        req: Request<ListAddressesRequest>,
    ) -> Result<Response<ListAddressesResponse>, Status> {
        let r = req.into_inner();
        let user_id = Uuid::parse_str(&r.user_id)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let ci = if r.chain_index == 0 {
            None
        } else {
            Some(ChainIndex(r.chain_index))
        };
        let items = wallet_domain::user::UserService::new(&self.0)
            .list_addresses(user_id, ci)
            .await?
            .into_iter()
            .map(|a| AddressInfo {
                address: a.address,
                chain_index: a.chain_index,
            })
            .collect();
        Ok(Response::new(ListAddressesResponse { items }))
    }
}

#[tonic::async_trait]
impl TokenService for TokenSvc {
    async fn get_token(&self, req: Request<GetTokenRequest>) -> Result<Response<Token>, Status> {
        let r = req.into_inner();
        let t = wallet_domain::token::TokenService::new(&self.0)
            .get(ChainIndex(r.chain_index), &r.address)
            .await?;
        Ok(Response::new(token_from_row(t)))
    }

    async fn list_tokens(
        &self,
        req: Request<ListTokensRequest>,
    ) -> Result<Response<ListTokensResponse>, Status> {
        let r = req.into_inner();
        let (page, page_size) = page_of(r.pagination.as_ref());
        let (items, total) = wallet_domain::token::TokenService::new(&self.0)
            .list(ChainIndex(r.chain_index), page, page_size)
            .await?;
        Ok(Response::new(ListTokensResponse {
            items: items.into_iter().map(token_from_row).collect(),
            meta: Some(PageMeta {
                page,
                page_size,
                total: total as u64,
            }),
        }))
    }

    async fn get_balances(
        &self,
        req: Request<GetBalancesRequest>,
    ) -> Result<Response<GetBalancesResponse>, Status> {
        let r = req.into_inner();
        let tokens: Vec<_> = r.tokens.iter().map(Address::new).collect();
        let items = wallet_domain::token::TokenService::new(&self.0)
            .balances(
                ChainIndex(r.chain_index),
                &Address::new(r.wallet),
                &tokens,
            )
            .await?
            .into_iter()
            .map(|(token, amount)| BalanceItem {
                token,
                amount: amount.raw.to_string(),
                decimals: amount.decimals,
            })
            .collect();
        Ok(Response::new(GetBalancesResponse { items }))
    }
}

#[tonic::async_trait]
impl TransactionService for TxSvc {
    async fn get_transaction(
        &self,
        req: Request<GetTransactionRequest>,
    ) -> Result<Response<Transaction>, Status> {
        let r = req.into_inner();
        let t = wallet_domain::transaction::TransactionService::new(&self.0)
            .get(ChainIndex(r.chain_index), &r.hash)
            .await?;
        Ok(Response::new(tx_from_row(t)))
    }

    async fn list_transactions(
        &self,
        req: Request<ListTransactionsRequest>,
    ) -> Result<Response<ListTransactionsResponse>, Status> {
        let r = req.into_inner();
        let (page, page_size) = page_of(r.pagination.as_ref());
        let (items, total) = wallet_domain::transaction::TransactionService::new(&self.0)
            .list(ChainIndex(r.chain_index), &r.address, page, page_size)
            .await?;
        Ok(Response::new(ListTransactionsResponse {
            items: items.into_iter().map(tx_from_row).collect(),
            meta: Some(PageMeta {
                page,
                page_size,
                total: total as u64,
            }),
        }))
    }

    async fn broadcast(
        &self,
        req: Request<BroadcastRequest>,
    ) -> Result<Response<BroadcastResponse>, Status> {
        let r = req.into_inner();
        let hash = wallet_domain::transaction::TransactionService::new(&self.0)
            .broadcast(ChainIndex(r.chain_index), &r.raw_tx)
            .await?;
        Ok(Response::new(BroadcastResponse {
            hash: hash.to_string(),
        }))
    }

    async fn estimate_gas(
        &self,
        req: Request<EstimateGasRequest>,
    ) -> Result<Response<EstimateGasResponse>, Status> {
        let r = req.into_inner();
        let to = if r.to_address.is_empty() {
            None
        } else {
            Some(Address::new(r.to_address))
        };
        let data = if r.data.is_empty() {
            None
        } else {
            Some(r.data)
        };
        let est = wallet_domain::transaction::TransactionService::new(&self.0)
            .estimate_gas(
                ChainIndex(r.chain_index),
                Address::new(r.from_address),
                to,
                data,
                None,
            )
            .await?;
        Ok(Response::new(EstimateGasResponse {
            gas_limit: est.gas_limit,
            max_fee_per_gas: est
                .max_fee_per_gas
                .map(|v| v.to_string())
                .unwrap_or_default(),
            max_priority_fee_per_gas: est
                .max_priority_fee_per_gas
                .map(|v| v.to_string())
                .unwrap_or_default(),
        }))
    }
}

#[tonic::async_trait]
impl NetworkService for NetworkSvc {
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

    async fn get_network(
        &self,
        req: Request<GetNetworkRequest>,
    ) -> Result<Response<Network>, Status> {
        let n = wallet_domain::network::NetworkService::new(&self.0)
            .get(ChainIndex(req.into_inner().chain_index))
            .await?;
        Ok(Response::new(network_from_row(n)))
    }

    async fn get_tip(
        &self,
        req: Request<GetTipRequest>,
    ) -> Result<Response<GetTipResponse>, Status> {
        let tip = wallet_domain::network::NetworkService::new(&self.0)
            .tip(ChainIndex(req.into_inner().chain_index))
            .await?;
        Ok(Response::new(GetTipResponse {
            block_number: tip,
        }))
    }
}

#[tonic::async_trait]
impl SwapService for SwapSvc {
    async fn quote(&self, req: Request<QuoteRequest>) -> Result<Response<QuoteResponse>, Status> {
        let r = req.into_inner();
        let q = wallet_domain::swap::SwapService::new(&self.0)
            .quote(
                ChainIndex(r.chain_index),
                &Address::new(r.from_token),
                &Address::new(r.to_token),
                &r.amount,
                &r.provider,
            )
            .await?;
        Ok(Response::new(QuoteResponse {
            provider: q.provider,
            amount_out: q.amount_out,
            route_json: q.route_json,
        }))
    }

    async fn build_swap(
        &self,
        req: Request<BuildSwapRequest>,
    ) -> Result<Response<BuildSwapResponse>, Status> {
        let r = req.into_inner();
        let b = wallet_domain::swap::SwapService::new(&self.0)
            .build(
                ChainIndex(r.chain_index),
                &Address::new(r.from_token),
                &Address::new(r.to_token),
                &r.amount,
                &r.slippage_bps,
                &Address::new(r.user_address),
                &r.provider,
            )
            .await?;
        Ok(Response::new(BuildSwapResponse {
            provider: b.provider,
            tx_data: b.tx_data,
            to_address: b.to_address,
            value: b.value,
        }))
    }
}

#[tonic::async_trait]
impl GasPoolService for GasPoolSvc {
    async fn get_pool(&self, req: Request<GetPoolRequest>) -> Result<Response<GasPool>, Status> {
        let p = wallet_domain::gaspool::GasPoolService::new(&self.0)
            .get(ChainIndex(req.into_inner().chain_index))
            .await?;
        Ok(Response::new(GasPool {
            chain_index: p.chain_index,
            balance: p.balance.to_string(),
            enabled: p.enabled,
        }))
    }

    async fn sponsor(
        &self,
        req: Request<SponsorRequest>,
    ) -> Result<Response<SponsorResponse>, Status> {
        let r = req.into_inner();
        let res = wallet_domain::gaspool::GasPoolService::new(&self.0)
            .sponsor(
                ChainIndex(r.chain_index),
                &Address::new(r.user_address),
                &r.user_op_or_tx,
            )
            .await?;
        Ok(Response::new(SponsorResponse {
            status: res.status,
            tx_hash: res.tx_hash,
        }))
    }
}

#[tonic::async_trait]
impl MarketService for MarketSvc {
    async fn get_price(&self, req: Request<GetPriceRequest>) -> Result<Response<Price>, Status> {
        let p = wallet_domain::market::MarketService::new(&self.0)
            .get_price(&req.into_inner().symbol)
            .await?;
        Ok(Response::new(Price {
            symbol: p.symbol,
            price: p.price,
            updated_at_unix: p.updated_at_unix,
        }))
    }

    async fn get_klines(
        &self,
        req: Request<GetKlinesRequest>,
    ) -> Result<Response<GetKlinesResponse>, Status> {
        let r = req.into_inner();
        let items = wallet_domain::market::MarketService::new(&self.0)
            .get_klines(&r.symbol, &r.interval)
            .await?
            .into_iter()
            .map(|k| Kline {
                open_time: k.open_time,
                open: k.open,
                high: k.high,
                low: k.low,
                close: k.close,
                volume: k.volume,
            })
            .collect();
        Ok(Response::new(GetKlinesResponse { items }))
    }
}

#[tonic::async_trait]
impl DappService for DappSvc {
    async fn list_dapps(
        &self,
        req: Request<ListDappsRequest>,
    ) -> Result<Response<ListDappsResponse>, Status> {
        let (page, page_size) = page_of(req.into_inner().pagination.as_ref());
        let (items, total) = wallet_domain::dapp::DappService::new(&self.0)
            .list(page, page_size)
            .await?;
        Ok(Response::new(ListDappsResponse {
            items: items.into_iter().map(dapp_from_row).collect(),
            meta: Some(PageMeta {
                page,
                page_size,
                total: total as u64,
            }),
        }))
    }

    async fn get_dapp(&self, req: Request<GetDappRequest>) -> Result<Response<Dapp>, Status> {
        let id = Uuid::parse_str(&req.into_inner().id)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let d = wallet_domain::dapp::DappService::new(&self.0).get(id).await?;
        Ok(Response::new(dapp_from_row(d)))
    }
}

#[tonic::async_trait]
impl RentService for RentSvc {
    async fn estimate_energy(
        &self,
        req: Request<EstimateEnergyRequest>,
    ) -> Result<Response<EstimateEnergyResponse>, Status> {
        let r = req.into_inner();
        let svc = wallet_domain::rent::RentService::new();
        let e = svc
            .estimate(&Address::new(r.address), r.energy)
            .await?;
        Ok(Response::new(EstimateEnergyResponse {
            price: e.price,
            duration_hours: e.duration_hours,
        }))
    }

    async fn order_rent(
        &self,
        req: Request<OrderRentRequest>,
    ) -> Result<Response<OrderRentResponse>, Status> {
        let r = req.into_inner();
        let svc = wallet_domain::rent::RentService::new();
        let o = svc
            .order(&Address::new(r.address), r.energy, r.duration_hours)
            .await?;
        Ok(Response::new(OrderRentResponse {
            order_id: o.order_id,
            status: o.status,
        }))
    }
}

#[tonic::async_trait]
impl SolanaService for SolanaSvc {
    async fn get_ata(
        &self,
        req: Request<GetAtaRequest>,
    ) -> Result<Response<GetAtaResponse>, Status> {
        let r = req.into_inner();
        let sol_chain = self.0.chains.get(wallet_types::ChainIndex::SOL)
            .map_err(|e| Status::internal(e.to_string()))?;
        let (ata, exists) = sol_chain.ata_address_rpc(
            &Address::new(r.owner),
            &Address::new(r.mint),
        ).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(GetAtaResponse {
            ata: ata.to_string(),
            exists,
        }))
    }

    async fn ensure_ata(
        &self,
        req: Request<EnsureAtaRequest>,
    ) -> Result<Response<EnsureAtaResponse>, Status> {
        let r = req.into_inner();
        let sol_chain = self.0.chains.get(wallet_types::ChainIndex::SOL)
            .map_err(|e| Status::internal(e.to_string()))?;
        let (ata, exists) = sol_chain.ata_address_rpc(
            &Address::new(r.owner),
            &Address::new(r.mint),
        ).await.map_err(|e| Status::internal(e.to_string()))?;
        let create_ix = if exists {
            vec![]
        } else {
            vec![0u8]
        };
        Ok(Response::new(EnsureAtaResponse {
            ata: ata.to_string(),
            create_ix,
        }))
    }
}

#[tonic::async_trait]
impl CmsService for CmsSvc {
    async fn get_app_config(
        &self,
        req: Request<GetAppConfigRequest>,
    ) -> Result<Response<AppConfig>, Status> {
        let r = req.into_inner();
        let c = wallet_domain::cms::CmsService::new(&self.0)
            .get_app_config(&r.platform)
            .await?;
        Ok(Response::new(AppConfig {
            min_version: c.min_version,
            latest_version: c.latest_version,
            force_update_url: c.force_update_url.unwrap_or_default(),
            features_json: c.features_json.to_string(),
        }))
    }

    async fn list_guides(
        &self,
        req: Request<ListGuidesRequest>,
    ) -> Result<Response<ListGuidesResponse>, Status> {
        let r = req.into_inner();
        let (page, page_size) = page_of(r.pagination.as_ref());
        let (items, total) = wallet_domain::cms::CmsService::new(&self.0)
            .list_guides(&r.locale, page, page_size)
            .await?;
        Ok(Response::new(ListGuidesResponse {
            items: items
                .into_iter()
                .map(|g| Guide {
                    id: g.id.to_string(),
                    title: g.title,
                    body: g.body,
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

pub fn page_of(p: Option<&Pagination>) -> (u32, u32) {
    let p = p.cloned().unwrap_or(Pagination {
        page: 1,
        page_size: 20,
    });
    (p.page.max(1), p.page_size.max(1).min(100))
}

pub fn token_from_row(t: wallet_db::TokenRow) -> Token {
    Token {
        id: t.id.to_string(),
        chain_index: t.chain_index,
        address: t.address,
        symbol: t.symbol,
        name: t.name,
        decimals: t.decimals as u32,
        logo_url: t.logo_url.unwrap_or_default(),
    }
}

pub fn tx_from_row(t: wallet_db::TxRow) -> Transaction {
    Transaction {
        hash: t.hash,
        chain_index: t.chain_index,
        from_address: t.from_address.unwrap_or_default(),
        to_address: t.to_address.unwrap_or_default(),
        value: t.value.to_string(),
        block_number: t.block_number as u64,
        status: t.status,
    }
}

pub fn network_from_row(n: wallet_db::NetworkRow) -> Network {
    Network {
        chain_index: n.chain_index,
        name: n.name,
        family: n.family,
        evm_chain_id: n.evm_chain_id.unwrap_or(0) as u64,
        enabled: n.enabled,
    }
}

pub fn dapp_from_row(d: wallet_db::DappRow) -> Dapp {
    Dapp {
        id: d.id.to_string(),
        name: d.name,
        url: d.url,
        logo_url: d.logo_url.unwrap_or_default(),
        chain_indexes: d.chain_indexes,
    }
}
