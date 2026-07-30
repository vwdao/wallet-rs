//! Price sync job — fetches prices from Binance and persists to DB + ClickHouse.

use std::sync::Arc;
use std::time::Duration;
use wallet_domain::AppState;

const SYMBOLS: &[&str] = &[
    "ETHUSDT",
    "BTCUSDT",
    "BNBUSDT",
    "SOLUSDT",
    "TRXUSDT",
    "DOGEUSDT",
    "ADAUSDT",
    "AVAXUSDT",
    "DOTUSDT",
    "MATICUSDT",
    "LINKUSDT",
    "UNIUSDT",
    "ATOMUSDT",
    "LTCUSDT",
    "XRPUSDT",
];

#[derive(clickhouse::Row, serde::Serialize)]
struct PriceTick {
    symbol: String,
    price: f64,
    ts: i64,
}

pub async fn run(state: Arc<AppState>) -> anyhow::Result<()> {
    let client = &state.http;
    loop {
        for symbol in SYMBOLS {
            match fetch_and_store(client, &state, symbol).await {
                Ok(()) => {}
                Err(e) => tracing::warn!(symbol, error = %e, "price sync failed"),
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}

async fn fetch_and_store(
    client: &reqwest::Client,
    state: &AppState,
    symbol: &str,
) -> wallet_error::AppResult<()> {
    let url = format!(
        "https://api.binance.com/api/v3/ticker/price?symbol={}",
        symbol
    );
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| wallet_error::AppError::Unavailable(e.to_string()))?;
    let v: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| wallet_error::AppError::Unavailable(e.to_string()))?;
    let price = v
        .get("price")
        .and_then(|p| p.as_str())
        .unwrap_or("0")
        .to_string();

    tracing::info!(symbol, price = %price, "price fetched");

    let mut db = state.db.clone_inner();
    let result = toasty::sql::statement(
        "INSERT INTO latest_prices (symbol, price, updated_at) \
         VALUES ($1, $2, NOW()) \
         ON CONFLICT (symbol) DO UPDATE SET price = $2, updated_at = NOW()",
    )
    .bind(symbol)
    .bind(&price)
    .exec(&mut db)
    .await;
    if let Err(e) = result {
        tracing::warn!(symbol, error = %e, "price pg write failed");
    }

    if let Some(ch) = &state.clickhouse {
        let insert_result = async {
            let mut insert = ch
                .client()
                .insert("price_tick")
                .map_err(|e| wallet_error::AppError::Unavailable(e.to_string()))?;
            insert
                .write(&PriceTick {
                    symbol: symbol.to_string(),
                    price: price.parse().unwrap_or(0.0),
                    ts: chrono::Utc::now().timestamp(),
                })
                .await
                .map_err(|e| wallet_error::AppError::Unavailable(e.to_string()))?;
            insert
                .end()
                .await
                .map_err(|e| wallet_error::AppError::Unavailable(e.to_string()))?;
            Ok::<(), wallet_error::AppError>(())
        }
        .await;
        if let Err(e) = insert_result {
            tracing::warn!(symbol, error = %e, "price clickhouse write failed");
        }
    }

    let _ = state
        .events
        .publish(
            "wallet.price.updated",
            serde_json::json!({
                "symbol": symbol,
                "price": price,
                "ts": chrono::Utc::now().timestamp(),
            }),
        )
        .await;

    Ok(())
}
