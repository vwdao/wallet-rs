use clap::Parser;
use salvo::prelude::*;
use serde::Deserialize;
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use wallet_config::{load_yaml, WebhookConfig};
use wallet_error::AppError;
use wallet_events::EventBus;

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

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "configs/apigw-webhook.yaml")]
    config: String,
}

#[derive(Deserialize)]
struct WebhookEvent {
    source: String,
    event_type: String,
    payload: Value,
}

#[derive(Clone)]
struct WebhookState {
    events: Arc<dyn EventBus>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();
    let args = Args::parse();
    let cfg: WebhookConfig = load_yaml(&args.config)?;

    let events = match wallet_events::NatsEventBus::connect(&cfg.nats.url).await {
        Ok(b) => b as Arc<dyn EventBus>,
        Err(_) => wallet_events::MemoryEventBus::new(256),
    };
    let state = WebhookState { events };

    let app = Router::new()
        .push(Router::with_path("healthz").get(healthz))
        .push(Router::with_path("webhooks/chain").post(ingest_chain))
        .push(Router::with_path("webhooks/dex").post(ingest_dex))
        .push(Router::with_path("webhooks/{source}").post(ingest_generic))
        .hoop(StateInjector(state));

    let addr: SocketAddr = cfg.listen.parse()?;
    tracing::info!("apigw-webhook on {addr}");
    let listener = TcpListener::new(addr.to_string()).bind().await;
    Server::new(listener).serve(app).await;
    Ok(())
}

#[handler]
async fn healthz(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Text::Plain("ok"));
}

#[handler]
async fn ingest_chain(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<Value>, AppError> = async {
        let st = depot
            .get_typed::<WebhookState>()
            .expect("WebhookState not inserted");
        let ev: WebhookEvent = req
            .parse_json()
            .await
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;
        tracing::info!(source = %ev.source, event = %ev.event_type, "chain webhook received");
        let subject = format!("webhook.{}.{}", ev.source, ev.event_type);
        st.events.publish(&subject, ev.payload).await?;
        Ok(Json(json!({ "accepted": true })))
    }
    .await;
    match result {
        Ok(j) => res.render(j),
        Err(e) => res.render(e),
    }
}

#[handler]
async fn ingest_dex(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<Value>, AppError> = async {
        let st = depot
            .get_typed::<WebhookState>()
            .expect("WebhookState not inserted");
        let ev: WebhookEvent = req
            .parse_json()
            .await
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;
        tracing::info!(source = %ev.source, event = %ev.event_type, "dex webhook received");
        let subject = format!("webhook.{}.{}", ev.source, ev.event_type);
        st.events.publish(&subject, ev.payload).await?;
        Ok(Json(json!({ "accepted": true })))
    }
    .await;
    match result {
        Ok(j) => res.render(j),
        Err(e) => res.render(e),
    }
}

#[handler]
async fn ingest_generic(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result: Result<Json<Value>, AppError> = async {
        let st = depot
            .get_typed::<WebhookState>()
            .expect("WebhookState not inserted");
        let source: String = req
            .param("source")
            .ok_or_else(|| AppError::InvalidArgument("missing source".into()))?;
        let ev: WebhookEvent = req
            .parse_json()
            .await
            .map_err(|e| AppError::InvalidArgument(e.to_string()))?;
        tracing::info!(source = %source, event = %ev.event_type, "generic webhook received");
        let subject = format!("webhook.{}.{}", source, ev.event_type);
        st.events.publish(&subject, ev.payload).await?;
        Ok(Json(json!({ "accepted": true })))
    }
    .await;
    match result {
        Ok(j) => res.render(j),
        Err(e) => res.render(e),
    }
}
