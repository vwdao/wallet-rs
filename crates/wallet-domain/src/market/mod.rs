use crate::AppState;
use wallet_db::clickhouse::KlineRow;
use wallet_error::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct PriceQuote {
    pub symbol: String,
    pub price: String,
    pub updated_at_unix: i64,
}

pub struct MarketService<'a> {
    pub state: &'a AppState,
}

impl<'a> MarketService<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    pub async fn get_price(&self, symbol: &str) -> AppResult<PriceQuote> {
        // Binance mini-ticker style fetch (best-effort).
        let url = format!(
            "https://api.binance.com/api/v3/ticker/price?symbol={}",
            symbol.to_uppercase()
        );
        let resp = self
            .state
            .http
            .get(url)
            .send()
            .await
            .map_err(|e| AppError::Unavailable(e.to_string()))?;
        let v: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Unavailable(e.to_string()))?;
        let price = v
            .get("price")
            .and_then(|p| p.as_str())
            .unwrap_or("0")
            .to_string();
        Ok(PriceQuote {
            symbol: symbol.to_string(),
            price,
            updated_at_unix: chrono::Utc::now().timestamp(),
        })
    }

    pub async fn get_klines(&self, symbol: &str, interval: &str) -> AppResult<Vec<KlineRow>> {
        let Some(ch) = &self.state.clickhouse else {
            return Err(AppError::Unavailable("clickhouse not configured".into()));
        };
        let table = match interval {
            "1m" => "kline_1m",
            "5m" => "kline_5m",
            "15m" => "kline_15m",
            "1h" => "kline_1h",
            "4h" => "kline_4h",
            "1d" => "kline_1d",
            _ => "kline_1m",
        };
        let query = format!(
            "SELECT ?fields FROM {} WHERE symbol = ? ORDER BY open_time DESC LIMIT 100",
            table,
        );
        let mut cursor = ch
            .client()
            .query(&query)
            .bind(symbol)
            .fetch::<KlineRow>()
            .map_err(|e| AppError::Unavailable(e.to_string()))?;
        let mut rows = Vec::new();
        while let Some(row) = cursor
            .next()
            .await
            .map_err(|e| AppError::Unavailable(e.to_string()))?
        {
            rows.push(row);
        }
        Ok(rows)
    }

    /// DexScreener-style ingest hook used by wallet-jobs.
    pub async fn ingest_dex_trade(
        &self,
        chain_index: i64,
        tx_hash: &str,
        payload: serde_json::Value,
    ) -> AppResult<()> {
        let Some(ch) = &self.state.clickhouse else {
            return Ok(());
        };
        let mut insert = ch
            .client()
            .insert("dex_trade")
            .map_err(|e| AppError::Unavailable(e.to_string()))?;
        #[derive(clickhouse::Row, serde::Serialize)]
        struct DexRow {
            chain_index: i64,
            tx_hash: String,
            ts: i64,
            payload: String,
        }
        insert
            .write(&DexRow {
                chain_index,
                tx_hash: tx_hash.into(),
                ts: chrono::Utc::now().timestamp(),
                payload: payload.to_string(),
            })
            .await
            .map_err(|e| AppError::Unavailable(e.to_string()))?;
        insert
            .end()
            .await
            .map_err(|e| AppError::Unavailable(e.to_string()))?;
        Ok(())
    }
}
