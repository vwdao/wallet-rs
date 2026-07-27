mod handler;
mod middleware;
mod types;

use clap::Parser;
use salvo::prelude::*;
use salvo::cors::Cors;
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
pub struct AdminState {
    pub cfg: GatewayConfig,
    pub networks:
        wallet_proto::wallet::v1::admin::admin_network_service_client::AdminNetworkServiceClient<
            tonic::transport::Channel,
        >,
    pub tokens:
        wallet_proto::wallet::v1::admin::admin_token_service_client::AdminTokenServiceClient<
            tonic::transport::Channel,
        >,
    pub users:
        wallet_proto::wallet::v1::admin::admin_user_service_client::AdminUserServiceClient<
            tonic::transport::Channel,
        >,
    pub dapps:
        wallet_proto::wallet::v1::admin::admin_dapp_service_client::AdminDappServiceClient<
            tonic::transport::Channel,
        >,
    pub rpc_endpoints:
        wallet_proto::wallet::v1::admin::admin_rpc_endpoint_service_client::AdminRpcEndpointServiceClient<
            tonic::transport::Channel,
        >,
    pub gas_pools:
        wallet_proto::wallet::v1::admin::admin_gas_pool_service_client::AdminGasPoolServiceClient<
            tonic::transport::Channel,
        >,
    pub swap:
        wallet_proto::wallet::v1::admin::admin_swap_service_client::AdminSwapServiceClient<
            tonic::transport::Channel,
        >,
    pub transactions:
        wallet_proto::wallet::v1::admin::admin_transaction_service_client::AdminTransactionServiceClient<
            tonic::transport::Channel,
        >,
    pub cms:
        wallet_proto::wallet::v1::admin::admin_cms_service_client::AdminCmsServiceClient<
            tonic::transport::Channel,
        >,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "configs/apigw-admin.yaml")]
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
    let state = Arc::new(AdminState {
        cfg: cfg.clone(),
        networks: wallet_proto::wallet::v1::admin::admin_network_service_client::AdminNetworkServiceClient::new(
            channel.clone(),
        ),
        tokens: wallet_proto::wallet::v1::admin::admin_token_service_client::AdminTokenServiceClient::new(
            channel.clone(),
        ),
        users: wallet_proto::wallet::v1::admin::admin_user_service_client::AdminUserServiceClient::new(
            channel.clone(),
        ),
        dapps: wallet_proto::wallet::v1::admin::admin_dapp_service_client::AdminDappServiceClient::new(
            channel.clone(),
        ),
        rpc_endpoints: wallet_proto::wallet::v1::admin::admin_rpc_endpoint_service_client::AdminRpcEndpointServiceClient::new(
            channel.clone(),
        ),
        gas_pools: wallet_proto::wallet::v1::admin::admin_gas_pool_service_client::AdminGasPoolServiceClient::new(
            channel.clone(),
        ),
        swap: wallet_proto::wallet::v1::admin::admin_swap_service_client::AdminSwapServiceClient::new(
            channel.clone(),
        ),
        transactions: wallet_proto::wallet::v1::admin::admin_transaction_service_client::AdminTransactionServiceClient::new(
            channel.clone(),
        ),
        cms: wallet_proto::wallet::v1::admin::admin_cms_service_client::AdminCmsServiceClient::new(
            channel,
        ),
    });

    let admin_router = Router::with_path("admin/v1")
        .push(
            Router::with_path("networks")
                .get(handler::list_networks)
                .post(handler::upsert_network),
        )
        .push(
            Router::with_path("tokens")
                .get(handler::list_tokens)
                .post(handler::upsert_token),
        )
        .push(Router::with_path("users").get(handler::list_users))
        .push(
            Router::with_path("dapps")
                .get(handler::list_dapps)
                .post(handler::upsert_dapp),
        )
        .push(
            Router::with_path("rpc-endpoints")
                .get(handler::list_rpc_endpoints)
                .post(handler::upsert_rpc_endpoint),
        )
        .push(Router::with_path("rpc-endpoints/{id}").delete(handler::delete_rpc_endpoint))
        .push(Router::with_path("gaspool/enable").post(handler::set_gas_pool_enabled))
        .push(Router::with_path("swap/providers").post(handler::upsert_swap_provider))
        .push(Router::with_path("transactions/reindex").post(handler::reindex))
        .push(
            Router::with_path("cms/app-configs")
                .get(handler::get_app_config)
                .post(handler::upsert_app_config),
        )
        .push(
            Router::with_path("cms/guides")
                .get(handler::list_guides)
                .post(handler::upsert_guide),
        )
        .push(Router::with_path("cms/guides/{id}").delete(handler::delete_guide))
        .hoop(middleware::require_admin_jwt);

    let cors = if cfg.allowed_origins.is_empty() {
        Cors::permissive()
    } else {
        use salvo::cors::AllowOrigin;
        let origins: Vec<String> = cfg.allowed_origins.clone();
        Cors::new()
            .allow_origin(AllowOrigin::list(origins.iter().filter_map(|o| o.parse().ok())))
            .allow_methods([salvo::http::Method::GET, salvo::http::Method::POST, salvo::http::Method::OPTIONS])
            .allow_headers(salvo::cors::AllowHeaders::mirror_request())
    };

    let app = Router::new()
        .push(Router::with_path("healthz").get(handler::healthz))
        .push(admin_router)
        .hoop(StateInjector(state))
        .hoop(cors.into_handler());

    let doc = OpenApi::new("Wallet Admin API", "0.1.0").merge_router(&app);
    let mut app = app.push(doc.into_router("/api-doc/openapi.json"));
    let is_prod = std::env::var("APP_ENV")
        .map(|v| v == "production" || v == "prod")
        .unwrap_or(false);
    if !is_prod {
        app = app.push(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"));
    }

    let addr: SocketAddr = cfg.listen.parse()?;
    tracing::info!("apigw-admin on {addr}");
    let listener = TcpListener::new(addr.to_string()).bind().await;
    let server = Server::new(listener);
    let handle = server.handle();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        tracing::info!("shutting down apigw-admin...");
        handle.stop_graceful(Some(std::time::Duration::from_secs(30)));
    });
    server.serve(app).await;
    Ok(())
}
