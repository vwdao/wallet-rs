use std::time::Duration;
use wallet_db::RpcEndpoint;
use wallet_error::AppResult;

use crate::protocol::EndpointProtocol;
use crate::settings::SettingsHandle;
use crate::transports;

const ARCHIVE_METHODS: &[&str] = &[
    "trace_block",
    "trace_transaction",
    "trace_call",
    "trace_callMany",
    "trace_rawTransaction",
    "trace_replayBlockTransactions",
    "trace_replayTransaction",
    "trace_filter",
    "trace_get",
    "debug_traceBlockByNumber",
    "debug_traceBlockByHash",
    "debug_traceTransaction",
    "debug_traceCall",
    "debug_storageRangeAt",
    "debug_getRawReceipts",
];

pub fn is_archive_method(method: &str) -> bool {
    ARCHIVE_METHODS
        .iter()
        .any(|m| m.eq_ignore_ascii_case(method))
}

const ARCHIVE_PROBE_ADDRESS: &str = "0x0000000000000000000000000000000000000000";
const ARCHIVE_PROBE_BLOCK: &str = "0x1";
const ARCHIVE_REPROBE_INTERVAL_HOURS: i64 = 6;

/// Consecutive health cycles an endpoint must be observed behind before the
/// lag rule marks it unhealthy. Public RPCs on fast chains (Arbitrum, Base,
/// OP) routinely trail the reference node by a handful of blocks for a few
/// seconds; a single observation would flap the endpoint in and out of
/// rotation every cycle.
const LAG_UNHEALTHY_CONSECUTIVE: u32 = 3;

async fn probe_archive_capability(
    http: &reqwest::Client,
    url: &str,
    protocol: Option<crate::protocol::EndpointProtocol>,
    headers: &crate::transports::EndpointHeaders,
) -> bool {
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_getBalance",
        "params": [ARCHIVE_PROBE_ADDRESS, ARCHIVE_PROBE_BLOCK],
    });
    match transports::execute_unary(http, url, &body, protocol, headers, Duration::from_secs(5)).await
    {
        Ok(v) => v.get("result").is_some(),
        Err(_) => false,
    }
}

pub fn probe_method_for_chain(family: &str) -> &str {
    match family {
        "evm" => "eth_blockNumber",
        "solana" => "getSlot",
        "bitcoin" => "getblockcount",
        "tron" => "getnowblock",
        // TON (The Open Network): masterchain head seqno.
        "ton" => "getMasterchainInfo",
        // Sui: latest checkpoint sequence number as the chain "height" (a
        // BigInt serialized as a decimal string). We use the legacy `sui_`
        // alias — public fullnodes (publicnode, zan.top, blockpi) reject the
        // modern `suix_` spelling of this method. The reference gas price
        // would also answer, but it is small and non-monotonic, i.e. useless
        // for lag-based health routing.
        "sui" => "sui_getLatestCheckpointSequenceNumber",
        _ => "eth_blockNumber",
    }
}

pub fn probe_params_for_chain(family: &str) -> serde_json::Value {
    match family {
        "solana" => serde_json::json!([{ "commitment": "finalized" }]),
        "ton" => serde_json::json!({}),
        "sui" => serde_json::json!([]),
        _ => serde_json::json!([]),
    }
}

pub struct HealthChecker {
    db: wallet_db::Db,
    http: reqwest::Client,
    interval: Duration,
    settings: SettingsHandle,
    consecutive_lag: std::collections::HashMap<String, u32>,
}

impl HealthChecker {
    pub fn new(
        db: wallet_db::Db,
        http: reqwest::Client,
        interval: Duration,
        settings: SettingsHandle,
    ) -> Self {
        Self {
            db,
            http,
            interval,
            settings,
            consecutive_lag: std::collections::HashMap::new(),
        }
    }

    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut this = self;
            loop {
                if let Err(e) = this.check_all().await {
                    tracing::error!("health check cycle failed: {e}");
                }
                tokio::time::sleep(this.interval).await;
            }
        })
    }

    async fn check_all(&mut self) -> AppResult<()> {
        let cfg = self.settings.get();
        let mut db = self.db.clone_inner();
        let mut endpoints: Vec<RpcEndpoint> =
            RpcEndpoint::filter(RpcEndpoint::fields().enabled().eq(true))
                .exec(&mut db)
                .await
                .map_err(|e| wallet_error::AppError::internal(e.to_string()))?;

        let networks: Vec<wallet_db::Network> =
            wallet_db::Network::filter(wallet_db::Network::fields().enabled().eq(true))
                .exec(&mut db)
                .await
                .map_err(|e| wallet_error::AppError::internal(e.to_string()))?;

        let family_map: std::collections::HashMap<i64, String> = networks
            .into_iter()
            .map(|n| (n.chain_index, n.family))
            .collect();

        let mut chain_heights: std::collections::HashMap<i64, Vec<i64>> =
            std::collections::HashMap::new();
        let mut seen_healthy: std::collections::HashSet<String> =
            std::collections::HashSet::new();

        for ep in endpoints.iter_mut() {
            let family = family_map
                .get(&ep.chain_index)
                .map(|s| s.as_str())
                .unwrap_or("evm");
            let probe_method = probe_method_for_chain(family);
            let protocol = crate::protocol::EndpointProtocol::from_config_opt(&ep.protocol);
            // The effective protocol is what `transports::probe` will actually
            // use: the config override when set, otherwise the URL scheme
            // (e.g. `grpcs://` must resolve to Grpc even when the endpoint row
            // has no protocol column). The gRPC exception below relies on it.
            let effective_protocol = crate::protocol::parse_endpoint_url_with(&ep.url, protocol)
                .map(|(p, _)| p)
                .unwrap_or(protocol.unwrap_or(EndpointProtocol::Http));
            let headers = serde_json::from_value(ep.headers.clone()).unwrap_or_default();
            let result =
                transports::probe(&self.http, &ep.url, protocol, &headers, probe_method, family)
                    .await;

            // A healthy endpoint must report a usable block height. A probe that
            // "succeeds" with a `null`/unparseable result means the endpoint is
            // not actually serving the chain (e.g. a lagging/broken node that
            // answers `eth_blockNumber` with `null`); treat it as a probe
            // failure so it accumulates error_count and is eventually marked
            // unhealthy instead of staying in rotation with a cleared height.
            // gRPC is the exception: its probe is connectivity-only.
            let result = match result {
                Ok((_latency, None)) if effective_protocol != EndpointProtocol::Grpc => {
                    tracing::warn!(
                        endpoint = %ep.url,
                        chain = ep.chain_index,
                        "probe returned no block height, counting as failure"
                    );
                    Err(wallet_error::AppError::Unavailable(
                        "probe returned no block height".into(),
                    ))
                }
                other => other,
            };

            let mut db = self.db.clone_inner();
            match result {
                Ok((latency, block_height)) => {
                    let new_avg = match ep.avg_latency_ms {
                        Some(prev) => (prev + latency) / 2,
                        None => latency,
                    };
                    let endpoint_url = ep.url.clone();
                    let chain_index = ep.chain_index;
                    let previous_archive = ep.is_archive;
                    let archive_checked_at = ep.archive_checked_at;
                    let mut upd = ep
                        .update()
                        .healthy(true)
                        .avg_latency_ms(Some(new_avg))
                        .error_count(0)
                        .block_height(block_height)
                        .last_health_check(Some(jiff::Timestamp::now()));

                    if family == "evm" {
                        let stale = archive_checked_at.is_none_or(|t| {
                            t.checked_add(jiff::Span::new().hours(ARCHIVE_REPROBE_INTERVAL_HOURS))
                                .map_or(true, |deadline| jiff::Timestamp::now() >= deadline)
                        });
                        if stale {
                            let is_archive = probe_archive_capability(
                                &self.http,
                                &endpoint_url,
                                protocol,
                                &headers,
                            )
                            .await;
                            upd = upd
                                .is_archive(is_archive)
                                .archive_checked_at(Some(jiff::Timestamp::now()));
                            if is_archive != previous_archive {
                                tracing::info!(
                                    endpoint = %endpoint_url,
                                    chain = chain_index,
                                    is_archive,
                                    "evm archive capability detected"
                                );
                            }
                        }
                    }

                    upd.exec(&mut db)
                        .await
                        .map_err(|e| wallet_error::AppError::internal(e.to_string()))?;

                    if let Some(h) = block_height {
                        chain_heights.entry(chain_index).or_default().push(h);
                    }
                }
                Err(_) => {
                    let new_err_count = ep.error_count + 1;
                    let healthy = (new_err_count as u32) < cfg.failure_threshold.max(1);
                    ep.update()
                        .healthy(healthy)
                        .error_count(new_err_count)
                        .last_health_check(Some(jiff::Timestamp::now()))
                        .exec(&mut db)
                        .await
                        .map_err(|e| wallet_error::AppError::internal(e.to_string()))?;
                    if !healthy {
                        if family == "ton" && !ep.url.to_ascii_lowercase().contains("/jsonrpc") {
                            tracing::warn!(
                                endpoint = %ep.url,
                                errors = new_err_count,
                                "TON endpoint is not a toncenter v2 JSON-RPC URL (path must contain /jsonRPC); toncenter v3 is a REST API and will never pass the probe"
                            );
                        }
                        tracing::warn!(
                            endpoint = %ep.url,
                            errors = new_err_count,
                            "rpc endpoint marked unhealthy"
                        );
                    }
                }
            }
        }

        for (chain_index, heights) in &chain_heights {
            if heights.is_empty() {
                continue;
            }
            let max_height = heights.iter().copied().max().unwrap_or(0);
            let family = family_map
                .get(chain_index)
                .map(|s| s.as_str())
                .unwrap_or("evm");
            let max_block_lag = cfg.max_block_lag_for_family(family);
            // tracing::info!(
            //     chain = chain_index,
            //     max_height,
            //     endpoints = heights.len(),
            //     "block height consensus"
            // );

            let mut db = self.db.clone_inner();
            let eps: Vec<RpcEndpoint> = RpcEndpoint::filter(
                RpcEndpoint::fields()
                    .chain_index()
                    .eq(*chain_index)
                    .and(RpcEndpoint::fields().enabled().eq(true))
                    .and(RpcEndpoint::fields().healthy().eq(true)),
            )
            .exec(&mut db)
            .await
            .map_err(|e| wallet_error::AppError::internal(e.to_string()))?;

            for mut ep in eps {
                seen_healthy.insert(ep.url.clone());
                if let Some(h) = ep.block_height {
                    let lag = max_height.saturating_sub(h);
                    if lag > max_block_lag {
                        let count =
                            self.consecutive_lag.get(&ep.url).copied().unwrap_or(0) + 1;
                        self.consecutive_lag.insert(ep.url.clone(), count);
                        if count >= LAG_UNHEALTHY_CONSECUTIVE {
                            tracing::warn!(
                                endpoint = %ep.url,
                                block_height = h,
                                max_height,
                                lag,
                                max_block_lag,
                                consecutive = count,
                                family,
                                "endpoint significantly behind, marking unhealthy"
                            );
                            ep.update()
                                .healthy(false)
                                .exec(&mut db)
                                .await
                                .map_err(|e| wallet_error::AppError::internal(e.to_string()))?;
                        } else {
                            tracing::warn!(
                                endpoint = %ep.url,
                                block_height = h,
                                max_height,
                                lag,
                                max_block_lag,
                                consecutive = count,
                                family,
                                "endpoint lagging, not yet marked unhealthy"
                            );
                        }
                    } else {
                        self.consecutive_lag.remove(&ep.url);
                    }
                }
            }
        }

        // Keep lag counters only for endpoints still in the healthy set this
        // cycle, so a recovered endpoint restarts from a clean slate.
        self.consecutive_lag
            .retain(|url, _| seen_healthy.contains(url));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{is_archive_method, probe_params_for_chain, ARCHIVE_METHODS};
    use serde_json::json;

    #[test]
    fn solana_probe_params_request_finalized_commitment() {
        assert_eq!(
            probe_params_for_chain("solana"),
            json!([{ "commitment": "finalized" }])
        );
    }

    #[test]
    fn non_solana_probe_params_are_empty() {
        for family in ["evm", "bitcoin", "tron", "unknown"] {
            assert_eq!(probe_params_for_chain(family), json!([]), "family={family}");
        }
    }

    #[test]
    fn archive_methods_are_case_insensitive() {
        assert!(is_archive_method("trace_block"));
        assert!(is_archive_method("TRACE_TRANSACTION"));
        assert!(is_archive_method("debug_traceTransaction"));
        assert!(is_archive_method("debug_traceBlockByNumber"));
        assert!(is_archive_method("trace_replayBlockTransactions"));
    }

    #[test]
    fn non_archive_methods_are_not_matched() {
        assert!(!is_archive_method("eth_blockNumber"));
        assert!(!is_archive_method("eth_chainId"));
        assert!(!is_archive_method("net_version"));
        assert!(!is_archive_method("eth_getBalance"));
        assert!(!is_archive_method("eth_call"));
        assert!(!is_archive_method("eth_getLogs"));
        assert!(!is_archive_method(""));
    }

    #[test]
    fn archive_method_list_contains_trace_and_debug_methods() {
        for method in [
            "trace_block",
            "trace_transaction",
            "trace_call",
            "trace_replayTransaction",
            "trace_filter",
            "debug_traceBlockByNumber",
            "debug_traceTransaction",
            "debug_traceCall",
            "debug_getRawReceipts",
        ] {
            assert!(
                ARCHIVE_METHODS.iter().any(|m| m.eq_ignore_ascii_case(method)),
                "expected {method} to be listed as an archive method"
            );
        }
    }

    #[test]
    fn non_archive_methods_are_not_listed() {
        for method in ["eth_getBalance", "eth_call", "eth_getLogs", "eth_getStorageAt"] {
            assert!(
                !ARCHIVE_METHODS.iter().any(|m| m.eq_ignore_ascii_case(method)),
                "{method} is not archive-only and should not be listed"
            );
        }
    }
}
