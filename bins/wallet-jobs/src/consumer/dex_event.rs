//! DEX event consumer — watches NATS for swap events and persists them.

use std::sync::Arc;
use tracing::{error, info};
use wallet_db::Db;
use wallet_events::EventBus;

pub async fn start(db: Db, bus: Arc<dyn EventBus>) -> anyhow::Result<()> {
    let mut sub = bus.subscribe("wallet.swap.*", "dex-consumer").await?;
    info!("dex_event consumer started");
    loop {
        match sub.next().await {
            Ok(Some(env)) => {
                let v = &env.payload;
                let from_token = v.get("from_token").and_then(|s| s.as_str()).unwrap_or("");
                let to_token = v.get("to_token").and_then(|s| s.as_str()).unwrap_or("");
                let amount = v.get("amount").and_then(|s| s.as_str()).unwrap_or("0");
                let price_impact = v
                    .get("price_impact")
                    .and_then(|s| s.as_str())
                    .unwrap_or("0");
                let tx_hash = v.get("tx_hash").and_then(|s| s.as_str()).unwrap_or("");
                let wallet = v.get("wallet").and_then(|s| s.as_str()).unwrap_or("");
                let chain_index = v.get("chain_index").and_then(|c| c.as_i64()).unwrap_or(0);

                // Execute with exponential backoff retry to handle transient DB blips
                let mut db_success = false;
                for attempt in 0..3 {
                    let mut inner = db.clone_inner();
                    let result = toasty::sql::statement(
                        "INSERT INTO dex_quotes (chain_index, wallet, from_token, to_token, amount, price_impact, tx_hash, created_at) \
                         VALUES ($1, $2, $3, $4, $5, $6, $7, NOW()) \
                         ON CONFLICT (tx_hash) DO NOTHING",
                    )
                    .bind(chain_index)
                    .bind(wallet)
                    .bind(from_token)
                    .bind(to_token)
                    .bind(amount)
                    .bind(price_impact)
                    .bind(tx_hash)
                    .exec(&mut inner)
                    .await;

                    match result {
                        Ok(_) => {
                            db_success = true;
                            break;
                        }
                        Err(e) => {
                            error!("dex_event db error (attempt {attempt}): {e}");
                            tokio::time::sleep(std::time::Duration::from_millis(200 * (1 << attempt))).await;
                        }
                    }
                }

                if !db_success {
                    // Route poisonous or unprocessable events to Dead Letter Queue (DLQ)
                    let dlq_payload = serde_json::json!({
                        "event": v,
                        "failed_at": chrono::Utc::now().to_rfc3339(),
                        "reason": "db_insert_failed_after_retries",
                    });
                    let _ = bus.publish("wallet.dlq.dex", dlq_payload).await;
                }

                if let Err(e) = sub.ack_last().await {
                    error!("dex_event ack error: {e}");
                }
            }
            Ok(None) => {
                info!("dex_event consumer: stream ended");
                break;
            }
            Err(e) => {
                error!("dex_event consume error: {e}");
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    }
    Ok(())
}
