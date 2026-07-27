//! Swap provider adapters — real API integrations.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use wallet_error::{AppError, AppResult};
use wallet_types::{Address, ChainIndex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapQuote {
    pub provider: String,
    pub amount_out: String,
    pub price_impact: String,
    pub route_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltSwap {
    pub provider: String,
    pub tx_data: Vec<u8>,
    pub to_address: String,
    pub value: String,
}

#[async_trait]
pub trait SwapProvider: Send + Sync {
    fn name(&self) -> &'static str;
    async fn quote(
        &self,
        chain_index: ChainIndex,
        from_token: &Address,
        to_token: &Address,
        amount: &str,
    ) -> AppResult<SwapQuote>;
    async fn build(
        &self,
        chain_index: ChainIndex,
        from_token: &Address,
        to_token: &Address,
        amount: &str,
        slippage_bps: &str,
        user: &Address,
    ) -> AppResult<BuiltSwap>;
}

fn chain_id_from_index(ci: ChainIndex) -> AppResult<u64> {
    match ci.as_i64() {
        60 => Ok(1),       // ETH mainnet
        56 => Ok(56),      // BSC
        137 => Ok(137),    // Polygon
        42161 => Ok(42161), // Arbitrum
        10 => Ok(10),      // Optimism
        8453 => Ok(8453),   // Base
        other => Err(AppError::InvalidArgument(format!(
            "unsupported chain_index {other} for swap"
        ))),
    }
}

// ── 1inch ────────────────────────────────────────────────────────────────

pub struct OneInchProvider {
    pub http: reqwest::Client,
    pub api_key: String,
}

#[async_trait]
impl SwapProvider for OneInchProvider {
    fn name(&self) -> &'static str {
        "1inch"
    }

    async fn quote(
        &self,
        chain_index: ChainIndex,
        from_token: &Address,
        to_token: &Address,
        amount: &str,
    ) -> AppResult<SwapQuote> {
        let chain_id = chain_id_from_index(chain_index)?;
        let url = format!(
            "https://api.1inch.dev/swap/v6.0/{}/quote?\
             src={}&dst={}&amount={}",
            chain_id,
            from_token.as_str(),
            to_token.as_str(),
            amount,
        );
        let resp = self
            .http
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| AppError::Unavailable(format!("1inch quote: {e}")))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::Unavailable(format!(
                "1inch quote HTTP {status}: {body}"
            )));
        }
        let v: Value = resp
            .json()
            .await
            .map_err(|e| AppError::Unavailable(format!("1inch quote parse: {e}")))?;
        let amount_out = v
            .get("toAmount")
            .and_then(|a| a.as_str())
            .unwrap_or("0")
            .to_string();
        let price_impact = compute_price_impact(amount, &amount_out);
        Ok(SwapQuote {
            provider: self.name().into(),
            amount_out,
            price_impact,
            route_json: v.to_string(),
        })
    }

    async fn build(
        &self,
        chain_index: ChainIndex,
        from_token: &Address,
        to_token: &Address,
        amount: &str,
        slippage_bps: &str,
        user: &Address,
    ) -> AppResult<BuiltSwap> {
        let chain_id = chain_id_from_index(chain_index)?;
        let slippage_pct: f64 = slippage_bps.parse().unwrap_or(50.0) / 100.0;
        let url = format!(
            "https://api.1inch.dev/swap/v6.0/{}/swap?\
             src={}&dst={}&amount={}&from={}&slippage={}",
            chain_id,
            from_token.as_str(),
            to_token.as_str(),
            amount,
            user.as_str(),
            slippage_pct,
        );
        let resp = self
            .http
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| AppError::Unavailable(format!("1inch swap: {e}")))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::Unavailable(format!(
                "1inch swap HTTP {status}: {body}"
            )));
        }
        let v: Value = resp
            .json()
            .await
            .map_err(|e| AppError::Unavailable(format!("1inch swap parse: {e}")))?;
        let to_address = v
            .get("tx")
            .and_then(|tx| tx.get("to"))
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();
        let value = v
            .get("tx")
            .and_then(|tx| tx.get("value"))
            .and_then(|v| v.as_str())
            .unwrap_or("0")
            .to_string();
        let data_hex = v
            .get("tx")
            .and_then(|tx| tx.get("data"))
            .and_then(|d| d.as_str())
            .unwrap_or("0x");
        let tx_data = hex::decode(data_hex.trim_start_matches("0x"))
            .map_err(|e| AppError::InvalidArgument(format!("1inch tx data hex: {e}")))?;
        Ok(BuiltSwap {
            provider: self.name().into(),
            tx_data,
            to_address,
            value,
        })
    }
}

// ── OKX DEX ──────────────────────────────────────────────────────────────

pub struct OkxProvider {
    pub http: reqwest::Client,
    pub api_key: String,
    pub secret: String,
    pub passphrase: String,
}

fn okx_hmac_sign(secret: &str, timestamp: &str, method: &str, path: &str, body: &str) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;
    let message = format!("{}{}{}{}", timestamp, method, path, body);
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(message.as_bytes());
    let result = mac.finalize();
    base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        result.into_bytes(),
    )
}

#[async_trait]
impl SwapProvider for OkxProvider {
    fn name(&self) -> &'static str {
        "okx"
    }

    async fn quote(
        &self,
        chain_index: ChainIndex,
        from_token: &Address,
        to_token: &Address,
        amount: &str,
    ) -> AppResult<SwapQuote> {
        let chain_id = chain_id_from_index(chain_index)?;
        let path = "/api/v5/dex/market/quote";
        let query_string = format!(
            "chainId={}&fromTokenAddress={}&toTokenAddress={}&amount={}",
            chain_id,
            from_token.as_str(),
            to_token.as_str(),
            amount,
        );
        let full_url = format!("{}?{}", path, query_string);
        let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
        let sign = okx_hmac_sign(&self.secret, &timestamp, "GET", &full_url, "");
        let resp = self
            .http
            .get(format!("https://www.okx.com{}", full_url))
            .header("OK-ACCESS-KEY", &self.api_key)
            .header("OK-ACCESS-SIGN", &sign)
            .header("OK-ACCESS-TIMESTAMP", &timestamp)
            .header("OK-ACCESS-PASSPHRASE", &self.passphrase)
            .send()
            .await
            .map_err(|e| AppError::Unavailable(format!("okx quote: {e}")))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::Unavailable(format!(
                "okx quote HTTP {status}: {body}"
            )));
        }
        let v: Value = resp
            .json()
            .await
            .map_err(|e| AppError::Unavailable(format!("okx quote parse: {e}")))?;
        let amount_out = v
            .pointer("/data/[0]/toTokenAmount")
            .and_then(|a| a.as_str())
            .unwrap_or("0")
            .to_string();
        let price_impact = compute_price_impact(amount, &amount_out);
        Ok(SwapQuote {
            provider: self.name().into(),
            amount_out,
            price_impact,
            route_json: v.to_string(),
        })
    }

    async fn build(
        &self,
        chain_index: ChainIndex,
        from_token: &Address,
        to_token: &Address,
        amount: &str,
        slippage_bps: &str,
        user: &Address,
    ) -> AppResult<BuiltSwap> {
        let chain_id = chain_id_from_index(chain_index)?;
        let slippage_pct: f64 = slippage_bps.parse().unwrap_or(50.0) / 100.0;
        let path = "/api/v5/dex/market/trade";
        let body_json = json!({
            "chainId": chain_id,
            "fromTokenAddress": from_token.as_str(),
            "toTokenAddress": to_token.as_str(),
            "amount": amount,
            "userWalletAddress": user.as_str(),
            "slippage": slippage_pct.to_string(),
        });
        let body_str = body_json.to_string();
        let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
        let sign = okx_hmac_sign(&self.secret, &timestamp, "POST", path, &body_str);
        let resp = self
            .http
            .post(format!("https://www.okx.com{path}"))
            .header("OK-ACCESS-KEY", &self.api_key)
            .header("OK-ACCESS-SIGN", &sign)
            .header("OK-ACCESS-TIMESTAMP", &timestamp)
            .header("OK-ACCESS-PASSPHRASE", &self.passphrase)
            .json(&body_json)
            .send()
            .await
            .map_err(|e| AppError::Unavailable(format!("okx trade: {e}")))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::Unavailable(format!(
                "okx trade HTTP {status}: {body}"
            )));
        }
        let v: Value = resp
            .json()
            .await
            .map_err(|e| AppError::Unavailable(format!("okx trade parse: {e}")))?;
        let to_address = v
            .pointer("/data/[0]/to")
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();
        let value = v
            .pointer("/data/[0]/value")
            .and_then(|v| v.as_str())
            .unwrap_or("0")
            .to_string();
        let data_hex = v
            .pointer("/data/[0]/data")
            .and_then(|d| d.as_str())
            .unwrap_or("0x");
        let tx_data = hex::decode(data_hex.trim_start_matches("0x"))
            .map_err(|e| AppError::InvalidArgument(format!("okx tx data hex: {e}")))?;
        Ok(BuiltSwap {
            provider: self.name().into(),
            tx_data,
            to_address,
            value,
        })
    }
}

// ── MetaPath (LiFi-style) ───────────────────────────────────────────────

pub struct MetaPathProvider {
    pub http: reqwest::Client,
    pub api_key: String,
}

#[async_trait]
impl SwapProvider for MetaPathProvider {
    fn name(&self) -> &'static str {
        "metapath"
    }

    async fn quote(
        &self,
        chain_index: ChainIndex,
        from_token: &Address,
        to_token: &Address,
        amount: &str,
    ) -> AppResult<SwapQuote> {
        let chain_id = chain_id_from_index(chain_index)?;
        // MetaPath uses LiFi-compatible API
        let url = "https://metapath-api.primestudio.xyz/v1/quote";
        let params = [
            ("chainIdFrom", chain_id.to_string()),
            ("chainIdTo", chain_id.to_string()),
            ("fromTokenAddress", from_token.as_str().to_string()),
            ("toTokenAddress", to_token.as_str().to_string()),
            ("amount", amount.to_string()),
        ];
        let resp = self
            .http
            .get(url)
            .query(&params)
            .header("x-api-key", &self.api_key)
            .send()
            .await
            .map_err(|e| AppError::Unavailable(format!("metapath quote: {e}")))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::Unavailable(format!(
                "metapath quote HTTP {status}: {body}"
            )));
        }
        let v: Value = resp
            .json()
            .await
            .map_err(|e| AppError::Unavailable(format!("metapath quote parse: {e}")))?;
        let amount_out = v
            .get("toAmount")
            .and_then(|a| a.as_str())
            .unwrap_or("0")
            .to_string();
        let price_impact = compute_price_impact(amount, &amount_out);
        Ok(SwapQuote {
            provider: self.name().into(),
            amount_out,
            price_impact,
            route_json: v.to_string(),
        })
    }

    async fn build(
        &self,
        chain_index: ChainIndex,
        from_token: &Address,
        to_token: &Address,
        amount: &str,
        slippage_bps: &str,
        user: &Address,
    ) -> AppResult<BuiltSwap> {
        let chain_id = chain_id_from_index(chain_index)?;
        let slippage_pct: f64 = slippage_bps.parse().unwrap_or(50.0) / 100.0;
        let url = "https://metapath-api.primestudio.xyz/v1/transaction";
        let body = json!({
            "chainIdFrom": chain_id,
            "chainIdTo": chain_id,
            "fromTokenAddress": from_token.as_str(),
            "toTokenAddress": to_token.as_str(),
            "amount": amount,
            "slippage": slippage_pct / 100.0,
            "fromAddress": user.as_str(),
        });
        let resp = self
            .http
            .post(url)
            .json(&body)
            .header("x-api-key", &self.api_key)
            .send()
            .await
            .map_err(|e| AppError::Unavailable(format!("metapath tx: {e}")))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::Unavailable(format!(
                "metapath tx HTTP {status}: {body}"
            )));
        }
        let v: Value = resp
            .json()
            .await
            .map_err(|e| AppError::Unavailable(format!("metapath tx parse: {e}")))?;
        let to_address = v
            .get("to")
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();
        let value = v
            .get("value")
            .and_then(|v| v.as_str())
            .unwrap_or("0")
            .to_string();
        let data_hex = v
            .get("data")
            .and_then(|d| d.as_str())
            .unwrap_or("0x");
        let tx_data = hex::decode(data_hex.trim_start_matches("0x"))
            .map_err(|e| AppError::InvalidArgument(format!("metapath tx data hex: {e}")))?;
        Ok(BuiltSwap {
            provider: self.name().into(),
            tx_data,
            to_address,
            value,
        })
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────

fn compute_price_impact(amount_in: &str, amount_out: &str) -> String {
    let a: f64 = amount_in.parse().unwrap_or(1.0);
    let b: f64 = amount_out.parse().unwrap_or(0.0);
    if a == 0.0 {
        return "0".into();
    }
    // Simplified: ratio deviation from 1:1
    let impact = ((b / a) - 1.0).abs() * 100.0;
    format!("{impact:.4}")
}

// ── Service ──────────────────────────────────────────────────────────────

pub struct SwapService<'a> {
    pub state: &'a crate::AppState,
}

impl<'a> SwapService<'a> {
    pub fn new(state: &'a crate::AppState) -> Self {
        Self { state }
    }

    fn provider(&self, name: &str) -> AppResult<Box<dyn SwapProvider>> {
        match name {
            "" | "1inch" => {
                let api_key = std::env::var("ONEINCH_API_KEY").unwrap_or_default();
                if api_key.is_empty() {
                    return Err(AppError::Unavailable("ONEINCH_API_KEY not configured".into()));
                }
                Ok(Box::new(OneInchProvider {
                    http: self.state.http.clone(),
                    api_key,
                }))
            }
            "okx" => {
                let api_key = std::env::var("OKX_API_KEY").unwrap_or_default();
                let secret = std::env::var("OKX_SECRET").unwrap_or_default();
                let passphrase = std::env::var("OKX_PASSPHRASE").unwrap_or_default();
                if api_key.is_empty() || secret.is_empty() || passphrase.is_empty() {
                    return Err(AppError::Unavailable(
                        "OKX_API_KEY/OKX_SECRET/OKX_PASSPHRASE not configured".into(),
                    ));
                }
                Ok(Box::new(OkxProvider {
                    http: self.state.http.clone(),
                    api_key,
                    secret,
                    passphrase,
                }))
            }
            "metapath" => {
                let api_key = std::env::var("METAPATH_API_KEY").unwrap_or_default();
                if api_key.is_empty() {
                    return Err(AppError::Unavailable("METAPATH_API_KEY not configured".into()));
                }
                Ok(Box::new(MetaPathProvider {
                    http: self.state.http.clone(),
                    api_key,
                }))
            }
            other => Err(AppError::InvalidArgument(format!(
                "unknown swap provider: {other}"
            ))),
        }
    }

    pub async fn quote(
        &self,
        chain_index: ChainIndex,
        from_token: &Address,
        to_token: &Address,
        amount: &str,
        provider: &str,
    ) -> AppResult<SwapQuote> {
        self.provider(provider)?
            .quote(chain_index, from_token, to_token, amount)
            .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn build(
        &self,
        chain_index: ChainIndex,
        from_token: &Address,
        to_token: &Address,
        amount: &str,
        slippage_bps: &str,
        user: &Address,
        provider: &str,
    ) -> AppResult<BuiltSwap> {
        self.provider(provider)?
            .build(
                chain_index,
                from_token,
                to_token,
                amount,
                slippage_bps,
                user,
            )
            .await
    }
}
