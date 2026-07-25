use dashmap::DashMap;
use serde_json::Value;
use tokio::sync::broadcast;

#[derive(Default)]
pub struct Hub {
    topics: DashMap<String, broadcast::Sender<Value>>,
}

impl Hub {
    pub fn subscribe(&self, topic: &str) -> broadcast::Receiver<Value> {
        self.topics
            .entry(topic.to_string())
            .or_insert_with(|| broadcast::channel(256).0)
            .subscribe()
    }

    pub fn publish(&self, topic: &str, payload: Value) {
        if let Some(tx) = self.topics.get(topic) {
            let _ = tx.send(payload);
        }
    }
}
