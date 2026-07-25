use parking_lot::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use wallet_config::RpcEndpoint;
use wallet_error::{AppError, AppResult};

#[derive(Debug)]
pub struct CircuitBreaker {
    failures: Mutex<u32>,
    open_until: Mutex<Option<Instant>>,
    threshold: u32,
    cool_down: Duration,
}

impl CircuitBreaker {
    pub fn new(threshold: u32, cool_down: Duration) -> Self {
        Self {
            failures: Mutex::new(0),
            open_until: Mutex::new(None),
            threshold,
            cool_down,
        }
    }

    pub fn allow(&self) -> bool {
        let open = self.open_until.lock();
        if let Some(until) = *open {
            if Instant::now() < until {
                return false;
            }
        }
        true
    }

    pub fn on_success(&self) {
        *self.failures.lock() = 0;
        *self.open_until.lock() = None;
    }

    pub fn on_failure(&self) {
        let mut failures = self.failures.lock();
        *failures += 1;
        if *failures >= self.threshold {
            *self.open_until.lock() = Some(Instant::now() + self.cool_down);
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
        Ok(Self {
            endpoints,
            cursor: AtomicUsize::new(0),
            breakers,
            client: reqwest::Client::new(),
        })
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn next_url(&self) -> AppResult<&str> {
        let n = self.endpoints.len();
        for _ in 0..n {
            let i = self.cursor.fetch_add(1, Ordering::Relaxed) % n;
            if self.breakers[i].allow() {
                return Ok(&self.endpoints[i].url);
            }
        }
        Err(AppError::Unavailable("all rpc endpoints open-circuit".into()))
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
