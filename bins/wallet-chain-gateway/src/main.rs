use clap::Parser;
use dashmap::DashMap;
use salvo::prelude::*;
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing_subscriber::EnvFilter;
use wallet_config::{load_yaml, ChainGatewayConfig};
use wallet_db::Db;
use wallet_error::AppError;

mod admin;
mod health;
mod router;
mod settings;
mod state_injector;
mod stats;

const ADMIN_HTML: &str = include_str!("../static/admin.html");

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
    pub settings: SettingsHandle,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "configs/chain-gateway.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();
    let args = Args::parse();
    let cfg: ChainGatewayConfig = load_yaml(&args.config)?;
    let db = Db::connect(&cfg.database).await?;
    let http = reqwest::Client::new();

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
        settings: settings_handle.clone(),
    };

    let health_checker = health::HealthChecker::new(
        db.clone(),
        http.clone(),
        Duration::from_millis(loaded_cfg.health_check_interval_ms),
        loaded_cfg.failure_threshold,
    );
    health_checker.spawn();

    let app = Router::new()
        .push(Router::with_path("healthz").get(healthz))
        .push(Router::with_path("readyz").get(readyz))
        .push(Router::with_path("rpc/{chain}").post(proxy_rpc))
        .push(Router::with_path("admin").get(admin_ui))
        .push(admin::admin_router())
        .hoop(StateInjector(state));

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
async fn admin_ui(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let _ = res.add_header("content-type", "text/html; charset=utf-8", true);
    res.render(ADMIN_HTML);
}

#[handler]
async fn proxy_rpc(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<Value>, AppError> = async {
        let st = depot
            .get_typed::<Gw>()
            .map_err(|_| AppError::internal("Gw state not inserted"))?;

        let cfg = st.settings.get();

        let api_key = req
            .headers()
            .get("x-api-key")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?
            .to_string();

        if cfg.global_rate_limit_per_min > 0 {
            check_rate(&st.rate, "_global", cfg.global_rate_limit_per_min)?;
        }

        let mut db = st.db.clone_inner();
        let mut key_row = wallet_db::ChainGatewayKey::filter_by_api_key(&api_key)
            .get(&mut db)
            .await
            .map_err(|_| AppError::Forbidden)?;

        if !key_row.enabled {
            return Err(AppError::Forbidden);
        }

        check_rate(
            &st.rate,
            &api_key,
            key_row.rate_limit_per_min.max(0) as usize,
        )?;

        let chain: String = req
            .param("chain")
            .ok_or_else(|| AppError::InvalidArgument("missing chain".into()))?;

        let chain_index: i64 = chain
            .parse()
            .map_err(|_| AppError::InvalidArgument(format!("invalid chain index: {chain}")))?;

        if !key_row.allowed_chains.is_empty() && !key_row.allowed_chains.contains(&chain_index) {
            return Err(AppError::Forbidden);
        }

        let body: Value = req
            .parse_json()
            .await
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;

        if cfg.log_requests {
            tracing::info!(
                api_key = %api_key,
                chain = chain_index,
                method = body.get("method").and_then(|v| v.as_str()).unwrap_or("unknown"),
                "proxy request"
            );
        }

        // Keep the aggregate counter best-effort so a database hiccup does not
        // turn an otherwise successful RPC request into a gateway error.
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

        let method = body
            .get("method")
            .and_then(|v| v.as_str())
            .map(String::from);

        match &rpc_result {
            Ok(_) => {
                st.stats.record(stats::StatsEvent {
                    api_key: api_key.clone(),
                    chain_index,
                    method,
                    status_code: 200,
                    latency_ms: latency,
                    error_msg: None,
                });
            }
            Err(e) => {
                st.stats.record(stats::StatsEvent {
                    api_key: api_key.clone(),
                    chain_index,
                    method,
                    status_code: 503,
                    latency_ms: latency,
                    error_msg: Some(e.to_string()),
                });
            }
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

fn check_rate(
    map: &DashMap<String, Vec<Instant>>,
    key: &str,
    limit: usize,
) -> Result<(), AppError> {
    if limit == 0 {
        return Ok(());
    }
    let now = Instant::now();
    let mut entry = map.entry(key.to_string()).or_default();
    entry.retain(|t| now.duration_since(*t) < Duration::from_secs(60));
    if entry.len() >= limit {
        return Err(AppError::TooManyRequests);
    }
    entry.push(now);
    if map.len() > 1000 {
        map.retain(|_, v| {
            !v.is_empty()
                && v.iter()
                    .any(|t| now.duration_since(*t) < Duration::from_secs(120))
        });
    }
    Ok(())
}
