use std::time::Duration;
use wallet_db::RpcEndpoint;
use wallet_error::AppResult;

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

struct ProbeResult {
    latency_ms: i32,
    block_height: Option<i64>,
}

pub struct HealthChecker {
    db: wallet_db::Db,
    http: reqwest::Client,
    interval: Duration,
    failure_threshold: u32,
}

impl HealthChecker {
    pub fn new(
        db: wallet_db::Db,
        http: reqwest::Client,
        interval: Duration,
        failure_threshold: u32,
    ) -> Self {
        Self {
            db,
            http,
            interval,
            failure_threshold,
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
            let result = self.probe(&ep.url, probe_method, family).await;

            let mut db = self.db.clone_inner();
            match result {
                Ok(probe) => {
                    let new_avg = match ep.avg_latency_ms {
                        Some(prev) => (prev + probe.latency_ms) / 2,
                        None => probe.latency_ms,
                    };
                    ep.update()
                        .healthy(true)
                        .avg_latency_ms(Some(new_avg))
                        .error_count(0)
                        .block_height(probe.block_height)
                        .last_health_check(Some(jiff::Timestamp::now()))
                        .exec(&mut db)
                        .await
                        .map_err(|e| wallet_error::AppError::internal(e.to_string()))?;

                    if let Some(h) = probe.block_height {
                        chain_heights.entry(ep.chain_index).or_default().push(h);
                    }
                }
                Err(_) => {
                    let new_err_count = ep.error_count + 1;
                    let healthy = (new_err_count as u32) < self.failure_threshold;
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
                    if lag > 10 {
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

    async fn probe(&self, url: &str, method: &str, family: &str) -> AppResult<ProbeResult> {
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": [],
        });
        let start = std::time::Instant::now();
        let resp = self
            .http
            .post(url)
            .json(&body)
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .map_err(|e| wallet_error::AppError::Unavailable(e.to_string()))?;
        let latency = start.elapsed().as_millis() as i32;
        if !resp.status().is_success() {
            return Err(wallet_error::AppError::Unavailable(format!(
                "probe status {}",
                resp.status()
            )));
        }
        let v: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| wallet_error::AppError::Unavailable(e.to_string()))?;
        if v.get("error").is_some() {
            return Err(wallet_error::AppError::Unavailable(
                "rpc probe returned error".into(),
            ));
        }

        let result = v.get("result").cloned().unwrap_or(serde_json::Value::Null);
        let block_height = self.parse_block_height(&result, family);

        Ok(ProbeResult {
            latency_ms: latency,
            block_height,
        })
    }

    fn parse_block_height(&self, result: &serde_json::Value, family: &str) -> Option<i64> {
        match family {
            "evm" => {
                let hex = result.as_str()?.trim_start_matches("0x");
                i64::from_str_radix(hex, 16).ok()
            }
            "bitcoin" => result.as_i64(),
            "solana" => result
                .as_i64()
                .or_else(|| result.get("value").and_then(|v| v.as_i64())),
            "tron" => result
                .get("block_header")
                .and_then(|h| h.get("raw_data"))
                .and_then(|r| r.get("number"))
                .and_then(|n| n.as_i64())
                .or_else(|| result.as_i64()),
            _ => {
                let hex = result.as_str()?.trim_start_matches("0x");
                i64::from_str_radix(hex, 16).ok()
            }
        }
    }
}
