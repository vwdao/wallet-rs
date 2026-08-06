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

        for event in &events {
            let mut db = self.db.clone_inner();
            let _ = toasty::create!(wallet_db::ChainGatewayStats {
                api_key: &event.api_key,
                chain_index: event.chain_index,
                client_ip: event.client_ip.clone(),
                protocol: event.protocol.clone(),
                method: event.method.clone(),
                status_code: event.status_code,
                latency_ms: event.latency_ms,
                error_msg: event.error_msg.clone(),
            })
            .exec(&mut db)
            .await;
        }

        tracing::debug!("flushed {count} stats events");
    }
}
