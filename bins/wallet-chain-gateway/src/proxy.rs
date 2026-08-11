use dashmap::DashMap;
use ipnet::IpNet;
use salvo::prelude::*;
use std::net::IpAddr;
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

    let client_ip = req.remote_addr().ip().map(|ip| ip.to_string());
    check_ip_policy(&key_row, client_ip.as_deref())?;

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

pub fn validate_ton_endpoint_url(url: &str) -> Result<(), AppError> {
    validate_endpoint_url(url)?;
    let lower = url.to_ascii_lowercase();
    if !lower.contains("/jsonrpc") {
        return Err(AppError::InvalidArgument(format!(
            "TON endpoint must be a toncenter v2 JSON-RPC URL (path must contain /jsonRPC, e.g. https://toncenter.com/api/v2/jsonRPC); toncenter v3 is a REST API and is not supported: {url}"
        )));
    }
    Ok(())
}

/// Validate that every entry is a plain IP address or a CIDR block.
pub fn validate_ip_list(entries: &[String]) -> Result<(), AppError> {
    for entry in entries {
        let trimmed = entry.trim();
        if trimmed.is_empty() {
            return Err(AppError::InvalidArgument(
                "IP list contains an empty entry".into(),
            ));
        }
        if parse_ip_entry(trimmed).is_none() {
            return Err(AppError::InvalidArgument(format!(
                "invalid IP or CIDR entry: '{trimmed}'"
            )));
        }
    }
    Ok(())
}

/// Check a client IP against a key's whitelist/blacklist.
///
/// Semantics:
/// - whitelist non-empty  -> the IP MUST match one of the entries.
/// - blacklist            -> the IP MUST NOT match any of the entries.
/// - entries are either plain IPs (v4/v6) or CIDR blocks.
/// - when the client IP cannot be determined, a non-empty whitelist fails closed.
pub fn check_ip_policy(
    key_row: &ChainGatewayKey,
    client_ip: Option<&str>,
) -> Result<(), AppError> {
    if key_row.ip_whitelist.is_empty() && key_row.ip_blacklist.is_empty() {
        return Ok(());
    }

    let ip: IpAddr = match client_ip {
        Some(raw) => raw
            .parse()
            .map_err(|_| AppError::Forbidden)?,
        None => return Err(AppError::Forbidden),
    };

    if !key_row.ip_whitelist.is_empty()
        && !key_row
            .ip_whitelist
            .iter()
            .any(|entry| ip_matches(entry, &ip))
    {
        return Err(AppError::Forbidden);
    }

    if key_row
        .ip_blacklist
        .iter()
        .any(|entry| ip_matches(entry, &ip))
    {
        return Err(AppError::Forbidden);
    }

    Ok(())
}

fn parse_ip_entry(entry: &str) -> Option<IpNet> {
    let trimmed = entry.trim();
    if let Ok(ip) = trimmed.parse::<IpAddr>() {
        return Some(IpNet::from(ip));
    }
    trimmed.parse::<IpNet>().ok()
}

fn ip_matches(entry: &str, ip: &IpAddr) -> bool {
    match parse_ip_entry(entry) {
        Some(net) => net.contains(ip),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{check_ip_policy, ip_matches, validate_ip_list};
    use std::net::IpAddr;
    use wallet_db::ChainGatewayKey;

    fn key_with(whitelist: Vec<String>, blacklist: Vec<String>) -> ChainGatewayKey {
        ChainGatewayKey {
            id: uuid::Uuid::new_v4(),
            api_key: "gw_test".into(),
            name: "test".into(),
            rate_limit_per_min: 60,
            enabled: true,
            allowed_chains: Vec::new(),
            allowed_tier: "all".into(),
            ip_whitelist: whitelist,
            ip_blacklist: blacklist,
            total_requests: 0,
            created_at: jiff::Timestamp::now(),
        }
    }

    #[test]
    fn ip_matches_exact_and_cidr() {
        let ip: IpAddr = "10.1.2.3".parse().unwrap();
        assert!(ip_matches("10.1.2.3", &ip));
        assert!(ip_matches("10.0.0.0/8", &ip));
        assert!(!ip_matches("11.0.0.0/8", &ip));
        assert!(!ip_matches("not-an-ip", &ip));
    }

    #[test]
    fn no_lists_always_allows() {
        let key = key_with(vec![], vec![]);
        assert!(check_ip_policy(&key, Some("1.2.3.4")).is_ok());
        assert!(check_ip_policy(&key, None).is_ok());
    }

    #[test]
    fn whitelist_allows_only_matching() {
        let key = key_with(vec!["192.168.1.0/24".into()], vec![]);
        assert!(check_ip_policy(&key, Some("192.168.1.10")).is_ok());
        assert!(check_ip_policy(&key, Some("10.0.0.1")).is_err());
        assert!(check_ip_policy(&key, None).is_err());
    }

    #[test]
    fn blacklist_rejects_matching() {
        let key = key_with(vec![], vec!["203.0.113.7".into()]);
        assert!(check_ip_policy(&key, Some("203.0.113.7")).is_err());
        assert!(check_ip_policy(&key, Some("203.0.113.8")).is_ok());
    }

    #[test]
    fn blacklist_wins_over_whitelist() {
        let key = key_with(
            vec!["10.0.0.0/8".into()],
            vec!["10.0.0.5".into()],
        );
        assert!(check_ip_policy(&key, Some("10.0.0.5")).is_err());
        assert!(check_ip_policy(&key, Some("10.0.0.6")).is_ok());
    }

    #[test]
    fn ipv6_cidr_matches() {
        let key = key_with(vec!["2001:db8::/32".into()], vec![]);
        assert!(check_ip_policy(&key, Some("2001:db8::1")).is_ok());
        assert!(check_ip_policy(&key, Some("2001:db9::1")).is_err());
    }

    #[test]
    fn validate_ip_list_rejects_garbage() {
        assert!(validate_ip_list(&["10.0.0.0/8".to_string()]).is_ok());
        assert!(validate_ip_list(&["nope".to_string()]).is_err());
        assert!(validate_ip_list(&["".to_string()]).is_err());
        assert!(validate_ip_list(&["10.0.0.0/8".to_string(), "300.1.1.1".to_string()]).is_err());
    }
}
