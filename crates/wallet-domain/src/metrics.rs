//! Basic metrics collection for observability.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// Simple metrics collector
#[derive(Debug, Clone, Default)]
pub struct Metrics {
    inner: Arc<MetricsInner>,
}

#[derive(Debug, Default)]
struct MetricsInner {
    requests_total: AtomicU64,
    requests_success: AtomicU64,
    requests_error: AtomicU64,
    db_queries_total: AtomicU64,
    db_queries_error: AtomicU64,
    rpc_calls_total: AtomicU64,
    rpc_calls_error: AtomicU64,
    events_published: AtomicU64,
}

impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_request(&self) {
        self.inner.requests_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_request_success(&self) {
        self.inner.requests_success.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_request_error(&self) {
        self.inner.requests_error.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_db_query(&self) {
        self.inner.db_queries_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_db_error(&self) {
        self.inner.db_queries_error.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_rpc_call(&self) {
        self.inner.rpc_calls_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_rpc_error(&self) {
        self.inner.rpc_calls_error.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_event_published(&self) {
        self.inner.events_published.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            requests_total: self.inner.requests_total.load(Ordering::Relaxed),
            requests_success: self.inner.requests_success.load(Ordering::Relaxed),
            requests_error: self.inner.requests_error.load(Ordering::Relaxed),
            db_queries_total: self.inner.db_queries_total.load(Ordering::Relaxed),
            db_queries_error: self.inner.db_queries_error.load(Ordering::Relaxed),
            rpc_calls_total: self.inner.rpc_calls_total.load(Ordering::Relaxed),
            rpc_calls_error: self.inner.rpc_calls_error.load(Ordering::Relaxed),
            events_published: self.inner.events_published.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MetricsSnapshot {
    pub requests_total: u64,
    pub requests_success: u64,
    pub requests_error: u64,
    pub db_queries_total: u64,
    pub db_queries_error: u64,
    pub rpc_calls_total: u64,
    pub rpc_calls_error: u64,
    pub events_published: u64,
}

/// Timer for measuring operation duration
pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn start() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
}
