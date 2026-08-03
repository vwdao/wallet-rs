use std::time::Duration;
use wallet_db::RpcEndpoint;
use wallet_error::AppResult;

use crate::settings::SettingsHandle;
use crate::transports;

const ARCHIVE_METHODS: &[&str] = &[
    "debug_traceBlockByNumber",
    "debug_traceBlockByHash",
    "debug_traceTransaction",
    "trace_block",
    "trace_transaction",
    "trace_call",
    "eth_getLogs",
    "debug_getRawReceipts",
];

pub fn is_archive_method(method: &str) -> bool {
    ARCHIVE_METHODS
        .iter()
        .any(|m| m.eq_ignore_ascii_case(method))
}

pub fn probe_method_for_chain(family: &str) -> &str {
    match family {
        "evm" => "eth_blockNumber",
        "solana" => "getSlot",
        "bitcoin" => "getblockcount",
        "tron" => "getnowblock",
        _ => "eth_blockNumber",
    }
}

pub struct HealthChecker {
    db: wallet_db::Db,
    http: reqwest::Client,
    interval: Duration,
    settings: SettingsHandle,
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
        }
    }

    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                if let Err(e) = self.check_all().await {
                    tracing::error!("health check cycle failed: {e}");
                }
                tokio::time::sleep(self.interval).await;
            }
        })
    }

    async fn check_all(&self) -> AppResult<()> {
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

        for ep in endpoints.iter_mut() {
            let family = family_map
                .get(&ep.chain_index)
                .map(|s| s.as_str())
                .unwrap_or("evm");
            let probe_method = probe_method_for_chain(family);
            let protocol = crate::protocol::EndpointProtocol::from_config_opt(&ep.protocol);
            let headers = serde_json::from_value(ep.headers.clone()).unwrap_or_default();
            let result =
                transports::probe(&self.http, &ep.url, protocol, &headers, probe_method, family)
                    .await;

            let mut db = self.db.clone_inner();
            match result {
                Ok((latency, block_height)) => {
                    let new_avg = match ep.avg_latency_ms {
                        Some(prev) => (prev + latency) / 2,
                        None => latency,
                    };
                    ep.update()
                        .healthy(true)
                        .avg_latency_ms(Some(new_avg))
                        .error_count(0)
                        .block_height(block_height)
                        .last_health_check(Some(jiff::Timestamp::now()))
                        .exec(&mut db)
                        .await
                        .map_err(|e| wallet_error::AppError::internal(e.to_string()))?;

                    if let Some(h) = block_height {
                        chain_heights.entry(ep.chain_index).or_default().push(h);
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
            tracing::info!(
                chain = chain_index,
                max_height,
                endpoints = heights.len(),
                "block height consensus"
            );

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
                if let Some(h) = ep.block_height {
                    let lag = max_height.saturating_sub(h);
                    if lag > cfg.max_block_lag.max(0) {
                        tracing::warn!(
                            endpoint = %ep.url,
                            block_height = h,
                            max_height,
                            lag,
                            "endpoint significantly behind, marking unhealthy"
                        );
                        ep.update()
                            .healthy(false)
                            .exec(&mut db)
                            .await
                            .map_err(|e| wallet_error::AppError::internal(e.to_string()))?;
                    }
                }
            }
        }

        Ok(())
    }
}
