mod admin;
mod service;

use clap::Parser;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tonic::transport::Server;
use tracing_subscriber::EnvFilter;
use wallet_chain::build_registry;
use wallet_config::{load_yaml, ApiConfig};
use wallet_db::{clickhouse::ClickHouseDb, Db};
use wallet_domain::AppState;
use wallet_events::{MemoryEventBus, NatsEventBus};
use wallet_proto::wallet::v1::admin::{
    admin_cms_service_server::AdminCmsServiceServer,
    admin_dapp_service_server::AdminDappServiceServer,
    admin_gas_pool_service_server::AdminGasPoolServiceServer,
    admin_network_service_server::AdminNetworkServiceServer,
    admin_rpc_endpoint_service_server::AdminRpcEndpointServiceServer,
    admin_swap_service_server::AdminSwapServiceServer,
    admin_token_service_server::AdminTokenServiceServer,
    admin_transaction_service_server::AdminTransactionServiceServer,
    admin_user_service_server::AdminUserServiceServer,
};
use wallet_proto::wallet::v1::{
    cms_service_server::CmsServiceServer, dapp_service_server::DappServiceServer,
    gas_pool_service_server::GasPoolServiceServer, market_service_server::MarketServiceServer,
    network_service_server::NetworkServiceServer, rent_service_server::RentServiceServer,
    solana_service_server::SolanaServiceServer, swap_service_server::SwapServiceServer,
    token_service_server::TokenServiceServer, transaction_service_server::TransactionServiceServer,
    user_service_server::UserServiceServer,
};

use crate::admin::*;
use crate::service::*;

#[derive(Parser, Debug)]
#[command(name = "wallet-api")]
struct Args {
    #[arg(long, default_value = "configs/wallet-api.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let args = Args::parse();
    let cfg: ApiConfig = load_yaml(&args.config)?;

    let db = Db::connect(&cfg.database).await?;
    if let Err(e) = db.migrate().await {
        tracing::warn!("migrate skipped/failed: {e}");
    }

    let chains = build_registry(&cfg.chains)?;
    let events = match NatsEventBus::connect(&cfg.nats.url).await {
        Ok(bus) => bus as Arc<dyn wallet_events::EventBus>,
        Err(e) => {
            tracing::warn!("nats unavailable ({e}), using memory bus");
            MemoryEventBus::new(1024) as Arc<dyn wallet_events::EventBus>
        }
    };

    let clickhouse = cfg.clickhouse.as_ref().map(ClickHouseDb::connect);
    if let Some(ch) = &clickhouse {
        if let Err(e) = ch.ensure_schema().await {
            tracing::warn!("clickhouse schema: {e}");
        }
    }

    let state = Arc::new(AppState::new(db, chains, events, clickhouse));
    let addr: SocketAddr = cfg.listen.parse()?;

    tracing::info!("wallet-api listening on {addr}");

    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        tracing::info!("shutting down wallet-api...");
        let _ = shutdown_tx.send(true);
    });

    Server::builder()
        .add_service(UserServiceServer::new(UserSvc(state.clone())))
        .add_service(TokenServiceServer::new(TokenSvc(state.clone())))
        .add_service(TransactionServiceServer::new(TxSvc(state.clone())))
        .add_service(NetworkServiceServer::new(NetworkSvc(state.clone())))
        .add_service(SwapServiceServer::new(SwapSvc(state.clone())))
        .add_service(GasPoolServiceServer::new(GasPoolSvc(state.clone())))
        .add_service(MarketServiceServer::new(MarketSvc(state.clone())))
        .add_service(DappServiceServer::new(DappSvc(state.clone())))
        .add_service(RentServiceServer::new(RentSvc))
        .add_service(SolanaServiceServer::new(SolanaSvc(state.clone())))
        .add_service(CmsServiceServer::new(CmsSvc(state.clone())))
        .add_service(AdminUserServiceServer::new(AdminUserSvc(state.clone())))
        .add_service(AdminNetworkServiceServer::new(AdminNetworkSvc(
            state.clone(),
        )))
        .add_service(AdminTokenServiceServer::new(AdminTokenSvc(state.clone())))
        .add_service(AdminDappServiceServer::new(AdminDappSvc(state.clone())))
        .add_service(AdminRpcEndpointServiceServer::new(AdminRpcSvc(
            state.clone(),
        )))
        .add_service(AdminGasPoolServiceServer::new(AdminGasSvc(state.clone())))
        .add_service(AdminSwapServiceServer::new(AdminSwapSvc(state.clone())))
        .add_service(AdminTransactionServiceServer::new(AdminTxSvc(
            state.clone(),
        )))
        .add_service(AdminCmsServiceServer::new(AdminCmsSvc(state.clone())))
        .serve_with_shutdown(addr, async move {
            shutdown_rx.clone().changed().await.ok();
            tokio::time::sleep(Duration::from_secs(5)).await;
        })
        .await?;

    Ok(())
}
