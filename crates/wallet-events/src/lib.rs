//! Domain event bus (NATS JetStream implementation).

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use wallet_error::{AppError, AppResult};

pub const SUBJECT_USER_REGISTER: &str = "wallet.user.register";
pub const SUBJECT_TOKEN_CREATE: &str = "wallet.token.create";
pub const SUBJECT_TX_INDEXED: &str = "wallet.tx.indexed";
pub const SUBJECT_DEX_TRADE: &str = "wallet.dex.trade";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub subject: String,
    pub payload: serde_json::Value,
}

#[async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, subject: &str, payload: serde_json::Value) -> AppResult<()>;
    async fn subscribe(
        &self,
        subject: &str,
        durable: &str,
    ) -> AppResult<Box<dyn EventSubscription>>;
}

#[async_trait]
pub trait EventSubscription: Send {
    async fn next(&mut self) -> AppResult<Option<EventEnvelope>>;
    async fn ack_last(&mut self) -> AppResult<()>;
}

/// In-memory bus for unit tests and local boot without NATS.
pub struct MemoryEventBus {
    tx: tokio::sync::broadcast::Sender<EventEnvelope>,
}

impl MemoryEventBus {
    pub fn new(capacity: usize) -> Arc<Self> {
        let (tx, _) = tokio::sync::broadcast::channel(capacity);
        Arc::new(Self { tx })
    }
}

#[async_trait]
impl EventBus for MemoryEventBus {
    async fn publish(&self, subject: &str, payload: serde_json::Value) -> AppResult<()> {
        if self
            .tx
            .send(EventEnvelope {
                subject: subject.to_string(),
                payload,
            })
            .is_err()
        {
            tracing::debug!(subject, "memory event bus has no active subscribers");
        }
        Ok(())
    }

    async fn subscribe(
        &self,
        subject: &str,
        _durable: &str,
    ) -> AppResult<Box<dyn EventSubscription>> {
        Ok(Box::new(MemorySub {
            subject: subject.to_string(),
            rx: self.tx.subscribe(),
            last: None,
        }))
    }
}

struct MemorySub {
    subject: String,
    rx: tokio::sync::broadcast::Receiver<EventEnvelope>,
    last: Option<EventEnvelope>,
}

#[async_trait]
impl EventSubscription for MemorySub {
    async fn next(&mut self) -> AppResult<Option<EventEnvelope>> {
        loop {
            match self.rx.recv().await {
                Ok(ev) if ev.subject == self.subject || self.subject.ends_with('>') => {
                    self.last = Some(ev.clone());
                    return Ok(Some(ev));
                }
                Ok(_) => continue,
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                Err(_) => return Ok(None),
            }
        }
    }

    async fn ack_last(&mut self) -> AppResult<()> {
        self.last = None;
        Ok(())
    }
}

pub struct NatsEventBus {
    client: async_nats::Client,
    jetstream: async_nats::jetstream::Context,
}

impl NatsEventBus {
    pub async fn connect(url: &str) -> AppResult<Arc<Self>> {
        let client = async_nats::connect(url)
            .await
            .map_err(|e| AppError::Unavailable(format!("nats: {e}")))?;
        let jetstream = async_nats::jetstream::new(client.clone());
        let _ = jetstream
            .get_or_create_stream(async_nats::jetstream::stream::Config {
                name: "WALLET".into(),
                subjects: vec!["wallet.>".into()],
                ..Default::default()
            })
            .await;
        Ok(Arc::new(Self { client, jetstream }))
    }
}

#[async_trait]
impl EventBus for NatsEventBus {
    async fn publish(&self, subject: &str, payload: serde_json::Value) -> AppResult<()> {
        let bytes = serde_json::to_vec(&payload).map_err(|e| AppError::internal(e.to_string()))?;
        self.jetstream
            .publish(subject.to_string(), bytes.into())
            .await
            .map_err(|e| AppError::Unavailable(e.to_string()))?
            .await
            .map_err(|e| AppError::Unavailable(e.to_string()))?;
        Ok(())
    }

    async fn subscribe(
        &self,
        subject: &str,
        durable: &str,
    ) -> AppResult<Box<dyn EventSubscription>> {
        let stream = self
            .jetstream
            .get_stream("WALLET")
            .await
            .map_err(|e| AppError::Unavailable(e.to_string()))?;
        let consumer = stream
            .get_or_create_consumer(
                durable,
                async_nats::jetstream::consumer::pull::Config {
                    durable_name: Some(durable.into()),
                    filter_subject: subject.into(),
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| AppError::Unavailable(e.to_string()))?;
        Ok(Box::new(NatsSub {
            consumer,
            last_msg: None,
        }))
    }
}

struct NatsSub {
    consumer:
        async_nats::jetstream::consumer::Consumer<async_nats::jetstream::consumer::pull::Config>,
    last_msg: Option<async_nats::jetstream::Message>,
}

#[async_trait]
impl EventSubscription for NatsSub {
    async fn next(&mut self) -> AppResult<Option<EventEnvelope>> {
        let mut messages = self
            .consumer
            .messages()
            .await
            .map_err(|e| AppError::Unavailable(e.to_string()))?;
        use futures::StreamExt;
        // Skip malformed payloads (ack them to avoid infinite redelivery) but
        // surface the first parseable envelope.
        while let Some(msg) = messages.next().await {
            let msg = msg.map_err(|e| AppError::Unavailable(e.to_string()))?;
            match serde_json::from_slice::<serde_json::Value>(&msg.payload) {
                Ok(payload) => {
                    let env = EventEnvelope {
                        subject: msg.subject.to_string(),
                        payload,
                    };
                    self.last_msg = Some(msg);
                    return Ok(Some(env));
                }
                Err(e) => {
                    tracing::warn!(
                        subject = %msg.subject,
                        error = %e,
                        "dropping malformed event payload"
                    );
                    msg.ack().await.ok();
                }
            }
        }
        Ok(None)
    }

    async fn ack_last(&mut self) -> AppResult<()> {
        if let Some(msg) = self.last_msg.take() {
            msg.ack()
                .await
                .map_err(|e| AppError::Unavailable(e.to_string()))?;
        }
        Ok(())
    }
}

// silence unused client field warning in some builds
impl NatsEventBus {
    pub fn client(&self) -> &async_nats::Client {
        &self.client
    }
}
