use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::task::JoinSet;
use url::Url;
use wallet_db::{Network, RpcEndpoint};
use wallet_error::{AppError, AppResult};

const CHAINLIST_RPCS_URL: &str = "https://chainlist.org/rpcs.json";
const DEFAULT_ENDPOINT_WEIGHT: i32 = 1;
const DEFAULT_ENDPOINT_PRIORITY: i32 = 10;

#[derive(Debug, Clone, Serialize)]
pub struct ChainSyncResult {
    pub chain_index: i64,
    pub family: String,
    pub source: String,
    pub discovered: usize,
    pub inserted: usize,
    pub updated: usize,
    pub skipped: usize,
    pub message: Option<String>,
}

#[derive(Clone)]
pub struct FreeRpcSyncer {
    db: wallet_db::Db,
    http: Client,
}

impl FreeRpcSyncer {
    pub fn new(db: wallet_db::Db, http: Client) -> Self {
        Self { db, http }
    }

    pub async fn sync_all_enabled(&self) -> AppResult<Vec<ChainSyncResult>> {
        let mut db = self.db.clone_inner();
        let networks: Vec<Network> = Network::filter(Network::fields().enabled().eq(true))
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        let mut results = Vec::with_capacity(networks.len());
        for network in networks {
            results.push(self.sync_network(&network).await?);
        }
        Ok(results)
    }

    pub async fn sync_chain(&self, chain_index: i64) -> AppResult<ChainSyncResult> {
        let mut db = self.db.clone_inner();
        let rows: Vec<Network> = Network::filter(Network::fields().chain_index().eq(chain_index))
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;
        let network = rows
            .into_iter()
            .next()
            .ok_or_else(|| AppError::NotFound(format!("network {chain_index} not found")))?;
        self.sync_network(&network).await
    }

    async fn sync_network(&self, network: &Network) -> AppResult<ChainSyncResult> {
        match network.family.as_str() {
            "evm" => self.sync_evm_network(network).await,
            "solana" => self.sync_solana_network(network).await,
            _ => Ok(ChainSyncResult {
                chain_index: network.chain_index,
                family: network.family.clone(),
                source: "unsupported".into(),
                discovered: 0,
                inserted: 0,
                updated: 0,
                skipped: 0,
                message: Some("当前仅自动导入 EVM 和 Solana 的免费 RPC".into()),
            }),
        }
    }

    async fn sync_evm_network(&self, network: &Network) -> AppResult<ChainSyncResult> {
        let evm_chain_id = network.evm_chain_id.ok_or_else(|| {
            AppError::InvalidArgument(format!(
                "network {} missing evm_chain_id",
                network.chain_index
            ))
        })?;

        let payload = self
            .http
            .get(CHAINLIST_RPCS_URL)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| AppError::Unavailable(format!("fetch chainlist failed: {e}")))?;

        if !payload.status().is_success() {
            return Err(AppError::Unavailable(format!(
                "fetch chainlist failed with status {}",
                payload.status()
            )));
        }

        let chains: Vec<ChainListChain> = payload
            .json()
            .await
            .map_err(|e| AppError::Unavailable(format!("decode chainlist failed: {e}")))?;

        let chain = chains
            .into_iter()
            .find(|item| item.chain_id == evm_chain_id as u64)
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "chainlist entry not found for evm_chain_id {evm_chain_id}"
                ))
            })?;

        let candidates = chain.public_https_urls();
        if candidates.is_empty() {
            return Ok(ChainSyncResult {
                chain_index: network.chain_index,
                family: network.family.clone(),
                source: "chainlist".into(),
                discovered: 0,
                inserted: 0,
                updated: 0,
                skipped: 0,
                message: Some("公开列表里没有找到可直接使用的 HTTPS 免费 RPC".into()),
            });
        }

        let verified =
            verify_evm_candidates(self.http.clone(), candidates, evm_chain_id as u64).await;
        self.upsert_endpoints(network, "chainlist", verified, None)
            .await
    }

    async fn sync_solana_network(&self, network: &Network) -> AppResult<ChainSyncResult> {
        let candidates = default_solana_rpcs(network.chain_index);
        if candidates.is_empty() {
            return Ok(ChainSyncResult {
                chain_index: network.chain_index,
                family: network.family.clone(),
                source: "builtin".into(),
                discovered: 0,
                inserted: 0,
                updated: 0,
                skipped: 0,
                message: Some("当前没有该 Solana 网络的内置免费 RPC".into()),
            });
        }

        let verified = verify_solana_candidates(self.http.clone(), candidates).await;
        self.upsert_endpoints(network, "builtin", verified, None)
            .await
    }

    async fn upsert_endpoints(
        &self,
        network: &Network,
        source: &str,
        urls: Vec<String>,
        message: Option<String>,
    ) -> AppResult<ChainSyncResult> {
        let mut inserted = 0usize;
        let mut updated = 0usize;
        let mut skipped = 0usize;

        for url in urls.iter() {
            let mut db = self.db.clone_inner();
            let rows: Vec<RpcEndpoint> = RpcEndpoint::filter(
                RpcEndpoint::fields()
                    .chain_index()
                    .eq(network.chain_index)
                    .and(RpcEndpoint::fields().url().eq(url.as_str())),
            )
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

            if let Some(mut existing) = rows.into_iter().next() {
                let weight = existing.weight.max(DEFAULT_ENDPOINT_WEIGHT);
                let priority = existing.priority.max(DEFAULT_ENDPOINT_PRIORITY);
                existing
                    .update()
                    .enabled(true)
                    .healthy(true)
                    .tier("free")
                    .weight(weight)
                    .priority(priority)
                    .error_count(0)
                    .exec(&mut db)
                    .await
                    .map_err(|e| AppError::internal(e.to_string()))?;
                updated += 1;
            } else {
                toasty::create!(RpcEndpoint {
                    chain_index: network.chain_index,
                    url,
                    weight: DEFAULT_ENDPOINT_WEIGHT,
                    enabled: true,
                    tier: "free",
                    priority: DEFAULT_ENDPOINT_PRIORITY,
                })
                .exec(&mut db)
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;
                inserted += 1;
            }
        }

        if inserted == 0 && updated == 0 {
            skipped = urls.len();
        }

        Ok(ChainSyncResult {
            chain_index: network.chain_index,
            family: network.family.clone(),
            source: source.into(),
            discovered: urls.len(),
            inserted,
            updated,
            skipped,
            message,
        })
    }
}

#[derive(Debug, Deserialize)]
struct ChainListChain {
    #[serde(rename = "chainId")]
    chain_id: u64,
    rpc: Vec<ChainListRpc>,
}

impl ChainListChain {
    fn public_https_urls(&self) -> Vec<String> {
        let mut urls = Vec::new();
        for rpc in &self.rpc {
            let (candidate, tracking) = rpc.as_parts();
            if is_acceptable_chainlist_rpc(candidate, tracking) {
                if let Some(normalized) = normalize_rpc_url(candidate) {
                    urls.push(normalized);
                }
            }
        }
        urls.sort();
        urls.dedup();
        urls
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ChainListRpc {
    Simple(String),
    Detailed(ChainListRpcDetails),
}

impl ChainListRpc {
    fn as_parts(&self) -> (&str, Option<&str>) {
        match self {
            Self::Simple(url) => (url.as_str(), None),
            Self::Detailed(details) => (details.url.as_str(), details.tracking.as_deref()),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ChainListRpcDetails {
    url: String,
    #[serde(default)]
    tracking: Option<String>,
}

async fn verify_evm_candidates(
    http: Client,
    urls: Vec<String>,
    expected_chain_id: u64,
) -> Vec<String> {
    let mut tasks = JoinSet::new();
    for url in urls {
        let client = http.clone();
        tasks.spawn(async move {
            if verify_evm_endpoint(client, &url, expected_chain_id).await {
                Some(url)
            } else {
                None
            }
        });
    }

    let mut verified = Vec::new();
    while let Some(result) = tasks.join_next().await {
        if let Ok(Some(url)) = result {
            verified.push(url);
        }
    }
    verified.sort();
    verified.dedup();
    verified
}

async fn verify_solana_candidates(http: Client, urls: Vec<String>) -> Vec<String> {
    let mut tasks = JoinSet::new();
    for url in urls {
        let client = http.clone();
        tasks.spawn(async move {
            if verify_solana_endpoint(client, &url).await {
                Some(url)
            } else {
                None
            }
        });
    }

    let mut verified = Vec::new();
    while let Some(result) = tasks.join_next().await {
        if let Ok(Some(url)) = result {
            verified.push(url);
        }
    }
    verified.sort();
    verified.dedup();
    verified
}

async fn verify_evm_endpoint(http: Client, url: &str, expected_chain_id: u64) -> bool {
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_chainId",
        "params": [],
    });
    let Ok(resp) = http
        .post(url)
        .json(&body)
        .timeout(std::time::Duration::from_secs(4))
        .send()
        .await
    else {
        return false;
    };

    if !resp.status().is_success() {
        return false;
    }

    let Ok(value) = resp.json::<serde_json::Value>().await else {
        return false;
    };
    let Some(chain_id_hex) = value.get("result").and_then(|v| v.as_str()) else {
        return false;
    };
    let parsed = chain_id_hex.trim_start_matches("0x");
    u64::from_str_radix(parsed, 16)
        .map(|chain_id| chain_id == expected_chain_id)
        .unwrap_or(false)
}

async fn verify_solana_endpoint(http: Client, url: &str) -> bool {
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getSlot",
        "params": crate::health::probe_params_for_chain("solana"),
    });
    let Ok(resp) = http
        .post(url)
        .json(&body)
        .timeout(std::time::Duration::from_secs(4))
        .send()
        .await
    else {
        return false;
    };

    if !resp.status().is_success() {
        return false;
    }

    let Ok(value) = resp.json::<serde_json::Value>().await else {
        return false;
    };
    value.get("result").and_then(|v| v.as_i64()).is_some()
}

fn default_solana_rpcs(chain_index: i64) -> Vec<String> {
    let urls = match chain_index {
        501 => vec![
            "https://api.mainnet.solana.com",
            "https://solana-rpc.publicnode.com",
        ],
        _ => Vec::new(),
    };

    urls.into_iter()
        .filter_map(normalize_rpc_url)
        .collect::<Vec<_>>()
}

fn is_acceptable_chainlist_rpc(url: &str, tracking: Option<&str>) -> bool {
    let lower = url.to_ascii_lowercase();
    if !lower.starts_with("https://") {
        return false;
    }
    if lower.contains("${")
        || lower.contains("your-api-key")
        || lower.contains("your_api_key")
        || lower.contains("{api_key}")
        || lower.contains("apikey=")
        || lower.contains("api_key=")
    {
        return false;
    }
    if let Some(mode) = tracking {
        if !mode.eq_ignore_ascii_case("none") {
            return false;
        }
    }
    normalize_rpc_url(url).is_some()
}

fn normalize_rpc_url(input: &str) -> Option<String> {
    let mut parsed = Url::parse(input).ok()?;
    if parsed.scheme() != "https" {
        return None;
    }
    parsed.set_fragment(None);
    let normalized = parsed.to_string();
    Some(normalized.trim_end_matches('/').to_string())
}

#[cfg(test)]
mod tests {
    use super::{is_acceptable_chainlist_rpc, normalize_rpc_url};

    #[test]
    fn normalize_rpc_url_removes_trailing_slash_and_fragment() {
        let normalized = normalize_rpc_url("https://ethereum-rpc.publicnode.com/#mainnet").unwrap();
        assert_eq!(normalized, "https://ethereum-rpc.publicnode.com");
    }

    #[test]
    fn reject_non_https_or_keyed_urls() {
        assert!(!is_acceptable_chainlist_rpc(
            "wss://ethereum-rpc.publicnode.com",
            Some("none")
        ));
        assert!(!is_acceptable_chainlist_rpc(
            "https://rpc.ankr.com/eth/${API_KEY}",
            Some("none")
        ));
        assert!(!is_acceptable_chainlist_rpc(
            "https://example.com/rpc?api_key=demo",
            Some("none")
        ));
    }

    #[test]
    fn accept_plain_public_https_urls() {
        assert!(is_acceptable_chainlist_rpc(
            "https://eth.llamarpc.com",
            Some("none")
        ));
        assert!(is_acceptable_chainlist_rpc(
            "https://ethereum-rpc.publicnode.com",
            None
        ));
    }
}
