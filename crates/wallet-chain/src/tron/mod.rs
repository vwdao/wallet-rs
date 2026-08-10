use crate::tron_grpc::client::TronGrpcClient;
use crate::tron_grpc::url::parse_gateway_grpc_url;
use crate::provider::RpcPool;
use crate::traits::{BalanceReader, BlockSource, GasEstimator, TokenBalance, TxBroadcaster};
use async_trait::async_trait;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use wallet_config::ChainRuntimeConfig;
use wallet_error::{AppError, AppResult};
use wallet_types::{
    Address, Amount, ChainIndex, GasEstimate, GasEstimateRequest, NormalizedTx, TxHash, TxStatus,
};

#[derive(Debug)]
pub struct TronChain {
    pub chain_index: ChainIndex,
    pub pool: RpcPool,
    /// Per-endpoint typed gRPC clients keyed by the original `grpc(s)://...`
    /// URL from `cfg.endpoints`. Endpoints that aren't in the chain-gateway
    /// gRPC format have no entry and fall through to the HTTP path.
    grpc_clients: HashMap<String, Arc<TronGrpcClient>>,
}

impl TronChain {
    pub fn new(cfg: &ChainRuntimeConfig) -> AppResult<Arc<Self>> {
        let mut grpc_clients = HashMap::new();
        for ep in &cfg.endpoints {
            if let Some(gw) = parse_gateway_grpc_url(&ep.url) {
                let client = TronGrpcClient::connect(&gw, Duration::from_secs(30))?;
                grpc_clients.insert(ep.url.clone(), Arc::new(client));
            }
        }
        Ok(Arc::new(Self {
            chain_index: ChainIndex(cfg.chain_index),
            pool: RpcPool::new(cfg.endpoints.clone())?,
            grpc_clients,
        }))
    }

    /// Send `body` to `path` over HTTP using the chain-gateway (or another
    /// HTTP URL). Returns the parsed JSON value. HTTP errors mark the URL
    /// as failed in the [`RpcPool`] circuit breaker.
    async fn post_http(&self, url: &str, path: &str, body: Value) -> AppResult<Value> {
        let full_url = format!("{}{}", url.trim_end_matches('/'), path);
        let resp = self
            .pool
            .client()
            .post(&full_url)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                self.pool.mark_failure(url);
                AppError::Unavailable(e.to_string())
            })?;
        let v: Value = resp.json().await.map_err(|e| {
            self.pool.mark_failure(url);
            AppError::Unavailable(e.to_string())
        })?;
        self.pool.mark_success(url);
        Ok(v)
    }

    /// Send a TRON JSON-RPC method via the gRPC proxy of the chain-gateway.
    /// `body` is the same JSON the HTTP path would have sent — its only use
    /// here is to pick the right typed `WalletClient` method (the actual
    /// request body is a protobuf, built from the typed args).
    async fn post_grpc(
        &self,
        url: &str,
        client: Arc<TronGrpcClient>,
        path: &str,
        body: Value,
    ) -> AppResult<Value> {
        let gw = parse_gateway_grpc_url(url)
            .ok_or_else(|| AppError::internal("invalid chain-gateway gRPC url"))?;
        if !gw.chain.eq_ignore_ascii_case("tron") {
            return Err(AppError::InvalidArgument(format!(
                "tron gRPC client cannot serve chain '{}'",
                gw.chain
            )));
        }
        let result = self.call_grpc_by_path(client.as_ref(), path, &body).await;
        match result {
            Ok(v) => {
                self.pool.mark_success(url);
                Ok(v)
            }
            Err(e) => {
                self.pool.mark_failure(url);
                Err(e)
            }
        }
    }

    /// Route a JSON-RPC path + body to the right typed `WalletClient` method.
    /// The path strings come from the existing `BlockSource`/`TxBroadcaster`
    /// callers; only the methods they use are wired up.
    async fn call_grpc_by_path(
        &self,
        client: &TronGrpcClient,
        path: &str,
        body: &Value,
    ) -> AppResult<Value> {
        let trimmed = path.trim_start_matches('/');
        let bare = trimmed
            .strip_prefix("wallet/")
            .or_else(|| trimmed.strip_prefix("Wallet/"))
            .unwrap_or(trimmed);
        match bare {
            "getnowblock" => client.get_now_block().await,
            "getblockbynum" => {
                let num = body
                    .get("num")
                    .and_then(|n| n.as_i64())
                    .ok_or_else(|| AppError::InvalidArgument("getblockbynum: missing num".into()))?;
                client.get_block_by_num(num).await
            }
            "gettransactioninfobyid" => {
                let value = body
                    .get("value")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        AppError::InvalidArgument("gettransactioninfobyid: missing value".into())
                    })?;
                let value = hex::decode(value.trim_start_matches("0x"))
                    .map_err(|e| AppError::InvalidArgument(format!("invalid txid hex: {e}")))?;
                client.get_transaction_info_by_id(value).await
            }
            "gettransactioninfobyblocknum" => {
                let num = body
                    .get("num")
                    .and_then(|n| n.as_i64())
                    .ok_or_else(|| {
                        AppError::InvalidArgument(
                            "gettransactioninfobyblocknum: missing num".into(),
                        )
                    })?;
                client.get_transaction_info_by_block_num(num).await
            }
            "getaccount" => {
                let address_hex = body
                    .get("address")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| AppError::InvalidArgument("getaccount: missing address".into()))?;
                let bytes = address_to_bytes(address_hex).ok_or_else(|| {
                    AppError::InvalidArgument(format!("invalid tron address: {address_hex}"))
                })?;
                client.get_account(bytes).await
            }
            "broadcasthex" => {
                let tx_hex = body
                    .get("transaction")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        AppError::InvalidArgument("broadcasthex: missing transaction".into())
                    })?;
                client.broadcast_hex(tx_hex).await
            }
            "triggerconstantcontract" => {
                let owner = body
                    .get("owner_address")
                    .and_then(|v| v.as_str())
                    .and_then(address_to_bytes)
                    .ok_or_else(|| {
                        AppError::InvalidArgument(
                            "triggerconstantcontract: invalid owner_address".into(),
                        )
                    })?;
                let contract = body
                    .get("contract_address")
                    .and_then(|v| v.as_str())
                    .and_then(address_to_bytes)
                    .ok_or_else(|| {
                        AppError::InvalidArgument(
                            "triggerconstantcontract: invalid contract_address".into(),
                        )
                    })?;
                let data_hex = body
                    .get("data")
                    .and_then(|v| v.as_str())
                    .unwrap_or("0x");
                let data = hex::decode(data_hex.trim_start_matches("0x"))
                    .map_err(|e| AppError::InvalidArgument(format!("invalid data hex: {e}")))?;
                client
                    .trigger_constant_contract(owner, contract, data)
                    .await
            }
            other => Err(AppError::InvalidArgument(format!(
                "unsupported tron gRPC method: {other}"
            ))),
        }
    }

    /// Convenience wrapper: pick the next URL, route to gRPC if configured,
    /// otherwise HTTP. The path is the TRON JSON-RPC method path (e.g.
    /// `/wallet/getnowblock`) and `body` is the JSON body.
    async fn post(&self, path: &str, body: Value) -> AppResult<Value> {
        let url = self.pool.next_url()?.to_string();
        if let Some(client) = self.grpc_clients.get(&url).cloned() {
            return self.post_grpc(&url, client, path, body).await;
        }
        self.post_http(&url, path, body).await
    }
}

/// Convert a TRON base58 address (the JSON form) to the 21-byte `0x41`-prefixed
/// payload expected by the protobuf `Account.address` / `TriggerSmartContract`
/// fields. The base58check encoding is exactly the same one used to encode the
/// protobuf bytes back to a string, so this is a reversible round-trip.
fn address_to_bytes(addr: &str) -> Option<Vec<u8>> {
    bs58::decode(addr).with_check(None).into_vec().ok()
}

#[async_trait]
impl BalanceReader for TronChain {
    async fn native_balance(&self, addr: &Address) -> AppResult<Amount> {
        let v = self
            .post(
                "/wallet/getaccount",
                json!({ "address": addr.as_str(), "visible": true }),
            )
            .await?;
        let balance = v.get("balance").and_then(|b| b.as_i64()).unwrap_or(0);
        Ok(Amount::new(Decimal::from(balance), 6))
    }
}

#[async_trait]
impl TxBroadcaster for TronChain {
    async fn send_raw(&self, raw: &[u8]) -> AppResult<TxHash> {
        let v = self
            .post(
                "/wallet/broadcasthex",
                json!({ "transaction": hex::encode(raw) }),
            )
            .await?;
        let hash = v.get("txid").and_then(|t| t.as_str()).unwrap_or_default();
        Ok(TxHash::new(hash))
    }
}

#[async_trait]
impl BlockSource for TronChain {
    async fn tip(&self) -> AppResult<u64> {
        let v = self.post("/wallet/getnowblock", json!({})).await?;
        Ok(v.pointer("/block_header/raw_data/number")
            .and_then(|n| n.as_u64())
            .unwrap_or(0))
    }

    async fn fetch_block_txs(&self, height: u64) -> AppResult<Vec<NormalizedTx>> {
        let v = self
            .post("/wallet/getblockbynum", json!({ "num": height }))
            .await?;
        let txs = v
            .get("transactions")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default();
        // One RPC for every receipt in the block — mirrors EVM's
        // `eth_getBlockReceipts`. The previous per-tx
        // `gettransactioninfobyid` loop was ~200 sequential round-trips
        // per Tron block and dominated sync wall-clock (~40s/batch).
        let info_by_id = self.tron_block_tx_info_map(height).await;
        let mut out = Vec::with_capacity(txs.len());
        for tx in txs {
            let hash = tx.get("txID").and_then(|h| h.as_str()).unwrap_or_default();
            let raw_data = tx.get("raw_data");
            let from = raw_data
                .and_then(|r| r.get("contract"))
                .and_then(|c| c.as_array())
                .and_then(|arr| arr.first())
                .and_then(|c| c.get("parameter"))
                .and_then(|p| p.get("value"))
                .and_then(|v| v.get("owner_address"))
                .and_then(|v| v.as_str())
                .and_then(normalize_tron_address);
            let to = raw_data
                .and_then(|r| r.get("contract"))
                .and_then(|c| c.as_array())
                .and_then(|arr| arr.first())
                .and_then(|c| c.get("parameter"))
                .and_then(|p| p.get("value"))
                .and_then(|v| v.get("to_address"))
                .and_then(|v| v.as_str())
                .and_then(normalize_tron_address);
            let value = raw_data
                .and_then(|r| r.get("contract"))
                .and_then(|c| c.as_array())
                .and_then(|arr| arr.first())
                .and_then(|c| c.get("parameter"))
                .and_then(|p| p.get("value"))
                .and_then(|v| v.get("amount"))
                .and_then(|v| v.as_i64())
                .map(|a| Amount::new(Decimal::from(a), 6))
                .unwrap_or_else(|| Amount::zero(6));
            let hash_str = hash.to_string();
            let status = tron_tx_status(&tx);
            let info = info_by_id.get(&normalize_txid(&hash_str));
            let fee = info
                .and_then(|v| v.get("fee"))
                .and_then(|f| f.as_i64())
                .map(|f| Amount::new(Decimal::from(f), 6));
            // Skip native rows that move zero TRX (pure contract interactions);
            // TRC20 transfers are parsed separately via logs.
            if !value.raw.is_zero() {
                out.push(NormalizedTx {
                    hash: TxHash::new(hash_str.clone()),
                    from,
                    to,
                    value,
                    gas_fee: fee.clone(),
                    block_number: height,
                    status,
                    raw: tx.clone(),
                    contract_address: None,
                    log_index: None,
                    // A native TRX value transfer row.
                    method: Some("transfer".to_string()),
                });
            }
            // TRC20 transfers: parse Transfer events from this tx's logs.
            out.extend(parse_trc20_logs(info, &hash_str, height, fee));
        }
        Ok(out)
    }
}

impl TronChain {
    /// Fetch `/wallet/gettransactioninfobyblocknum` and index receipts by tx id.
    ///
    /// Best-effort: unsupported/failed calls return an empty map (native TRX
    /// rows still sync; TRC20 log rows and fees are simply omitted).
    async fn tron_block_tx_info_map(&self, height: u64) -> HashMap<String, Value> {
        let infos = match self
            .post(
                "/wallet/gettransactioninfobyblocknum",
                json!({ "num": height as i64 }),
            )
            .await
        {
            Ok(v) => match v {
                Value::Array(arr) => arr,
                other => {
                    let shape = match other {
                        Value::Null => "null",
                        Value::Bool(_) => "bool",
                        Value::Number(_) => "number",
                        Value::String(_) => "string",
                        Value::Object(_) => "object",
                        Value::Array(_) => "array",
                    };
                    tracing::debug!(
                        height,
                        shape,
                        "gettransactioninfobyblocknum: unexpected shape"
                    );
                    return HashMap::new();
                }
            },
            Err(e) => {
                tracing::debug!(height, error = %e, "gettransactioninfobyblocknum failed");
                return HashMap::new();
            }
        };
        let mut map = HashMap::with_capacity(infos.len());
        for info in infos {
            let Some(id) = info.get("id").and_then(|v| v.as_str()) else {
                continue;
            };
            map.insert(normalize_txid(id), info);
        }
        map
    }
}

/// Parse TRC20 `Transfer(address,address,uint256)` logs from a tx's
/// `gettransactioninfobyid` response into token transfer rows.
fn parse_trc20_logs(
    info: Option<&Value>,
    tx_id: &str,
    height: u64,
    gas_fee: Option<Amount>,
) -> Vec<NormalizedTx> {
    let Some(v) = info else {
        return Vec::new();
    };
    let Some(logs) = v.get("log").and_then(|l| l.as_array()) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (idx, log) in logs.iter().enumerate() {
        let Some(contract_hex) = log.get("address").and_then(|a| a.as_str()) else {
            continue;
        };
        let Some(topics) = log.get("topics").and_then(|t| t.as_array()) else {
            continue;
        };
        if topics.len() < 3 {
            continue;
        }
        let Some(topic0) = topics[0].as_str() else {
            continue;
        };
        // HTTP returns the topic bare; gRPC → JSON adds a `0x` prefix via
        // `bytes_to_hex`. Compare after stripping so both paths match.
        if !is_trc20_transfer_topic(topic0) {
            continue;
        }
        let from = topics[1].as_str().and_then(tron_topic_address);
        let to = topics[2].as_str().and_then(tron_topic_address);
        let amount = log
            .get("data")
            .and_then(|d| d.as_str())
            .map(tron_hex_amount)
            .unwrap_or(Decimal::ZERO);
        let Some(contract) = normalize_tron_address(contract_hex) else {
            continue;
        };
        let log_index = log
            .get("logIndex")
            .and_then(|i| i.as_i64())
            .map(|i| i as u64)
            .or_else(|| {
                log.get("logIndex")
                    .and_then(|i| i.as_str())
                    .and_then(|h| u64::from_str_radix(h.trim_start_matches("0x"), 16).ok())
            });
        out.push(NormalizedTx {
            hash: TxHash::new(tx_id),
            from,
            to,
            value: Amount::new(amount, 6),
            gas_fee: gas_fee.clone(),
            block_number: height,
            status: TxStatus::Success,
            raw: json!({ "log": log, "logIndex": idx }),
            contract_address: Some(contract),
            log_index,
            // Rows derived from a TRC20 `Transfer(address,address,uint256)`
            // event; the method is the event itself regardless of the tx data.
            method: Some("transfer".to_string()),
        });
    }
    out
}

/// `keccak256("Transfer(address,address,uint256)")` as used by TRC20.
const TRC20_TRANSFER_TOPIC: &str =
    "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";

/// True when `topic` is the TRC20 `Transfer` event signature, with or without
/// a leading `0x` (gRPC JSON conversion prefixes hex; HTTP does not).
fn is_trc20_transfer_topic(topic: &str) -> bool {
    topic
        .strip_prefix("0x")
        .unwrap_or(topic)
        .eq_ignore_ascii_case(TRC20_TRANSFER_TOPIC)
}

/// Strip an optional `0x` / `0X` prefix and lowercase so tx ids from the block
/// (`txID`, bare hex) and from `TransactionInfo.id` (`0x…` after gRPC
/// conversion) hash to the same map key.
fn normalize_txid(id: &str) -> String {
    id.strip_prefix("0x")
        .or_else(|| id.strip_prefix("0X"))
        .unwrap_or(id)
        .to_ascii_lowercase()
}

/// TRON tx status from the `ret` array (contractRet == SUCCESS).
fn tron_tx_status(tx: &Value) -> TxStatus {
    tx.get("ret")
        .and_then(|r| r.as_array())
        .and_then(|arr| arr.first())
        .and_then(|first| first.get("contractRet"))
        .and_then(|v| v.as_str())
        .map(|s| if s == "SUCCESS" { TxStatus::Success } else { TxStatus::Failed })
        .unwrap_or(TxStatus::Success)
}

/// Normalize a TRON address field to base58check (`T…`).
///
/// Accepts:
/// - base58check (`T…`) — returned as-is when checksum-valid
/// - 21-byte hex (`0x41…` / `41…`) — converted via base58check
/// - 20-byte hex (EVM-style, no `41` prefix) — prefixed with `0x41` then converted
///
/// Used for normalized `from` / `to` / `contract_address` fields. Raw JSON
/// from gRPC may still carry hex; that is intentional.
pub fn normalize_tron_address(addr: &str) -> Option<Address> {
    let addr = addr.trim();
    if addr.is_empty() {
        return None;
    }
    if addr.starts_with('T') {
        bs58::decode(addr).with_check(None).into_vec().ok()?;
        return Some(Address::new(addr));
    }
    tron_hex_to_address(addr).map(Address::new)
}

/// Convert a 32-byte right-padded TRON address topic (hex) to base58 address.
fn tron_topic_address(topic: &str) -> Option<Address> {
    let hex = topic.strip_prefix("0x").unwrap_or(topic);
    if hex.len() < 40 {
        return None;
    }
    let body = &hex[hex.len() - 40..];
    tron_hex_to_address(&format!("41{body}")).map(Address::new)
}

/// Convert a `41`-prefixed hex TRON address to base58check.
/// Also accepts a bare 20-byte (40 hex char) body by prepending `41`.
fn tron_hex_to_address(hex_addr: &str) -> Option<String> {
    let hex_addr = hex_addr.strip_prefix("0x").unwrap_or(hex_addr);
    let bytes = if hex_addr.len() == 40 {
        let mut out = Vec::with_capacity(21);
        out.push(0x41);
        out.extend(hex::decode(hex_addr).ok()?);
        out
    } else {
        hex::decode(hex_addr).ok()?
    };
    if bytes.len() != 21 || bytes[0] != 0x41 {
        return None;
    }
    Some(bs58::encode(bytes).with_check().into_string())
}

/// Parse a uint256 amount from TRON log data (hex) into a Decimal.
/// Overflowing values saturate at `Decimal::MAX` instead of panicking.
fn tron_hex_amount(hex: &str) -> Decimal {
    let hex = hex.strip_prefix("0x").unwrap_or(hex).trim_start_matches('0');
    match u128::from_str_radix(if hex.is_empty() { "0" } else { hex }, 16) {
        Ok(v) => wallet_types::decimal_from_u128(v),
        Err(_) => Decimal::ZERO,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tron_hex_to_address() {
        // USDT-TRON contract: 41a614f803b6fd780986a42c78ec9c7f77e6ded13c
        assert_eq!(
            tron_hex_to_address("41a614f803b6fd780986a42c78ec9c7f77e6ded13c").unwrap(),
            "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t"
        );
    }

    #[test]
    fn test_normalize_tron_address_hex_0x41_to_base58() {
        let addr = normalize_tron_address(
            "0x41a614f803b6fd780986a42c78ec9c7f77e6ded13c",
        )
        .unwrap();
        assert_eq!(addr.as_str(), "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t");
    }

    #[test]
    fn test_normalize_tron_address_keeps_base58() {
        let addr = normalize_tron_address("TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t").unwrap();
        assert_eq!(addr.as_str(), "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t");
    }

    #[test]
    fn test_tron_topic_address_extracts_right_padded() {
        let topic = format!(
            "0x{}",
            "0".repeat(24) + "a614f803b6fd780986a42c78ec9c7f77e6ded13c"
        );
        let addr = tron_topic_address(&topic).unwrap();
        assert_eq!(addr.as_str(), "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t");
    }

    #[test]
    fn test_tron_hex_amount() {
        assert_eq!(tron_hex_amount("0x64"), Decimal::from(100));
        assert_eq!(tron_hex_amount("0x0"), Decimal::ZERO);
        assert_eq!(
            tron_hex_amount("0x000000000000000000000000000000000000000000000000000000000000000f"),
            Decimal::from(15)
        );
    }

    #[test]
    fn test_normalize_txid_strips_0x_and_lowercases() {
        assert_eq!(
            normalize_txid("0xAaBbCc"),
            normalize_txid("AABBCC")
        );
        assert_eq!(normalize_txid("deadbeef"), "deadbeef");
    }

    #[test]
    fn test_is_trc20_transfer_topic_strips_0x() {
        assert!(is_trc20_transfer_topic(TRC20_TRANSFER_TOPIC));
        assert!(is_trc20_transfer_topic(&format!("0x{TRC20_TRANSFER_TOPIC}")));
        assert!(is_trc20_transfer_topic(&format!(
            "0x{}",
            TRC20_TRANSFER_TOPIC.to_ascii_uppercase()
        )));
        assert!(!is_trc20_transfer_topic("00"));
    }

    #[test]
    fn test_tron_tx_status() {
        let ok = json!({ "ret": [{ "contractRet": "SUCCESS" }] });
        assert_eq!(tron_tx_status(&ok), TxStatus::Success);
        let fail = json!({ "ret": [{ "contractRet": "REVERT" }] });
        assert_eq!(tron_tx_status(&fail), TxStatus::Failed);
    }

    #[test]
    fn test_address_to_bytes_round_trip() {
        let addr = "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t";
        let bytes = address_to_bytes(addr).unwrap();
        assert_eq!(bytes.len(), 21);
        assert_eq!(bytes[0], 0x41);
        let round = bs58::encode(&bytes).with_check().into_string();
        assert_eq!(round, addr);
    }

    #[test]
    fn test_parse_trc20_logs_sets_gas_fee() {
        let topic_owner = format!(
            "0x{}",
            "0".repeat(24) + "a614f803b6fd780986a42c78ec9c7f77e6ded13c"
        );
        let info = json!({
            "fee": 12345,
            "log": [{
                "address": "0x41a614f803b6fd780986a42c78ec9c7f77e6ded13c",
                "topics": [
                    TRC20_TRANSFER_TOPIC,
                    topic_owner.clone(),
                    topic_owner,
                ],
                "data": "0x64",
                "logIndex": 0
            }]
        });
        let rows = parse_trc20_logs(
            Some(&info),
            "tx1",
            5,
            Some(Amount::new(Decimal::from(12345), 6)),
        );
        assert_eq!(rows.len(), 1);
        let r = &rows[0];
        assert_eq!(r.gas_fee.as_ref().unwrap().raw, Decimal::from(12345));
        assert_eq!(r.method.as_deref(), Some("transfer"));
    }

    /// gRPC → JSON conversion prefixes every topic with `0x` (see
    /// `proto_value::bytes_to_hex`). The Transfer topic must still match.
    #[test]
    fn test_parse_trc20_logs_accepts_0x_prefixed_topic() {
        let topic_owner = format!(
            "0x{}",
            "0".repeat(24) + "a614f803b6fd780986a42c78ec9c7f77e6ded13c"
        );
        let info = json!({
            "fee": 1,
            "log": [{
                "address": "0xa614f803b6fd780986a42c78ec9c7f77e6ded13c",
                "topics": [
                    format!("0x{TRC20_TRANSFER_TOPIC}"),
                    topic_owner.clone(),
                    topic_owner,
                ],
                "data": "0x64",
                "logIndex": 0
            }]
        });
        let rows = parse_trc20_logs(Some(&info), "tx1", 5, None);
        assert_eq!(rows.len(), 1);
        assert_eq!(
            rows[0].contract_address.as_ref().unwrap().as_str(),
            "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t"
        );
    }
}

#[async_trait]
impl GasEstimator for TronChain {
    async fn estimate_gas(&self, tx: &GasEstimateRequest) -> AppResult<GasEstimate> {
        if let Some(to_addr) = &tx.to {
            let data_hex = tx
                .data
                .as_ref()
                .map(|d| format!("0x{}", hex::encode(d)))
                .unwrap_or_else(|| "0x06feacc0".to_string());
            let _selector = if data_hex.len() >= 10 {
                &data_hex[2..10]
            } else {
                ""
            };
            let params: Vec<String> = if let Some(raw) = tx.data.as_ref() {
                if raw.len() > 4 {
                    raw.chunks(32).map(hex::encode).collect()
                } else {
                    vec![]
                }
            } else {
                vec![]
            };
            let mut body = json!({
                "owner_address": tx.from.as_str(),
                "contract_address": to_addr.as_str(),
                "function_selector": "estimateEnergy(address)",
                "parameter": params,
                "visible": true,
            });
            if let Some(val) = &tx.value {
                body["call_value"] = json!(val.raw.to_string());
            }
            let v = self.post("/wallet/triggerconstantcontract", body).await?;
            if let Some(constant_result) = v.get("constant_result").and_then(|r| r.as_array()) {
                if let Some(first) = constant_result.first() {
                    let hex_str = first.as_str().unwrap_or("0");
                    let energy = u64::from_str_radix(hex_str, 16).unwrap_or(65_000);
                    return Ok(GasEstimate {
                        gas_limit: energy.max(65_000),
                        max_fee_per_gas: None,
                        max_priority_fee_per_gas: None,
                        ..GasEstimate::default()
                    });
                }
            }
        }
        Ok(GasEstimate {
            gas_limit: 65_000,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            ..GasEstimate::default()
        })
    }
}

#[async_trait]
impl TokenBalance for TronChain {
    async fn token_balance(&self, wallet: &Address, token: &Address) -> AppResult<Amount> {
        // Use TRON RPC to query TRC20 balance
        let v = self
            .post(
                "/wallet/triggerconstantcontract",
                json!({
                    "owner_address": wallet.as_str(),
                    "contract_address": token.as_str(),
                    "function_selector": "balanceOf(address)",
                    "parameter": [wallet.as_str()],
                    "visible": true,
                }),
            )
            .await?;
        if let Some(constant_result) = v.get("constant_result").and_then(|r| r.as_array()) {
            if let Some(first) = constant_result.first() {
                let hex_str = first.as_str().unwrap_or("0");
                let balance = u64::from_str_radix(hex_str, 16).unwrap_or(0);
                return Ok(Amount::new(Decimal::from(balance), 6));
            }
        }
        Ok(Amount::zero(6))
    }
}
