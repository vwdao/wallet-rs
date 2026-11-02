use parking_lot::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use wallet_config::RpcEndpoint;
use wallet_error::{AppError, AppResult};

#[derive(Debug, Default)]
struct CircuitState {
    failures: u32,
    open_until: Option<Instant>,
}

#[derive(Debug)]
pub struct CircuitBreaker {
    state: Mutex<CircuitState>,
    threshold: u32,
    cool_down: Duration,
}

impl CircuitBreaker {
    pub fn new(threshold: u32, cool_down: Duration) -> Self {
        Self {
            state: Mutex::new(CircuitState::default()),
            threshold,
            cool_down,
        }
    }

    pub fn allow(&self) -> bool {
        let state = self.state.lock();
        if let Some(until) = state.open_until {
            if Instant::now() < until {
                return false;
            }
        }
        true
    }

    pub fn on_success(&self) {
        let mut state = self.state.lock();
        state.failures = 0;
        state.open_until = None;
    }

    pub fn on_failure(&self) {
        let mut state = self.state.lock();
        state.failures = state.failures.saturating_add(1);
        if state.failures >= self.threshold {
            state.open_until = Some(Instant::now() + self.cool_down);
        }
    }
}

#[derive(Debug)]
pub struct RpcPool {
    endpoints: Vec<RpcEndpoint>,
    cursor: AtomicUsize,
    breakers: Vec<CircuitBreaker>,
    client: reqwest::Client,
}

impl RpcPool {
    pub fn new(endpoints: Vec<RpcEndpoint>) -> AppResult<Self> {
        if endpoints.is_empty() {
            return Err(AppError::InvalidArgument("no rpc endpoints".into()));
        }
        let breakers = endpoints
            .iter()
            .map(|_| CircuitBreaker::new(3, Duration::from_secs(30)))
            .collect();
        let client = reqwest::Client::builder()
            .no_proxy()
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(32)
            .tcp_keepalive(Duration::from_secs(60))
            .tcp_nodelay(true)
            .timeout(Duration::from_secs(20))
            .connect_timeout(Duration::from_secs(8))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Ok(Self {
            endpoints,
            cursor: AtomicUsize::new(0),
            breakers,
            client,
        })
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn len(&self) -> usize {
        self.endpoints.len()
    }

    pub fn next_url(&self) -> AppResult<&str> {
        let n = self.endpoints.len();
        for _ in 0..n {
            let i = self.cursor.fetch_add(1, Ordering::Relaxed) % n;
            if self.breakers[i].allow() {
                return Ok(&self.endpoints[i].url);
            }
        }
        Err(AppError::Unavailable(
            "all rpc endpoints open-circuit".into(),
        ))
    }

    pub fn mark_success(&self, url: &str) {
        if let Some((i, _)) = self
            .endpoints
            .iter()
            .enumerate()
            .find(|(_, e)| e.url == url)
        {
            self.breakers[i].on_success();
        }
    }

    pub fn mark_failure(&self, url: &str) {
        if let Some((i, _)) = self
            .endpoints
            .iter()
            .enumerate()
            .find(|(_, e)| e.url == url)
        {
            self.breakers[i].on_failure();
        }
    }
}
