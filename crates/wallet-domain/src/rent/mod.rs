//! TRON energy/bandwidth rental — real marketplace API integration.

use rust_decimal::{prelude::ToPrimitive, Decimal};
use serde_json::{json, Value};
use wallet_error::{AppError, AppResult};
use wallet_types::Address;

/// 1 TRX = 1_000_000 SUN.
const SUN_PER_TRX: i64 = 1_000_000;

/// Rental duration bucket for a given energy amount.
fn duration_for_energy(energy: u64) -> u64 {
    match energy {
        0..=10_000 => 1,
        10_001..=100_000 => 6,
        100_001..=1_000_000 => 24,
        _ => 72,
    }
}

/// Total cost in SUN for `energy` units at `price_per_energy` TRX per unit,
/// using exact decimal math (rounded to the nearest whole SUN).
fn price_sun(price_per_energy: Decimal, energy: u64) -> AppResult<u64> {
    let total_sun = price_per_energy
        .checked_mul(Decimal::from(SUN_PER_TRX))
        .and_then(|p| p.checked_mul(Decimal::from(energy)))
        .ok_or_else(|| AppError::internal("energy total overflow"))?
        .round();
    total_sun
        .to_u64()
        .ok_or_else(|| AppError::internal("energy total out of u64 range"))
}

#[derive(Debug, Clone)]
pub struct EnergyEstimate {
    pub price: String,
    pub duration_hours: u64,
}

#[derive(Debug, Clone)]
pub struct RentOrder {
    pub order_id: String,
    pub status: String,
}

// TRON energy rental API providers
const RENT_API_BASE: &str = "https://apilist.tronscan.org/api";

pub struct RentService {
    pub http: reqwest::Client,
}

impl Default for RentService {
    fn default() -> Self {
        Self::new()
    }
}

impl RentService {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
        }
    }

    /// Estimate energy rental cost by querying available providers.
    pub async fn estimate(&self, address: &Address, energy: u64) -> AppResult<EnergyEstimate> {
        // 1. Query current energy price from TronStake / resource markets
        let price_per_energy = self.fetch_energy_price().await?;

        // 2. Check account's current staked energy for dynamic pricing
        let account_energy = self.fetch_account_energy(address).await?;
        let effective_energy = energy.saturating_sub(account_energy);

        // 3. Calculate total cost in SUN using integer decimal math
        let price_sun_out = price_sun(price_per_energy, effective_energy)?;

        // 4. Determine duration based on energy amount
        let duration_hours = duration_for_energy(energy);

        Ok(EnergyEstimate {
            price: price_sun_out.to_string(),
            duration_hours,
        })
    }

    /// Query a TRON account's current staked energy from the chain.
    async fn fetch_account_energy(&self, address: &Address) -> AppResult<u64> {
        let url = format!(
            "{}/v2/account/energy?address={}",
            RENT_API_BASE,
            address.as_str()
        );
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::Unavailable(format!("account energy fetch: {e}")))?;
        if !resp.status().is_success() {
            return Err(AppError::Unavailable(format!(
                "account energy HTTP {}",
                resp.status()
            )));
        }
        let v: Value = resp
            .json()
            .await
            .map_err(|e| AppError::Unavailable(format!("account energy parse: {e}")))?;
        v.get("energy")
            .and_then(|e| e.as_u64())
            .or_else(|| v.get("totalEnergyWeight").and_then(|e| e.as_u64()))
            .ok_or_else(|| {
                AppError::Unavailable("account energy response missing energy field".into())
            })
    }

    /// Place an energy rental order via the marketplace.
    pub async fn order(
        &self,
        address: &Address,
        energy: u64,
        duration_hours: u64,
    ) -> AppResult<RentOrder> {
        // 1. Fetch available rental offers
        let offers = self.fetch_rental_offers(energy, duration_hours).await?;

        // 2. Select best offer (lowest price)
        let best = offers
            .into_iter()
            .min_by_key(|o| {
                o.get("price_sun")
                    .and_then(|p| p.as_u64())
                    .unwrap_or(u64::MAX)
            })
            .ok_or_else(|| AppError::Unavailable("no rental offers available".into()))?;

        let provider = best
            .get("provider")
            .and_then(|p| p.as_str())
            .unwrap_or("unknown");
        let price_sun = best.get("price_sun").and_then(|p| p.as_u64()).unwrap_or(0);

        // 3. Submit order to provider API
        let order_resp = self
            .submit_order(provider, address, energy, duration_hours, price_sun)
            .await?;

        let order_id = order_resp
            .get("order_id")
            .and_then(|o| o.as_str())
            .unwrap_or("")
            .to_string();

        Ok(RentOrder {
            order_id,
            status: "submitted".into(),
        })
    }

    /// Price per energy unit in TRX. Parsed as `Decimal` to keep money math
    /// exact; failures are surfaced instead of silently falling back.
    async fn fetch_energy_price(&self) -> AppResult<Decimal> {
        // TronScan / TronStake API for current energy resource pricing
        let url = format!("{}/v2/account/energy", RENT_API_BASE);
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::Unavailable(format!("energy price fetch: {e}")))?;
        if !resp.status().is_success() {
            return Err(AppError::Unavailable(format!(
                "energy price HTTP {}",
                resp.status()
            )));
        }
        let v: Value = resp
            .json()
            .await
            .map_err(|e| AppError::Unavailable(format!("energy price parse: {e}")))?;
        let raw = v
            .get("price_per_energy")
            .ok_or_else(|| AppError::Unavailable("energy price missing".into()))?;
        let price = if let Some(s) = raw.as_str() {
            Decimal::from_str_exact(s).map_err(|e| {
                AppError::Unavailable(format!("energy price parse decimal: {e}"))
            })?
        } else {
            raw.as_f64()
                .and_then(|f| Decimal::from_f64_retain(f))
                .ok_or_else(|| AppError::Unavailable("energy price invalid".into()))?
        };
        if price < Decimal::ZERO {
            return Err(AppError::Unavailable("energy price negative".into()));
        }
        Ok(price)
    }

    async fn fetch_rental_offers(&self, energy: u64, duration_hours: u64) -> AppResult<Vec<Value>> {
        // Query multiple rental providers
        let mut offers = Vec::new();

        // Provider 1: TronEnergy.io
        if let Ok(offer) = self.fetch_tron_energy_offer(energy, duration_hours).await {
            offers.push(offer);
        }

        // Provider 2: TRON Stake 2.0
        if let Ok(offer) = self.fetch_stake_offer(energy, duration_hours).await {
            offers.push(offer);
        }

        // If no real offers, return a synthetic one based on market rate
        if offers.is_empty() {
            let price_per_energy = self.fetch_energy_price().await?;
            let price_sun_out = price_sun(price_per_energy, energy)?;
            offers.push(json!({
                "provider": "market_average",
                "price_sun": price_sun_out,
                "energy": energy,
                "duration_hours": duration_hours,
            }));
        }

        Ok(offers)
    }

    async fn fetch_tron_energy_offer(&self, energy: u64, duration_hours: u64) -> AppResult<Value> {
        let url = "https://api.tronenergy.io/v1/offer";
        let body = json!({
            "energy": energy,
            "duration_hours": duration_hours,
        });
        let resp = self
            .http
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Unavailable(format!("tronenergy offer: {e}")))?;
        if resp.status().is_success() {
            let v: Value = resp
                .json()
                .await
                .map_err(|e| AppError::Unavailable(format!("tronenergy offer parse: {e}")))?;
            Ok(json!({
                "provider": "tronenergy",
                "price_sun": v.get("price_sun").and_then(|p| p.as_u64()).unwrap_or(0),
                "energy": energy,
                "duration_hours": duration_hours,
            }))
        } else {
            Err(AppError::Unavailable("tronenergy offer failed".into()))
        }
    }

    async fn fetch_stake_offer(&self, energy: u64, duration_hours: u64) -> AppResult<Value> {
        let url = "https://tronstake.io/api/v1/resource/energy/quote";
        let body = json!({
            "amount": energy,
            "period": duration_hours * 3600,
        });
        let resp = self
            .http
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Unavailable(format!("tronstake offer: {e}")))?;
        if resp.status().is_success() {
            let v: Value = resp
                .json()
                .await
                .map_err(|e| AppError::Unavailable(format!("tronstake offer parse: {e}")))?;
            Ok(json!({
                "provider": "tronstake",
                "price_sun": v.get("cost").and_then(|p| p.as_u64()).unwrap_or(0),
                "energy": energy,
                "duration_hours": duration_hours,
            }))
        } else {
            Err(AppError::Unavailable("tronstake offer failed".into()))
        }
    }

    async fn submit_order(
        &self,
        provider: &str,
        address: &Address,
        energy: u64,
        duration_hours: u64,
        price_sun: u64,
    ) -> AppResult<Value> {
        let url = match provider {
            "tronenergy" => "https://api.tronenergy.io/v1/order",
            "tronstake" => "https://tronstake.io/api/v1/order",
            _ => {
                return Err(AppError::InvalidArgument(format!(
                    "unknown rental provider: {provider}"
                )))
            }
        };

        let body = json!({
            "receiver": address.as_str(),
            "energy": energy,
            "duration_hours": duration_hours,
            "price_sun": price_sun,
        });

        let resp = self
            .http
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Unavailable(format!("rent order submit: {e}")))?;

        if resp.status().is_success() {
            resp.json()
                .await
                .map_err(|e| AppError::Unavailable(format!("rent order parse: {e}")))
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            Err(AppError::Unavailable(format!(
                "rent order HTTP {status}: {body}"
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_buckets_match_energy_ranges() {
        assert_eq!(duration_for_energy(0), 1);
        assert_eq!(duration_for_energy(10_000), 1);
        assert_eq!(duration_for_energy(10_001), 6);
        assert_eq!(duration_for_energy(100_000), 6);
        assert_eq!(duration_for_energy(100_001), 24);
        assert_eq!(duration_for_energy(1_000_000), 24);
        assert_eq!(duration_for_energy(1_000_001), 72);
    }

    #[test]
    fn price_sun_uses_exact_decimal_math() {
        // 0.000001 TRX per energy = 1 SUN per energy.
        let p = Decimal::from_str_exact("0.000001").unwrap();
        assert_eq!(price_sun(p, 1000).unwrap(), 1000);
        // 0.01 TRX per energy = 10_000 SUN per energy.
        let p = Decimal::from_str_exact("0.01").unwrap();
        assert_eq!(price_sun(p, 5).unwrap(), 50_000);
        // Rounds fractional SUN.
        let p = Decimal::from_str_exact("0.0000015").unwrap();
        assert_eq!(price_sun(p, 1).unwrap(), 2);
    }

    #[test]
    fn price_sun_rejects_zero_energy_price_overflow() {
        let p = Decimal::from_str_exact("9999999999999999999999").unwrap();
        assert!(price_sun(p, u64::MAX).is_err());
    }
}
