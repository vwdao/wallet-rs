use dashmap::DashMap;
use salvo::prelude::*;
use std::time::Instant;
use wallet_db::{ChainGatewayKey, Db, Network};
use wallet_error::AppError;

use crate::Gw;

pub struct ProxyAuth {
    pub api_key: String,
    pub chain_index: i64,
    pub chain_name: String,
    pub key_row: ChainGatewayKey,
}

pub async fn authenticate(req: &Request, st: &Gw) -> Result<ProxyAuth, AppError> {
    let cfg = st.settings.get();

    let api_key = extract_api_key(req)?;

    if cfg.global_rate_limit_per_min > 0 {
        check_rate(&st.rate, "_global", cfg.global_rate_limit_per_min)?;
    }

    let mut db = st.db.clone_inner();
    let key_row = ChainGatewayKey::filter_by_api_key(&api_key)
        .get(&mut db)
        .await
        .map_err(|_| AppError::Forbidden)?;

    if !key_row.enabled {
        return Err(AppError::Forbidden);
    }

    check_rate(
        &st.rate,
        &api_key,
        key_row.rate_limit_per_min.max(0) as usize,
    )?;

    let chain_name: String = req
        .param("chain")
        .ok_or_else(|| AppError::InvalidArgument("missing chain".into()))?;

    let chain_index = resolve_chain_index(&st.db, &chain_name).await?;

    if !key_row.allowed_chains.is_empty() && !key_row.allowed_chains.contains(&chain_index) {
        return Err(AppError::Forbidden);
    }

    Ok(ProxyAuth {
        api_key,
        chain_index,
        chain_name,
        key_row,
    })
}

pub fn extract_api_key(req: &Request) -> Result<String, AppError> {
    if let Some(key) = req.param::<String>("api_key") {
        return Ok(key);
    }
    req.headers()
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or(AppError::Unauthorized)
}

pub async fn resolve_chain_index(db: &Db, chain: &str) -> Result<i64, AppError> {
    let mut db = db.clone_inner();
    let networks: Vec<Network> = Network::filter(Network::fields().enabled().eq(true))
        .exec(&mut db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

    networks
        .into_iter()
        .find(|n| n.name.eq_ignore_ascii_case(chain))
        .map(|n| n.chain_index)
        .ok_or_else(|| AppError::InvalidArgument(format!("unknown chain: {chain}")))
}

pub fn check_rate(
    map: &DashMap<String, Vec<Instant>>,
    key: &str,
    limit: usize,
) -> Result<(), AppError> {
    if limit == 0 {
        return Ok(());
    }
    let now = Instant::now();
    let should_cleanup = map.len() > 1000;
    {
        let mut entry = map.entry(key.to_string()).or_default();
        entry.retain(|t| now.duration_since(*t) < std::time::Duration::from_secs(60));
        if entry.len() >= limit {
            return Err(AppError::TooManyRequests);
        }
        entry.push(now);
    }
    if should_cleanup {
        map.retain(|_, v| {
            !v.is_empty()
                && v.iter()
                    .any(|t| now.duration_since(*t) < std::time::Duration::from_secs(120))
        });
    }
    Ok(())
}

pub fn validate_endpoint_url(url: &str) -> Result<(), AppError> {
    let (_, parsed) = crate::protocol::parse_endpoint_url(url)?;
    if parsed.host_str().is_none() {
        return Err(AppError::InvalidArgument("url missing host".into()));
    }
    Ok(())
}
