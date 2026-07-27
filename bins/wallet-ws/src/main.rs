mod handler;
mod hub;
mod subscriber;

use clap::Parser;
use salvo::prelude::*;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use wallet_config::{load_yaml, WsConfig};
use wallet_events::{MemoryEventBus, NatsEventBus};

use crate::hub::Hub;

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

    let addr: SocketAddr = cfg.listen.parse()?;
    tracing::info!("wallet-ws on {addr}");
    let listener = TcpListener::new(addr.to_string()).bind().await;
    let server = Server::new(listener);
    let handle = server.handle();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        tracing::info!("shutting down wallet-ws...");
        handle.stop_graceful(Some(std::time::Duration::from_secs(10)));
    });
    server.serve(app).await;
    Ok(())
}

#[handler]
async fn healthz(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Text::Plain("ok"));
}
