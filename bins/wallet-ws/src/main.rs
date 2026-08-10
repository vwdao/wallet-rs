mod handler;
mod hub;
mod subscriber;

use clap::Parser;
use salvo::prelude::*;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use wallet_config::{load_yaml, WsConfig};
use wallet_events::{MemoryEventBus, NatsEventBus};
use wallet_gateway::{serve, StateInjector};

use crate::hub::Hub;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "configs/wallet-ws.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();
    let args = Args::parse();
    let cfg: WsConfig = load_yaml(&args.config)?;
    let hub = Arc::new(Hub::default());
    let events = match NatsEventBus::connect(&cfg.nats.url).await {
        Ok(b) => b as Arc<dyn wallet_events::EventBus>,
        Err(_) => MemoryEventBus::new(256),
    };
    tokio::spawn(subscriber::fanout(events, hub.clone()));

    let app = Router::new()
        .push(Router::with_path("healthz").get(healthz))
        .push(Router::with_path("ws").get(handler::ws_upgrade))
        .hoop(StateInjector(hub));

    serve(app, &cfg.listen, "wallet-ws", std::time::Duration::from_secs(10)).await
}

#[handler]
async fn healthz(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Text::Plain("ok"));
}
