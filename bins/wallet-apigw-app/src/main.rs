mod handler;
mod middleware;
mod types;

use clap::Parser;
use salvo::cors::Cors;
use salvo::prelude::*;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use wallet_config::{load_yaml, GatewayConfig};

mod state_injector {
    use super::*;

    pub struct StateInjector<T: Clone + Send + Sync + 'static>(pub T);

    #[async_trait]
    impl<T: Clone + Send + Sync + 'static> Handler for StateInjector<T> {
        async fn handle(
            &self,
            _req: &mut Request,
            depot: &mut Depot,
            _res: &mut Response,
            flow: &mut FlowCtrl,
        ) {
            depot.insert_typed(self.0.clone());
            flow.call_next(_req, depot, _res).await;
        }
    }
}

use state_injector::StateInjector;

#[derive(Clone)]
pub struct GwState {
    pub cfg: GatewayConfig,
    pub metrics: wallet_domain::Metrics,
    pub user:
        wallet_proto::wallet::v1::user_service_client::UserServiceClient<tonic::transport::Channel>,
    pub token: wallet_proto::wallet::v1::token_service_client::TokenServiceClient<
        tonic::transport::Channel,
    >,
    pub tx: wallet_proto::wallet::v1::transaction_service_client::TransactionServiceClient<
        tonic::transport::Channel,
    >,
    pub network: wallet_proto::wallet::v1::network_service_client::NetworkServiceClient<
        tonic::transport::Channel,
    >,
    pub swap:
        wallet_proto::wallet::v1::swap_service_client::SwapServiceClient<tonic::transport::Channel>,
    pub market: wallet_proto::wallet::v1::market_service_client::MarketServiceClient<
        tonic::transport::Channel,
    >,
    pub dapp:
        wallet_proto::wallet::v1::dapp_service_client::DappServiceClient<tonic::transport::Channel>,
    pub rent:
        wallet_proto::wallet::v1::rent_service_client::RentServiceClient<tonic::transport::Channel>,
    pub cms:
        wallet_proto::wallet::v1::cms_service_client::CmsServiceClient<tonic::transport::Channel>,
    pub gaspool: wallet_proto::wallet::v1::gas_pool_service_client::GasPoolServiceClient<
        tonic::transport::Channel,
    >,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "configs/apigw-app.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();
    let args = Args::parse();
    let cfg: GatewayConfig = load_yaml(&args.config)?;
    let channel = tonic::transport::Endpoint::from_shared(cfg.grpc_endpoint.clone())?
        .connect()
        .await?;

    let state = Arc::new(GwState {
        cfg: cfg.clone(),
        metrics: wallet_domain::Metrics::new(),
        user: wallet_proto::wallet::v1::user_service_client::UserServiceClient::new(
            channel.clone(),
        ),
        token: wallet_proto::wallet::v1::token_service_client::TokenServiceClient::new(
            channel.clone(),
        ),
        tx: wallet_proto::wallet::v1::transaction_service_client::TransactionServiceClient::new(
            channel.clone(),
        ),
        network: wallet_proto::wallet::v1::network_service_client::NetworkServiceClient::new(
            channel.clone(),
        ),
        swap: wallet_proto::wallet::v1::swap_service_client::SwapServiceClient::new(
            channel.clone(),
        ),
        market: wallet_proto::wallet::v1::market_service_client::MarketServiceClient::new(
            channel.clone(),
        ),
        dapp: wallet_proto::wallet::v1::dapp_service_client::DappServiceClient::new(
            channel.clone(),
        ),
        rent: wallet_proto::wallet::v1::rent_service_client::RentServiceClient::new(
            channel.clone(),
        ),
        cms: wallet_proto::wallet::v1::cms_service_client::CmsServiceClient::new(channel.clone()),
        gaspool: wallet_proto::wallet::v1::gas_pool_service_client::GasPoolServiceClient::new(
            channel,
        ),
    });

    let auth_router = Router::with_path("v1")
        .push(Router::with_path("tokens").get(handler::list_tokens))
        .push(Router::with_path("balances").get(handler::get_balances))
        .push(Router::with_path("transactions").get(handler::list_txs))
        .push(Router::with_path("transactions/broadcast").post(handler::broadcast))
        .push(Router::with_path("swap/quote").post(handler::swap_quote))
        .push(Router::with_path("gaspool/{chain_index}").get(handler::gas_pool))
        .push(Router::with_path("gaspool/sponsor").post(handler::gas_sponsor))
        .push(Router::with_path("transactions/estimate-gas").post(handler::estimate_gas))
        .push(Router::with_path("rent/estimate").post(handler::rent_estimate))
        .push(Router::with_path("dapps").get(handler::list_dapps))
        .hoop(middleware::require_app_jwt);

    let public_router = Router::with_path("v1")
        .push(Router::with_path("networks").get(handler::list_networks))
        .push(Router::with_path("market/price").get(handler::price))
        .push(Router::with_path("market/klines").get(handler::klines))
        .push(Router::with_path("cms/config").get(handler::app_config))
        .push(Router::with_path("cms/guides").get(handler::list_guides))
        .push(Router::with_path("users/register").post(handler::register))
        .push(Router::with_path("users/addresses").get(handler::list_addresses))
        .push(Router::with_path("tokens/list").get(handler::list_tokens));

    let cors = if cfg.allowed_origins.is_empty() {
        Cors::permissive()
    } else {
        use salvo::cors::AllowOrigin;
        let origins: Vec<String> = cfg.allowed_origins.clone();
        Cors::new()
            .allow_origin(AllowOrigin::list(
                origins.iter().filter_map(|o| o.parse().ok()),
            ))
            .allow_methods([
                salvo::http::Method::GET,
                salvo::http::Method::POST,
                salvo::http::Method::OPTIONS,
            ])
            .allow_headers(salvo::cors::AllowHeaders::mirror_request())
    };

    let app = Router::new()
        .push(Router::with_path("healthz").get(handler::healthz))
        .push(Router::with_path("metrics").get(handler::metrics))
        .push(auth_router)
        .push(public_router)
        .hoop(StateInjector(state))
        .hoop(cors.into_handler());

    let doc = OpenApi::new("Wallet App API", "0.1.0").merge_router(&app);
    let mut app = app.push(doc.into_router("/api-doc/openapi.json"));
    let is_prod = std::env::var("APP_ENV")
        .map(|v| v == "production" || v == "prod")
        .unwrap_or(false);
    if !is_prod {
        app = app.push(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"));
    }

    let addr: SocketAddr = cfg.listen.parse()?;
    tracing::info!("apigw-app on {addr}");
    let listener = TcpListener::new(addr.to_string()).bind().await;
    let server = Server::new(listener);
    let handle = server.handle();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        tracing::info!("shutting down apigw-app...");
        handle.stop_graceful(Some(std::time::Duration::from_secs(30)));
    });
    server.serve(app).await;
    Ok(())
}
