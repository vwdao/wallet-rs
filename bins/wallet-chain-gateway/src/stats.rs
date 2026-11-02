use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct StatsEvent {
    pub api_key: String,
    pub chain_index: i64,
    pub client_ip: Option<String>,
    pub protocol: Option<String>,
    pub method: Option<String>,
    pub status_code: i32,
    pub latency_ms: i32,
    pub error_msg: Option<String>,
}

pub struct StatsCollector {
    tx: mpsc::Sender<StatsEvent>,
}

impl Clone for StatsCollector {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

impl StatsCollector {
    pub fn new(db: wallet_db::Db, batch_interval: Duration) -> Self {
        let (tx, rx) = mpsc::channel(4096);
        let worker = StatsWorker {
            db,
            rx,
            batch_interval,
        };
        worker.spawn();
        Self { tx }
    }

    pub fn record(&self, event: StatsEvent) {
        let _ = self.tx.try_send(event);
    }
}

struct StatsWorker {
    db: wallet_db::Db,
    rx: mpsc::Receiver<StatsEvent>,
    batch_interval: Duration,
}

impl StatsWorker {
    fn spawn(self) {
        tokio::spawn(async move {
            self.run().await;
        });
    }

    async fn run(mut self) {
        let mut buffer = Vec::with_capacity(200);
        let mut interval = tokio::time::interval(self.batch_interval);
        loop {
            tokio::select! {
                Some(event) = self.rx.recv() => {
                    buffer.push(event);
                    if buffer.len() >= 200 {
                        self.flush(&mut buffer).await;
                    }
                }
                _ = interval.tick() => {
                    if !buffer.is_empty() {
                        self.flush(&mut buffer).await;
                    }
                }
                else => {
                    if !buffer.is_empty() {
                        self.flush(&mut buffer).await;
                    }
                    break;
                }
            }
        }
    }

    async fn flush(&self, buffer: &mut Vec<StatsEvent>) {
        let events: Vec<StatsEvent> = buffer.drain(..).collect();
        let count = events.len();
        if count == 0 {
            return;
        }

        let mut db = self.db.clone_inner();

        // 1. Batch increment total_requests on chain_gateway_keys, eliminating
        // per-request synchronous row-level lock contention in Postgres.
        let mut key_counts: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        for event in &events {
            if !event.api_key.is_empty() {
                *key_counts.entry(event.api_key.clone()).or_insert(0) += 1;
            }
        }
        for (api_key, inc) in key_counts {
            let _ = toasty::sql::statement(
                "UPDATE chain_gateway_keys SET total_requests = total_requests + $1 WHERE api_key = $2",
            )
            .bind(inc)
            .bind(&api_key)
            .exec(&mut db)
            .await;
        }

        // 2. Multi-row batch insertion: converts up to 200 sequential DB round trips into 1-2 statements
        for chunk in events.chunks(100) {
            let cols_per_row = 9;
            let mut placeholders = String::with_capacity(chunk.len() * 28);
            for i in 0..chunk.len() {
                if i > 0 {
                    placeholders.push(',');
                }
                let base = i * cols_per_row + 1;
                placeholders.push_str(&format!(
                    "(${},${},${},${},${},${},${},${},${})",
                    base, base + 1, base + 2, base + 3, base + 4, base + 5, base + 6, base + 7, base + 8
                ));
            }
            let sql = format!(
                "INSERT INTO chain_gateway_stats (id, api_key, chain_index, client_ip, protocol, method, status_code, latency_ms, error_msg) \
                 VALUES {placeholders}"
            );
            let mut stmt = toasty::sql::statement(&sql);
            for event in chunk {
                stmt = stmt
                    .bind(uuid::Uuid::new_v4())
                    .bind(&event.api_key)
                    .bind(event.chain_index)
                    .bind_opt_text(event.client_ip.as_deref())
                    .bind_opt_text(event.protocol.as_deref())
                    .bind_opt_text(event.method.as_deref())
                    .bind(event.status_code)
                    .bind(event.latency_ms)
                    .bind_opt_text(event.error_msg.as_deref());
            }
            if let Err(e) = stmt.exec(&mut db).await {
                tracing::warn!(error = %e, "batch stats insert failed");
            }
        }

        tracing::debug!("flushed {count} stats events");
    }
}

trait BindOptText {
    fn bind_opt_text(self, value: Option<&str>) -> Self;
}

impl BindOptText for toasty::sql::Statement {
    fn bind_opt_text(mut self, value: Option<&str>) -> Self {
        self = match value {
            Some(s) => self.bind(s),
            None => self.bind_typed(toasty::stmt::Value::Null, toasty::schema::db::Type::Text),
        };
        self
    }
}
