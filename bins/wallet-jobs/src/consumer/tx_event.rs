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

                // Execute with exponential backoff retry to handle transient DB blips
                let mut db_success = false;
                for attempt in 0..3 {
                    let mut inner = db.clone_inner();
                    let result = toasty::sql::statement(
                        "UPDATE transactions SET status = $1, updated_at = NOW() WHERE hash = $2 AND chain_index = $3",
                    )
                    .bind(status)
                    .bind(hash)
                    .bind(chain)
                    .exec(&mut inner)
                    .await;

                    match result {
                        Ok(_) => {
                            db_success = true;
                            // Auto-discover newly encountered token contracts from sync events
                            if let Some(contract) = v.get("contract_address").and_then(|c| c.as_str()) {
                                if !contract.is_empty() {
                                    let _ = toasty::sql::statement(
                                        "INSERT INTO tokens (id, chain_index, address, symbol, name, decimals, created_at) \
                                         VALUES ($1, $2, $3, $4, $5, $6, NOW()) \
                                         ON CONFLICT (chain_index, address) DO NOTHING",
                                    )
                                    .bind(uuid::Uuid::new_v4())
                                    .bind(chain)
                                    .bind(contract)
                                    .bind("UNKNOWN")
                                    .bind("Unknown Token")
                                    .bind(18i32)
                                    .exec(&mut inner)
                                    .await;
                                }
                            }
                            break;
                        }
                        Err(e) => {
                            error!("tx_event db error (attempt {attempt}): {e}");
                            tokio::time::sleep(std::time::Duration::from_millis(200 * (1 << attempt))).await;
                        }
                    }
                }

                if !db_success {
                    // Route poisonous or unprocessable events to Dead Letter Queue (DLQ)
                    let dlq_payload = serde_json::json!({
                        "event": v,
                        "failed_at": chrono::Utc::now().to_rfc3339(),
                        "reason": "db_update_failed_after_retries",
                    });
                    let _ = bus.publish("wallet.dlq.tx", dlq_payload).await;
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
