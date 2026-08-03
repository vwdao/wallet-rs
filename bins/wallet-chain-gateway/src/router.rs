use dashmap::DashMap;
use rand::Rng;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use wallet_db::RpcEndpoint;
use wallet_error::{AppError, AppResult};

use crate::health::is_archive_method;
use crate::protocol::{parse_endpoint_url_with, EndpointProtocol};
use crate::transports::{self, EndpointHeaders};

#[derive(Debug, Clone)]
struct CachedEndpoint {
    url: String,
    protocol: Option<EndpointProtocol>,
    tier: String,
    is_archive: bool,
    priority: i32,
    weight: i32,
    avg_latency_ms: i32,
    block_height: Option<i64>,
    headers: EndpointHeaders,
}

#[derive(Debug, Clone)]
pub struct SelectedEndpoint {
    pub url: String,
    pub protocol: Option<EndpointProtocol>,
    pub headers: EndpointHeaders,
}

#[derive(Debug)]
struct ChainEndpoints {
    endpoints: Vec<CachedEndpoint>,
    max_block_height: Option<i64>,
    refreshed_at: Instant,
}

#[derive(Debug)]
struct EndpointCircuit {
    failures: AtomicUsize,
    open_until: parking_lot::Mutex<Option<Instant>>,
    failure_threshold: u32,
    cool_down: Duration,
}

impl EndpointCircuit {
    fn new(failure_threshold: u32, cool_down: Duration) -> Self {
        Self {
            failures: AtomicUsize::new(0),
            open_until: parking_lot::Mutex::new(None),
            failure_threshold,
            cool_down,
        }
    }

    fn allow(&self) -> bool {
        let guard = self.open_until.lock();
        if let Some(until) = *guard {
            if Instant::now() < until {
                return false;
            }
        }
        true
    }

    fn on_success(&self) {
        self.failures.store(0, Ordering::Relaxed);
        *self.open_until.lock() = None;
    }

    fn on_failure(&self) {
        let count = self.failures.fetch_add(1, Ordering::Relaxed) + 1;
        if count >= self.failure_threshold as usize {
            *self.open_until.lock() = Some(Instant::now() + self.cool_down);
        }
    }
}

pub struct RpcRouter {
    db: wallet_db::Db,
    http: reqwest::Client,
    cache: DashMap<i64, ChainEndpoints>,
    circuits: DashMap<String, Arc<EndpointCircuit>>,
    failure_threshold: u32,
    cache_ttl: Duration,
}

impl RpcRouter {
    pub fn new(db: wallet_db::Db, http: reqwest::Client, failure_threshold: u32) -> Self {
        Self {
            db,
            http,
            cache: DashMap::new(),
            circuits: DashMap::new(),
            failure_threshold,
            cache_ttl: Duration::from_secs(15),
        }
    }

    async fn get_endpoints(&self, chain_index: i64) -> AppResult<Vec<CachedEndpoint>> {
        if let Some(entry) = self.cache.get(&chain_index) {
            if entry.refreshed_at.elapsed() < self.cache_ttl {
                return Ok(entry.endpoints.clone());
            }
        }
        self.refresh_endpoints(chain_index).await
    }

    async fn refresh_endpoints(&self, chain_index: i64) -> AppResult<Vec<CachedEndpoint>> {
        let mut db = self.db.clone_inner();
        let rows: Vec<RpcEndpoint> = RpcEndpoint::filter(
            RpcEndpoint::fields()
                .chain_index()
                .eq(chain_index)
                .and(RpcEndpoint::fields().enabled().eq(true))
                .and(RpcEndpoint::fields().healthy().eq(true)),
        )
        .exec(&mut db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

        let eps: Vec<CachedEndpoint> = rows
            .into_iter()
            .map(|r| CachedEndpoint {
                url: r.url,
                protocol: EndpointProtocol::from_config_opt(&r.protocol),
                tier: r.tier,
                is_archive: r.is_archive,
                priority: r.priority,
                weight: r.weight.max(1),
                avg_latency_ms: r.avg_latency_ms.unwrap_or(5000),
                block_height: r.block_height,
                headers: serde_json::from_value(r.headers).unwrap_or_default(),
            })
            .collect();

        let max_block_height = eps.iter().filter_map(|e| e.block_height).max();

        self.cache.insert(
            chain_index,
            ChainEndpoints {
                endpoints: eps.clone(),
                max_block_height,
                refreshed_at: Instant::now(),
            },
        );
        Ok(eps)
    }

    pub async fn select_endpoint(
        &self,
        chain_index: i64,
        method: Option<&str>,
        user_tier: &str,
        max_block_lag: i64,
        require_ws_tunnel: bool,
    ) -> AppResult<SelectedEndpoint> {
        let all_eps = self.get_endpoints(chain_index).await?;
        if all_eps.is_empty() {
            return Err(AppError::Unavailable(format!(
                "no healthy rpc endpoints for chain {chain_index}"
            )));
        }

        let need_archive = method.map(is_archive_method).unwrap_or(false);

        let cached = self.cache.get(&chain_index);
        let max_height = cached.as_ref().and_then(|c| c.max_block_height);

        let filtered: Vec<&CachedEndpoint> = all_eps
            .iter()
            .filter(|ep| {
                if require_ws_tunnel
                    && !parse_endpoint_url_with(&ep.url, ep.protocol)
                        .map(|(p, _)| p.supports_ws_tunnel())
                        .unwrap_or(false)
                {
                    return false;
                }
                if !self.is_tier_allowed(&ep.tier, user_tier) {
                    return false;
                }
                if need_archive && !ep.is_archive {
                    return false;
                }
                if let (Some(mh), Some(eh)) = (max_height, ep.block_height) {
                    if mh.saturating_sub(eh) > max_block_lag {
                        return false;
                    }
                }
                let circuit = self
                    .circuits
                    .entry(ep.url.clone())
                    .or_insert_with(|| {
                        Arc::new(EndpointCircuit::new(
                            self.failure_threshold,
                            Duration::from_secs(30),
                        ))
                    })
                    .clone();
                circuit.allow()
            })
            .collect();

        if filtered.is_empty() {
            return Err(AppError::Unavailable(format!(
                "all rpc endpoints for chain {chain_index} are unavailable"
            )));
        }

        let mut sorted = filtered;
        sorted.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then(
                    b.block_height
                        .unwrap_or_default()
                        .cmp(&a.block_height.unwrap_or_default()),
                )
                .then(a.avg_latency_ms.cmp(&b.avg_latency_ms))
        });

        let top_n = sorted.len().min(8);
        let candidates = &sorted[..top_n];

        let total_weight: i32 = candidates.iter().map(|e| self.effective_weight(e)).sum();
        let mut rng = rand::thread_rng();
        let mut pick = rng.gen_range(0..total_weight);
        for ep in candidates {
            pick -= self.effective_weight(ep);
            if pick < 0 {
                return Ok(SelectedEndpoint {
                    url: ep.url.clone(),
                    protocol: ep.protocol,
                    headers: ep.headers.clone(),
                });
            }
        }
        Ok(SelectedEndpoint {
            url: candidates.last().unwrap().url.clone(),
            protocol: candidates.last().unwrap().protocol,
            headers: candidates.last().unwrap().headers.clone(),
        })
    }

    fn effective_weight(&self, endpoint: &CachedEndpoint) -> i32 {
        let latency_bonus = match endpoint.avg_latency_ms {
            l if l <= 250 => 4,
            l if l <= 600 => 3,
            l if l <= 1200 => 2,
            _ => 1,
        };
        let priority_bonus = endpoint.priority.clamp(0, 5) + 1;
        endpoint
            .weight
            .max(1)
            .saturating_mul(priority_bonus)
            .saturating_mul(latency_bonus)
    }

    fn is_tier_allowed(&self, endpoint_tier: &str, user_tier: &str) -> bool {
        match user_tier {
            "all" => true,
            "paid" => endpoint_tier == "paid",
            "free" => endpoint_tier == "free",
            _ => false,
        }
    }

    pub fn mark_success(&self, url: &str) {
        if let Some(circuit) = self.circuits.get(url) {
            circuit.on_success();
        }
    }

    pub fn mark_failure(&self, url: &str) {
        if let Some(circuit) = self.circuits.get(url) {
            circuit.on_failure();
        }
    }

    pub async fn execute_rpc(
        &self,
        chain_index: i64,
        body: &serde_json::Value,
        user_tier: &str,
        timeout: Duration,
        max_retries: u32,
        max_block_lag: i64,
    ) -> AppResult<serde_json::Value> {
        let method = body.get("method").and_then(|v| v.as_str());

        let mut last_err = None;
        for _ in 0..max_retries.max(1) {
            let selection = match self
                .select_endpoint(chain_index, method, user_tier, max_block_lag.max(0), false)
                .await
            {
                Ok(s) => s,
                Err(e) => return Err(e),
            };
            let url = &selection.url;

            if parse_endpoint_url_with(url, selection.protocol).is_err() {
                self.mark_failure(url);
                last_err = Some(AppError::InvalidArgument(format!(
                    "invalid endpoint url: {url}"
                )));
                continue;
            }

            match transports::execute_unary(
                &self.http,
                url,
                body,
                selection.protocol,
                &selection.headers,
                timeout,
            )
            .await
            {
                Ok(v) => {
                    self.mark_success(url);
                    return Ok(v);
                }
                Err(e) => {
                    self.mark_failure(url);
                    last_err = Some(e);
                }
            }
        }
        Err(last_err.unwrap_or_else(|| AppError::Unavailable("all rpc attempts failed".into())))
    }

    pub fn invalidate_cache(&self, chain_index: i64) {
        self.cache.remove(&chain_index);
    }

    pub fn invalidate_all(&self) {
        self.cache.clear();
    }
}
