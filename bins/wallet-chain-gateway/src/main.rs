use clap::Parser;
use dashmap::DashMap;
use rust_embed::RustEmbed;
use salvo::prelude::*;
use salvo::serve_static::static_embed;
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing_subscriber::EnvFilter;
use wallet_config::{load_yaml, ChainGatewayConfig};
use wallet_db::Db;
use wallet_error::AppError;

mod admin;
mod free_rpc;
mod grpc_proxy;
mod health;
mod protocol;
mod proxy;
mod proxy_ws;
mod router;
mod settings;
mod state_injector;
mod stats;
mod transports;

#[derive(RustEmbed)]
#[folder = "static/admin"]
struct AdminAssets;

use settings::SettingsHandle;
use state_injector::StateInjector;

#[derive(Clone)]
pub struct Gw {
    pub db: Db,
    pub http: reqwest::Client,
    pub router: Arc<router::RpcRouter>,
    pub stats: stats::StatsCollector,
    pub rate: Arc<DashMap<String, Vec<Instant>>>,
    pub admin_key: Option<String>,
    pub admin_username: Option<String>,
    pub admin_password: Option<String>,
    pub settings: SettingsHandle,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "configs/chain-gateway.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = rustls::crypto::ring::default_provider().install_default();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();
    let args = Args::parse();
    let cfg: ChainGatewayConfig = load_yaml(&args.config)?;
    let db = Db::connect(&cfg.database).await?;
    let http = build_http_client();

    let (reloader, settings_handle) =
        settings::SettingsReloader::new(db.clone(), Duration::from_secs(10));
    let loaded_cfg = reloader.load_now().await?;
    reloader.spawn();

    let router = router::RpcRouter::new(db.clone(), http.clone(), loaded_cfg.failure_threshold);
    let router = Arc::new(router);

    let stats_collector = stats::StatsCollector::new(
        db.clone(),
        Duration::from_millis(loaded_cfg.stats_batch_interval_ms),
    );

    let state = Gw {
        db: db.clone(),
        http: http.clone(),
        router: router.clone(),
        stats: stats_collector,
        rate: Arc::new(DashMap::new()),
        admin_key: cfg.admin_key.clone(),
        admin_username: cfg.admin_username.clone(),
        admin_password: cfg.admin_password.clone(),
        settings: settings_handle.clone(),
    };

    let health_checker = health::HealthChecker::new(
        db.clone(),
        http.clone(),
        Duration::from_millis(loaded_cfg.health_check_interval_ms),
        settings_handle.clone(),
    );
    health_checker.spawn();

    if let Some(grpc_listen) = cfg.grpc_listen.clone().filter(|s| !s.is_empty()) {
        match grpc_listen.parse::<SocketAddr>() {
            Ok(addr) => grpc_proxy::spawn(state.clone(), addr),
            Err(e) => tracing::error!("invalid grpc_listen {grpc_listen}: {e}"),
        }
    }

    {
        let syncer = free_rpc::FreeRpcSyncer::new(db.clone(), http.clone());
        let router = router.clone();
        tokio::spawn(async move {
            match syncer.sync_all_enabled().await {
                Ok(results) => {
                    let inserted: usize = results.iter().map(|r| r.inserted).sum();
                    let updated: usize = results.iter().map(|r| r.updated).sum();
                    if inserted > 0 || updated > 0 {
                        router.invalidate_all();
                    }
                    tracing::info!(
                        inserted,
                        updated,
                        chains = results.len(),
                        "free rpc bootstrap finished"
                    );
                    for result in results {
                        if let Some(message) = &result.message {
                            tracing::info!(
                                chain_index = result.chain_index,
                                family = %result.family,
                                source = %result.source,
                                inserted = result.inserted,
                                updated = result.updated,
                                discovered = result.discovered,
                                skipped = result.skipped,
                                message = %message,
                                "free rpc sync result"
                            );
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("free rpc bootstrap failed: {e}");
                }
            }
        });
    }

    let app = Router::new()
        .push(Router::with_path("healthz").get(healthz))
        .push(
            Router::new()
                .push(Router::with_path("readyz").get(readyz))
                .push(
                    Router::with_path("rpc/{chain}/{api_key}")
                        .post(proxy_rpc)
                        .get(proxy_ws::proxy_rpc_ws),
                )
                .push(
                    Router::with_path("rpc/{chain}")
                        .post(proxy_rpc)
                        .get(proxy_ws::proxy_rpc_ws),
                )
                .push(admin::admin_router())
                .push(
                    Router::with_path("admin/{*path}")
                        .get(static_embed::<AdminAssets>().fallback("index.html")),
                )
                .push(
                    Router::with_path("admin")
                        .get(static_embed::<AdminAssets>().fallback("index.html")),
                )
                .hoop(StateInjector(state)),
        );

    let addr: SocketAddr = cfg.listen.parse()?;
    tracing::info!("chain-gateway on {addr}");
    let listener = TcpListener::new(addr.to_string()).bind().await;
    let server = Server::new(listener);
    let handle = server.handle();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        tracing::info!("shutting down chain-gateway...");
        handle.stop_graceful(Some(std::time::Duration::from_secs(30)));
    });
    server.serve(app).await;
    Ok(())
}

#[handler]
async fn healthz(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Text::Plain("ok"));
}

#[handler]
async fn readyz(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result = depot.get_typed::<Gw>();
    match result {
        Ok(st) if st.settings.is_ready() => {
            res.render(Text::Plain("ok"));
        }
        _ => {
            res.status_code(StatusCode::SERVICE_UNAVAILABLE);
            res.render(Text::Plain("not ready"));
        }
    }
}

#[handler]
async fn proxy_rpc(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<Value>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("Gw state not inserted"))?;

        let cfg = st.settings.get();
        let auth = proxy::authenticate(req, st).await?;
        let api_key = auth.api_key.clone();
        let chain_index = auth.chain_index;
        let chain = auth.chain_name.clone();
        let mut key_row = auth.key_row;

        let body: Value = req
            .parse_json()
            .await
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;

        let method = body
            .get("method")
            .and_then(|v| v.as_str())
            .map(String::from);
        let params = body.get("params").cloned().unwrap_or(Value::Null);
        let id = body.get("id").cloned().unwrap_or(Value::Null);

        let next_total_requests = key_row.total_requests.saturating_add(1);
        let mut counter_db = st.db.clone_inner();
        let _ = key_row
            .update()
            .total_requests(next_total_requests)
            .exec(&mut counter_db)
            .await;

        let start = Instant::now();
        let timeout = Duration::from_secs(cfg.rpc_timeout_secs);
        let rpc_result = st
            .router
            .execute_rpc(
                chain_index,
                &body,
                &key_row.allowed_tier,
                timeout,
                cfg.max_retries,
                cfg.max_block_lag,
            )
            .await;
        let latency = start.elapsed().as_millis() as i32;

        let client_ip = client_ip_of(req);

        let (upstream_url, status_code, error_msg, rpc_result) = match rpc_result {
            Ok((url, v)) => (Some(url), 200, None, Ok(v)),
            Err(e) => (None, 503, Some(e.to_string()), Err(e)),
        };

        st.stats.record(stats::StatsEvent {
            api_key: api_key.clone(),
            chain_index,
            client_ip: client_ip.clone(),
            method: method.clone(),
            status_code,
            latency_ms: latency,
            error_msg,
        });

        if cfg.log_requests {
            let log = serde_json::json!({
                "type": "proxy_request",
                "method": method,
                "params": params,
                "id": id,
                "api_key": api_key,
                "chain": chain,
                "chain_index": chain_index,
                "upstream_url": upstream_url,
                "status_code": status_code,
                "latency_ms": latency,
                "client_ip": client_ip,
            });
            tracing::info!(log = %log, "proxy request");
        }

        let v = rpc_result?;
        Ok(Json(v))
    }
    .await;
    match result {
        Ok(j) => res.render(j),
        Err(e) => res.render(e),
    }
}

fn build_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        // RPC calls must reach endpoints directly; honoring HTTP_PROXY here
        // can stall requests for rpc_timeout_secs * max_retries.
        .no_proxy()
        .build()
        .expect("failed to build HTTP client")
}

fn client_ip_of(req: &Request) -> Option<String> {
    req.remote_addr().ip().map(|ip| ip.to_string())
}

