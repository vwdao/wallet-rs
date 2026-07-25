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
struct Gw {
    db: Db,
    http: reqwest::Client,
    /// api_key -> timestamps in the last minute
    rate: Arc<DashMap<String, Vec<Instant>>>,
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
    let state = Gw {
        db,
        http: reqwest::Client::new(),
        rate: Arc::new(DashMap::new()),
    };

    let app = Router::new()
        .push(Router::with_path("healthz").get(healthz))
        .push(Router::with_path("rpc/{chain}").post(proxy_rpc))
        .hoop(StateInjector(state));

    let addr: SocketAddr = cfg.listen.parse()?;
    tracing::info!("chain-gateway on {addr}");
    let listener = TcpListener::new(addr.to_string()).bind().await;
    Server::new(listener).serve(app).await;
    Ok(())
}

#[handler]
async fn healthz(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Text::Plain("ok"));
}

#[handler]
async fn proxy_rpc(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<Value>, AppError> = async {
        let st = depot.get_typed::<Gw>().expect("Gw state not inserted");

        let api_key = req
            .headers()
            .get("x-api-key")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let mut db = st.db.clone_inner();
        let (limit, enabled) = match wallet_db::ChainGatewayKey::filter_by_api_key(api_key)
            .get(&mut db)
            .await
        {
            Ok(r) => (r.rate_limit_per_min, r.enabled),
            Err(_) => (60, true),
        };
        if !enabled {
            return Err(AppError::Forbidden);
        }
        check_rate(&st.rate, api_key, limit as usize)?;

        let chain: String = req
            .param("chain")
            .ok_or_else(|| AppError::InvalidArgument("missing chain".into()))?;
        let body: Value = req
            .parse_json()
            .await
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;

        let chain_index: i64 = chain.parse().unwrap_or(60);
        let endpoints: Vec<wallet_db::RpcEndpoint> =
            wallet_db::RpcEndpoint::filter(
                wallet_db::RpcEndpoint::fields()
                    .chain_index()
                    .eq(chain_index)
                    .and(wallet_db::RpcEndpoint::fields().enabled().eq(true)),
            )
            .limit(1)
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        let Some(ep) = endpoints.into_iter().next() else {
            return Err(AppError::NotFound(format!("rpc for chain {chain}")));
        };
        let url = ep.url;

        let resp = st
            .http
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Unavailable(e.to_string()))?;
        let v: Value = resp
            .json()
            .await
            .map_err(|e| AppError::Unavailable(e.to_string()))?;
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
    let now = Instant::now();
    let mut entry = map.entry(key.to_string()).or_default();
    entry.retain(|t| now.duration_since(*t) < Duration::from_secs(60));
    if entry.len() >= limit {
        return Err(AppError::Unavailable("rate limit".into()));
    }
    entry.push(now);
    Ok(())
}
