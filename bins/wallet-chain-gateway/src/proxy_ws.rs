use salvo::prelude::*;
use salvo::websocket::WebSocketUpgrade;
use std::time::Instant;

use crate::proxy;
use crate::protocol::parse_endpoint_url_with;
use crate::stats;
use crate::Gw;

#[handler]
pub async fn proxy_rpc_ws(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), StatusError> {
    let st = depot
        .get_typed::<Gw>()
        .map_err(|_| StatusError::internal_server_error())?
        .clone();

    let auth = match proxy::authenticate(req, &st).await {
        Ok(a) => a,
        Err(e) => {
            res.render(e);
            return Ok(());
        }
    };

    let cfg = st.settings.get();
    let method = req
        .queries()
        .get("method")
        .map(|s| s.as_str());

    let chain_index = auth.chain_index;
    let user_tier = auth.key_row.allowed_tier.clone();
    let api_key = auth.api_key.clone();
    let client_ip = req.remote_addr().ip().map(|ip| ip.to_string());

    let selection = match st
        .router
        .select_endpoint(
            chain_index,
            method,
            &user_tier,
            cfg.max_block_lag.max(0),
            true,
            false,
        )
        .await
    {
        Ok(s) => s,
        Err(e) => {
            res.render(e);
            return Ok(());
        }
    };
    let url = selection.url;
    let protocol = selection.protocol;
    let headers = selection.headers;

    let (protocol, _) = match parse_endpoint_url_with(&url, protocol) {
        Ok(v) => v,
        Err(e) => {
            res.render(e);
            return Ok(());
        }
    };

    if !protocol.supports_ws_tunnel() {
        res.render(wallet_error::AppError::InvalidArgument(format!(
            "endpoint {url} does not support websocket tunnel"
        )));
        return Ok(());
    }

    if cfg.log_requests {
        tracing::info!(
            api_key = %api_key,
            chain = %auth.chain_name,
            chain_index,
            endpoint = %url,
            "proxy ws tunnel"
        );
    }

    let router = st.router.clone();
    let stats = st.stats.clone();
    let endpoint_url = url.clone();
    let start = Instant::now();
    let method_owned = method.map(String::from);

    WebSocketUpgrade::new()
        .upgrade(req, res, move |ws| async move {
            stats.record(stats::StatsEvent {
                api_key: api_key.clone(),
                chain_index,
                client_ip: client_ip.clone(),
                method: method_owned.clone(),
                status_code: 101,
                latency_ms: 0,
                error_msg: None,
            });
            let result = crate::transports::tunnel(ws, &endpoint_url, Some(protocol), &headers)
                .await;
            let latency = start.elapsed().as_millis() as i32;
            match &result {
                Ok(()) => {
                    router.mark_success(&endpoint_url);
                }
                Err(e) => {
                    router.mark_failure(&endpoint_url);
                    stats.record(stats::StatsEvent {
                        api_key,
                        chain_index,
                        client_ip: client_ip.clone(),
                        method: method_owned,
                        status_code: 503,
                        latency_ms: latency,
                        error_msg: Some(e.to_string()),
                    });
                    tracing::warn!(endpoint = %endpoint_url, error = %e, "ws tunnel failed");
                }
            }
        })
        .await
}
