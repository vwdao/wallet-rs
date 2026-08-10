//! Typed TRON gRPC client used by [`crate::tron::TronChain`] when the
//! configured endpoint is a `grpc(s)://...` chain-gateway URL.
//!
//! The chain-gateway proxies gRPC calls to the upstream TRON node unchanged,
//! so we just use the tonic-generated [`protocol::wallet_client::WalletClient`]
//! from `tron-rs` against the gateway's gRPC port and translate the typed
//! responses back to the JSON shape the rest of the wallet-sync parser
//! already understands (see [`proto_value`]).
//!
//! This mirrors the Go `demo_tron_grpc_client` reference: same
//! `client.NewGrpcClientWithTimeout(...)` + `SetAPIKey(...)` flow, just
//! in Rust. The API key is sent as `x-api-key` / `TRON-PRO-API-KEY`
//! metadata so the chain-gateway's existing auth lookup accepts it.

use std::time::Duration;

use prost::Message;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use tonic::metadata::{Ascii, MetadataValue};
use tonic::Request;
use wallet_error::{AppError, AppResult};

use super::proto_value::{
    account_to_value, block_to_value, transaction_extention_to_value, transaction_info_to_value,
};
use super::protocol::wallet_client::WalletClient;
use super::protocol::{Account, BytesMessage, EmptyMessage, NumberMessage, TriggerSmartContract};
use super::url::GatewayGrpcUrl;

/// A reusable gRPC client bound to one chain-gateway endpoint. The underlying
/// channel is created lazily on the first call so config-time URL parse
/// errors still surface but transient network issues do not block startup.
pub(crate) struct TronGrpcClient {
    http_url: String,
    api_key: String,
    connect_lazy: tonic::transport::Channel,
}

impl std::fmt::Debug for TronGrpcClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TronGrpcClient")
            .field("http_url", &self.http_url)
            .finish_non_exhaustive()
    }
}

impl TronGrpcClient {
    pub(crate) fn connect(gw: &GatewayGrpcUrl, timeout: Duration) -> AppResult<Self> {
        let mut endpoint = tonic::transport::Endpoint::from_shared(gw.http_url.clone())
            .map_err(|e| AppError::InvalidArgument(format!("invalid grpc endpoint: {e}")))?;
        if gw.http_url.starts_with("https://") {
            endpoint = endpoint
                .tls_config(tonic::transport::ClientTlsConfig::new().with_enabled_roots())
                .map_err(|e| AppError::InvalidArgument(e.to_string()))?;
        }
        endpoint = endpoint.timeout(timeout).connect_timeout(timeout);
        let connect_lazy = endpoint.connect_lazy();
        Ok(Self {
            http_url: gw.http_url.clone(),
            api_key: gw.api_key.clone(),
            connect_lazy,
        })
    }

    fn build_request<T>(&self, inner: T) -> Request<T> {
        let mut request = Request::new(inner);
        // gRPC metadata keys must be lowercase ASCII; the chain-gateway
        // looks both up case-insensitively, so lowercased TRON-PRO-API-KEY
        // still matches its `HeaderMap::get` check.
        if let Ok(value) = self
            .api_key
            .parse::<MetadataValue<Ascii>>()
        {
            request.metadata_mut().insert("x-api-key", value.clone());
            request.metadata_mut().insert("tron-pro-api-key", value);
        }
        request
    }

    fn client(&self) -> WalletClient<tonic::transport::Channel> {
        WalletClient::new(self.connect_lazy.clone())
    }

    /// `getnowblock` → returns JSON `{"block_header": {"raw_data": {"number": N}}}`.
    pub(crate) async fn get_now_block(&self) -> AppResult<Value> {
        let mut client = self.client();
        let resp = client
            .get_now_block(self.build_request(EmptyMessage {}))
            .await
            .map_err(|e| AppError::Unavailable(format!("getnowblock: {e}")))?;
        Ok(block_to_value(&resp.into_inner()))
    }

    /// `getblockbynum` → returns the full block JSON (with `transactions`).
    pub(crate) async fn get_block_by_num(&self, num: i64) -> AppResult<Value> {
        let mut client = self.client();
        let resp = client
            .get_block_by_num(self.build_request(NumberMessage { num }))
            .await
            .map_err(|e| AppError::Unavailable(format!("getblockbynum: {e}")))?;
        Ok(block_to_value(&resp.into_inner()))
    }

    /// `gettransactioninfobyid` → returns the receipt JSON (with `fee`, `log[]`).
    pub(crate) async fn get_transaction_info_by_id(
        &self,
        value: Vec<u8>,
    ) -> AppResult<Value> {
        let mut client = self.client();
        let resp = client
            .get_transaction_info_by_id(self.build_request(BytesMessage { value }))
            .await
            .map_err(|e| AppError::Unavailable(format!("gettransactioninfobyid: {e}")))?;
        Ok(transaction_info_to_value(&resp.into_inner()))
    }

    /// `gettransactioninfobyblocknum` → JSON array of receipts for every tx
    /// in the block (same shape as the HTTP endpoint).
    pub(crate) async fn get_transaction_info_by_block_num(
        &self,
        num: i64,
    ) -> AppResult<Value> {
        let mut client = self.client();
        let resp = client
            .get_transaction_info_by_block_num(self.build_request(NumberMessage { num }))
            .await
            .map_err(|e| {
                AppError::Unavailable(format!("gettransactioninfobyblocknum: {e}"))
            })?;
        let list = resp.into_inner();
        Ok(Value::Array(
            list.transaction_info
                .iter()
                .map(transaction_info_to_value)
                .collect(),
        ))
    }

    /// `getaccount` → returns the account JSON (with `balance`).
    pub(crate) async fn get_account(&self, address: Vec<u8>) -> AppResult<Value> {
        let mut client = self.client();
        let resp = client
            .get_account(self.build_request(Account {
                address,
                ..Default::default()
            }))
            .await
            .map_err(|e| AppError::Unavailable(format!("getaccount: {e}")))?;
        Ok(account_to_value(&resp.into_inner()))
    }

    /// `broadcasthex` → returns `{"txid": "..."}`. The JSON-RPC variant
    /// takes a hex-encoded `Transaction`; on gRPC we decode it and call
    /// `BroadcastTransaction` directly. The `Return` response tells us
    /// whether the broadcast succeeded; the `txid` is recovered from the
    /// decoded `raw_data` sha256 (same as the JSON-RPC server's `txid`).
    pub(crate) async fn broadcast_hex(&self, transaction_hex: &str) -> AppResult<Value> {
        let tx_bytes = hex::decode(transaction_hex.trim_start_matches("0x"))
            .map_err(|e| AppError::InvalidArgument(format!("invalid hex: {e}")))?;
        let tx = super::protocol::Transaction::decode(tx_bytes.as_slice())
            .map_err(|e| AppError::InvalidArgument(format!("invalid tron transaction: {e}")))?;
        let mut client = self.client();
        let resp = client
            .broadcast_transaction(self.build_request(tx.clone()))
            .await
            .map_err(|e| AppError::Unavailable(format!("broadcasthex: {e}")))?;
        let inner = resp.into_inner();
        if !inner.result {
            return Err(AppError::Unavailable(format!(
                "broadcasthex failed (code={})",
                inner.code
            )));
        }
        // Recover the txid from raw_data for symmetry with the JSON path.
        let txid = tx
            .raw_data
            .as_ref()
            .and_then(|rd| {
                let mut buf = Vec::with_capacity(rd.encoded_len());
                rd.encode(&mut buf).ok()?;
                Some(hex::encode(Sha256::digest(&buf)))
            })
            .unwrap_or_default();
        Ok(json!({ "txid": txid, "result": true, "code": inner.code }))
    }

    /// `triggerconstantcontract` → returns `{"constant_result": [...], "result": bool}`.
    pub(crate) async fn trigger_constant_contract(
        &self,
        owner_address: Vec<u8>,
        contract_address: Vec<u8>,
        data: Vec<u8>,
    ) -> AppResult<Value> {
        let mut client = self.client();
        let resp = client
            .trigger_constant_contract(self.build_request(TriggerSmartContract {
                owner_address,
                contract_address,
                data,
                ..Default::default()
            }))
            .await
            .map_err(|e| AppError::Unavailable(format!("triggerconstantcontract: {e}")))?;
        Ok(transaction_extention_to_value(&resp.into_inner()))
    }
}
