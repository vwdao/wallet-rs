//! Transaction event consumer — watches NATS and updates database.

use std::sync::Arc;
use tracing::{error, info};
use wallet_db::Db;
use wallet_events::EventBus;

pub async fn start(db: Db, bus: Arc<dyn EventBus>) -> anyhow::Result<()> {
    let mut sub = bus.subscribe("wallet.tx.*", "tx-consumer").await?;
    info!("tx_event consumer started");
    loop {
        match sub.next().await {
            Ok(Some(env)) => {
                let v = &env.payload;
                let hash = v.get("hash").and_then(|h| h.as_str()).unwrap_or("");
                let chain = v.get("chain_index").and_then(|c| c.as_i64()).unwrap_or(0);
                let status = v
                    .get("status")
                    .and_then(|s| s.as_str())
                    .unwrap_or("pending");
                let mut inner = db.clone_inner();
                let result = toasty::sql::statement(
                    "UPDATE transactions SET status = $1, updated_at = NOW() WHERE hash = $2 AND chain_index = $3",
                )
                .bind(status)
                .bind(hash)
                .bind(chain)
                .exec(&mut inner)
                .await;
                if let Err(e) = result {
                    error!("tx_event db error: {e}");
                }
                if let Err(e) = sub.ack_last().await {
                    error!("tx_event ack error: {e}");
                }
            }
            Ok(None) => {
                info!("tx_event consumer: stream ended");
                break;
            }
            Err(e) => {
                error!("tx_event consume error: {e}");
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    }
    Ok(())
}
