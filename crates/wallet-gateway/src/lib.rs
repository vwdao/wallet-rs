//! Shared salvo (HTTP) gateway building blocks used by the edge binaries:
//! `StateInjector`, CORS construction and the bind/serve/shutdown bootstrap.

pub mod dto;

use salvo::cors::Cors;
use salvo::prelude::*;
use std::net::SocketAddr;
use std::time::Duration;

/// Injects an arbitrary cloned state value into every request depot so that
/// handlers can retrieve it via `depot.try_obtain::<T>()`.
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

/// Builds a CORS handler. An empty origin list means permissive (dev default).
pub fn cors_handler(origins: &[String]) -> salvo::cors::CorsHandler {
    if origins.is_empty() {
        Cors::permissive().into_handler()
    } else {
        use salvo::cors::AllowOrigin;
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
            .into_handler()
    }
}

/// Binds the listener, serves `app`, and shuts down gracefully on SIGINT.
/// Mirrors the bootstrap every gateway binary previously duplicated.
pub async fn serve(
    app: Router,
    addr: &str,
    name: &str,
    shutdown: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = addr.parse()?;
    tracing::info!("{name} on {addr}");
    let listener = TcpListener::new(addr.to_string()).bind().await;
    let server = Server::new(listener);
    let handle = server.handle();
    let name = name.to_string();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        tracing::info!("shutting down {name}...");
        handle.stop_graceful(Some(shutdown));
    });
    server.serve(app).await;
    Ok(())
}
