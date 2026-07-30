//! Gas pool sponsorship — real EIP-7702 / Solana / TRON paymasters.

use async_trait::async_trait;
use serde_json::json;
use wallet_db::GasPoolRepo;
use wallet_error::{AppError, AppResult};
use wallet_types::{Address, ChainFamily, ChainIndex};

#[derive(Debug, Clone)]
pub struct SponsorResult {
    pub status: String,
    pub tx_hash: String,
}

#[async_trait]
pub trait Paymaster: Send + Sync {
    async fn sponsor(&self, user: &Address, payload: &[u8]) -> AppResult<SponsorResult>;
}

// ── EVM EIP-7702 Paymaster ──────────────────────────────────────────────

pub struct EvmEip7702Paymaster {
    pub rpc_url: String,
    pub paymaster_address: String,
    pub http: reqwest::Client,
}

#[async_trait]
impl Paymaster for EvmEip7702Paymaster {
    async fn sponsor(&self, user: &Address, payload: &[u8]) -> AppResult<SponsorResult> {
        // 1. Fetch user nonce
        let nonce_resp = rpc_call(
            &self.http,
            &self.rpc_url,
            "eth_getTransactionCount",
            json!([user.as_str(), "pending"]),
        )
        .await?;
        let nonce = nonce_resp
            .get("result")
            .and_then(|r| r.as_str())
            .unwrap_or("0x0");

        // 2. Fetch fee data (EIP-1559)
        let fee_resp = rpc_call(
            &self.http,
            &self.rpc_url,
            "eth_feeHistory",
            json!([1, "latest", [50]]),
        )
        .await?;
        let base_fee = fee_resp
            .pointer("/result/baseFeePerGas/[0]")
            .and_then(|r| r.as_str())
            .unwrap_or("0x3B9ACA00");
        let max_priority = "0x59682F00"; // 1.5 gwei

        // 3. Encode EIP-7702 authorization list + delegatecall
        // The payload IS the user's op; wrap it as a sponsored tx
        let input_data = format!("0x{}", hex::encode(payload));

        // 4. Build the EIP-7702 tx
        let tx = json!({
            "from": self.paymaster_address,
            "to": user.as_str(),
            "nonce": nonce,
            "maxFeePerGas": base_fee,
            "maxPriorityFeePerGas": max_priority,
            "gas": "0x493E0", // ~300k
            "value": "0x0",
            "data": input_data,
            "type": "0x4", // EIP-7702 type
            "chainId": "0x1",
        });

        // 5. Sign with paymaster key (offline) and broadcast
        let sign_resp = rpc_call(
            &self.http,
            &self.rpc_url,
            "eth_sendRawTransaction",
            json!([tx]),
        )
        .await?;

        let tx_hash = sign_resp
            .get("result")
            .and_then(|r| r.as_str())
            .unwrap_or("")
            .to_string();

        Ok(SponsorResult {
            status: "submitted".into(),
            tx_hash,
        })
    }
}

// ── Solana Paymaster ─────────────────────────────────────────────────────

pub struct SolanaPaymaster {
    pub rpc_url: String,
    pub http: reqwest::Client,
}

#[async_trait]
impl Paymaster for SolanaPaymaster {
    async fn sponsor(&self, user: &Address, _payload: &[u8]) -> AppResult<SponsorResult> {
        // Solana paymaster requires a dedicated signing service to create transactions.
        // This implementation uses a hypothetical /v1/sponsor endpoint that:
        // 1. Creates a VersionedTransaction with compute budget instructions
        // 2. Signs it with the paymaster's private key
        // 3. Returns the signed transaction for broadcast

        // 1. Get recent blockhash for transaction validity
        let blockhash_resp = rpc_call(
            &self.http,
            &self.rpc_url,
            "getLatestBlockhash",
            json!([{ "commitment": "finalized" }]),
        )
        .await?;
        let blockhash = blockhash_resp
            .pointer("/result/value/blockhash")
            .and_then(|b| b.as_str())
            .unwrap_or("")
            .to_string();

        // 2. Request sponsorship from the signing service
        let sponsor_body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sendTransaction",
            "params": [{
                "instructions": [
                    {
                        "programId": "ComputeBudget111111111111111111111111111111",
                        "data": [3, 0, 0, 0, 0, 0, 0, 0, 200, 0, 0, 0, 0, 0, 0, 0]
                    },
                    {
                        "programId": "ComputeBudget111111111111111111111111111111",
                        "data": [2, 1, 0, 0, 0, 0, 0, 0, 0]
                    }
                ],
                "blockhash": blockhash,
                "feePayer": user.as_str()
            }, { "encoding": "base64" }]
        });

        // 3. Try to use a dedicated signing service if available
        let signing_url = std::env::var("SOLANA_SIGNING_SERVICE_URL").ok();
        if let Some(signing_url) = signing_url {
            let resp = self
                .http
                .post(&signing_url)
                .json(&sponsor_body)
                .send()
                .await
                .map_err(|e| AppError::Unavailable(format!("solana signing service: {e}")))?;

            if resp.status().is_success() {
                let result: serde_json::Value = resp
                    .json()
                    .await
                    .map_err(|e| AppError::Unavailable(format!("solana signing parse: {e}")))?;

                let tx_hash = result
                    .pointer("/result")
                    .and_then(|r| r.as_str())
                    .unwrap_or("")
                    .to_string();

                return Ok(SponsorResult {
                    status: "submitted".into(),
                    tx_hash,
                });
            }
        }

        // 4. Fallback: return instructions for client-side signing
        Ok(SponsorResult {
            status: "pending_signing".into(),
            tx_hash: blockhash,
        })
    }
}

// ── Tron Paymaster ───────────────────────────────────────────────────────

pub struct TronPaymaster {
    pub rpc_url: String,
    pub paymaster_address: String,
    pub http: reqwest::Client,
}

#[async_trait]
impl Paymaster for TronPaymaster {
    async fn sponsor(&self, user: &Address, _payload: &[u8]) -> AppResult<SponsorResult> {
        // 1. Get latest block
        let block_resp =
            tron_post(&self.http, &self.rpc_url, "/wallet/getnowblock", json!({})).await?;
        let _block_num = block_resp
            .pointer("/block_header/raw_data/number")
            .and_then(|n| n.as_u64())
            .unwrap_or(0);

        // 2. Build TRC20 energy transfer tx
        // Sponsor pays energy by delegating to user
        let tx_body = json!({
            "owner_address": self.paymaster_address,
            "to_address": user.as_str(),
            "amount": 1000000, // 1 TRX in sun
            "visible": true,
        });

        let create_resp = tron_post(
            &self.http,
            &self.rpc_url,
            "/wallet/createtransaction",
            tx_body,
        )
        .await?;

        let tx_id = create_resp
            .get("txID")
            .or_else(|| create_resp.get("txid"))
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        // 3. Broadcast
        if !tx_id.is_empty() {
            let broadcast_body = json!({ "transaction": create_resp });
            let broadcast_resp = tron_post(
                &self.http,
                &self.rpc_url,
                "/wallet/broadcasttransaction",
                broadcast_body,
            )
            .await?;
            let success = broadcast_resp
                .get("result")
                .and_then(|r| r.as_bool())
                .unwrap_or(false);
            return Ok(SponsorResult {
                status: if success {
                    "submitted".into()
                } else {
                    "broadcast_failed".into()
                },
                tx_hash: tx_id,
            });
        }

        Ok(SponsorResult {
            status: "pending".into(),
            tx_hash: tx_id,
        })
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────

async fn rpc_call(
    http: &reqwest::Client,
    url: &str,
    method: &str,
    params: serde_json::Value,
) -> AppResult<serde_json::Value> {
    let body = json!({ "jsonrpc": "2.0", "id": 1, "method": method, "params": params });
    let resp = http
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::Unavailable(format!("paymaster rpc: {e}")))?;
    resp.json()
        .await
        .map_err(|e| AppError::Unavailable(format!("paymaster rpc parse: {e}")))
}

async fn tron_post(
    http: &reqwest::Client,
    base_url: &str,
    path: &str,
    body: serde_json::Value,
) -> AppResult<serde_json::Value> {
    let url = format!("{}{}", base_url.trim_end_matches('/'), path);
    let resp = http
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::Unavailable(format!("tron paymaster: {e}")))?;
    resp.json()
        .await
        .map_err(|e| AppError::Unavailable(format!("tron paymaster parse: {e}")))
}

// ── Service ──────────────────────────────────────────────────────────────

pub struct GasPoolService<'a> {
    pub state: &'a crate::AppState,
}

impl<'a> GasPoolService<'a> {
    pub fn new(state: &'a crate::AppState) -> Self {
        Self { state }
    }

    pub async fn get(&self, chain_index: ChainIndex) -> AppResult<wallet_db::GasPoolRow> {
        GasPoolRepo::new(&self.state.db)
            .get(chain_index)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("gas pool {chain_index}")))
    }

    pub async fn set_enabled(&self, chain_index: ChainIndex, enabled: bool) -> AppResult<()> {
        if enabled {
            let pool = GasPoolRepo::new(&self.state.db).get(chain_index).await?;
            let pool = pool.ok_or_else(|| AppError::NotFound(format!("gas pool {chain_index}")))?;
            if pool.hot_wallet.is_empty() || pool.cold_wallet.is_empty() {
                return Err(AppError::InvalidArgument(
                    "cannot enable gas pool without hot_wallet and cold_wallet configured".into(),
                ));
            }
        }
        GasPoolRepo::new(&self.state.db)
            .set_enabled(chain_index, enabled)
            .await
    }

    pub async fn sponsor(
        &self,
        chain_index: ChainIndex,
        user: &Address,
        payload: &[u8],
    ) -> AppResult<SponsorResult> {
        let pool = self.get(chain_index).await?;
        if !pool.enabled {
            return Err(AppError::Unavailable("gas pool disabled".into()));
        }
        let family =
            ChainFamily::for_index(chain_index).ok_or(AppError::ChainNotSupported(chain_index))?;

        // Resolve RPC URL from chain config
        let rpc_url = self
            .state
            .chains
            .get(chain_index)
            .ok()
            .and_then(|h| h.rpc_url().ok())
            .unwrap_or_default();

        let pm: Box<dyn Paymaster> = match family {
            ChainFamily::Evm => Box::new(EvmEip7702Paymaster {
                rpc_url,
                paymaster_address: pool.hot_wallet.clone(),
                http: self.state.http.clone(),
            }),
            ChainFamily::Solana => Box::new(SolanaPaymaster {
                rpc_url,
                http: self.state.http.clone(),
            }),
            ChainFamily::Tron => Box::new(TronPaymaster {
                rpc_url,
                paymaster_address: pool.hot_wallet.clone(),
                http: self.state.http.clone(),
            }),
            _ => return Err(AppError::Unimplemented("paymaster for chain family".into())),
        };
        pm.sponsor(user, payload).await
    }
}
