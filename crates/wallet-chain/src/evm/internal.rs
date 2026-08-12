//! Internal native transfers for EVM transactions.
//!
//! Contracts can move native tokens through internal `CALL`s (e.g. a swap
//! router forwarding ETH to the user, or an EIP-7702 authorized call). These
//! are only discoverable via `debug_traceTransaction`, which is expensive, so
//! we only trace transactions that emit a Swap event or are EIP-7702 txs.

use crate::evm::EvmChain;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use wallet_types::{Address, Amount, NormalizedTx, TxHash, TxStatus};

/// `keccak256("Swap(address,uint256,uint256,uint256,uint256,address)")`
/// (Uniswap V2 / PancakeSwap).
const SWAP_V2_TOPIC: &str =
    "d78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822";
/// `keccak256("Swap(address,address,int256,int256,uint160,uint128,int24)")`
/// (Uniswap V3).
const SWAP_V3_TOPIC: &str =
    "c42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67";
/// `keccak256("TokenExchange(address,address,int128,uint256,int128,uint256)")`
/// (Curve).
const CURVE_SWAP_TOPIC: &str =
    "8b3e96f2b889fa771c53c981b40daf005f63f637f1869f707052d15a3dd97140";

/// Whether the given logs emit a known Swap event.
pub fn has_swap_event(logs: &[Value]) -> bool {
    logs.iter().any(|log| {
        log.get("topics")
            .and_then(|t| t.as_array())
            .and_then(|topics| topics.first())
            .and_then(|t| t.as_str())
            .map(|topic| {
                let topic = topic.strip_prefix("0x").unwrap_or(topic);
                topic == SWAP_V2_TOPIC || topic == SWAP_V3_TOPIC || topic == CURVE_SWAP_TOPIC
            })
            .unwrap_or(false)
    })
}

/// EIP-7702 txs are `type 0x4` and carry an `authorizationList`.
pub fn is_eip7702(tx: &Value) -> bool {
    tx.get("type")
        .and_then(|v| v.as_str())
        .map(|t| t.trim_start_matches("0x") == "4")
        .unwrap_or(false)
        || tx.get("authorizationList").map(|v| v.is_array()).unwrap_or(false)
}

/// A native value transfer performed by an internal `CALL`.
struct InternalCall {
    from: String,
    to: String,
    value: Decimal,
}

fn hex_value(v: Option<&Value>) -> Decimal {
    match v.and_then(|v| v.as_str()) {
        Some(h) => u128::from_str_radix(h.trim_start_matches("0x"), 16)
            .map(wallet_types::decimal_from_u128)
            .unwrap_or(Decimal::ZERO),
        None => Decimal::ZERO,
    }
}

/// Walk a callTracer tree, collecting native value transfers from internal
/// `CALL`s (depth >= 1). The top-level frame is the tx itself and is skipped.
fn collect_internal_calls(call: &Value, depth: usize, out: &mut Vec<InternalCall>) {
    if depth >= 1 {
        let is_call = call
            .get("type")
            .and_then(|v| v.as_str())
            .map(|t| t == "CALL")
            .unwrap_or(false);
        if is_call && call.get("error").is_none() {
            let from = call.get("from").and_then(|v| v.as_str()).unwrap_or_default();
            let to = call.get("to").and_then(|v| v.as_str()).unwrap_or_default();
            let value = hex_value(call.get("value"));
            if !from.is_empty() && !to.is_empty() && !value.is_zero() {
                out.push(InternalCall {
                    from: from.to_string(),
                    to: to.to_string(),
                    value,
                });
            }
        }
    }
    if let Some(children) = call.get("calls").and_then(|c| c.as_array()) {
        for child in children {
            collect_internal_calls(child, depth + 1, out);
        }
    }
}

impl EvmChain {
    /// Fetch internal native transfers for a tx via `debug_traceTransaction`
    /// (callTracer). Best-effort: unsupported/missing traces are skipped.
    pub async fn fetch_internal_transfers(
        &self,
        tx_hash: &str,
        height: u64,
        tx_status: TxStatus,
    ) -> Vec<NormalizedTx> {
        if tx_status != TxStatus::Success {
            return Vec::new();
        }
        let Some(trace) = self
            .rpc_best_effort(
                "debug_traceTransaction",
                json!([
                    tx_hash,
                    { "tracer": "callTracer", "tracerConfig": { "onlyTopCall": false } }
                ]),
            )
            .await
        else {
            return Vec::new();
        };
        let mut calls = Vec::new();
        collect_internal_calls(&trace, 0, &mut calls);
        calls
            .into_iter()
            .enumerate()
            .map(|(i, c)| NormalizedTx {
                hash: TxHash::new(tx_hash),
                from: Some(Address::new(c.from)),
                to: Some(Address::new(c.to)),
                value: Amount::new(c.value, 18),
                gas_fee: None,
                block_number: height,
                status: TxStatus::Success,
                raw: json!({ "internal": true, "trace_index": i }),
                contract_address: None,
                log_index: Some(i as i64),
                method: Some("internal_transfer".to_string()),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_event_detected() {
        let log = json!({
            "topics": ["0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822"]
        });
        assert!(has_swap_event(&[log]));
    }

    #[test]
    fn non_swap_events_ignored() {
        let log = json!({
            "topics": ["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"]
        });
        assert!(!has_swap_event(&[log]));
    }

    #[test]
    fn eip7702_detected_by_type_and_authorization_list() {
        assert!(is_eip7702(&json!({ "type": "0x4" })));
        assert!(is_eip7702(&json!({ "authorizationList": [] })));
        assert!(!is_eip7702(&json!({ "type": "0x2" })));
        assert!(!is_eip7702(&json!({})));
    }

    #[test]
    fn internal_calls_collected_skipping_top_level() {
        let trace = json!({
            "from": "0xaaa",
            "to": "0xswap",
            "value": "0x0",
            "type": "CALL",
            "calls": [
                {
                    "from": "0xswap",
                    "to": "0xuser",
                    "value": "0x64",
                    "type": "CALL"
                },
                {
                    "from": "0xswap",
                    "to": "0xcontract",
                    "value": "0x0",
                    "type": "CALL",
                    "calls": [
                        {
                            "from": "0xcontract",
                            "to": "0xuser2",
                            "value": "0x2a",
                            "type": "CALL"
                        }
                    ]
                },
                {
                    "from": "0xswap",
                    "to": "0xdead",
                    "value": "0x32",
                    "type": "CALL",
                    "error": "execution reverted"
                }
            ]
        });
        let mut calls = Vec::new();
        collect_internal_calls(&trace, 0, &mut calls);
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].from, "0xswap");
        assert_eq!(calls[0].to, "0xuser");
        assert_eq!(calls[0].value, Decimal::from(100));
        assert_eq!(calls[1].to, "0xuser2");
        assert_eq!(calls[1].value, Decimal::from(42));
    }

    #[test]
    fn zero_value_internal_calls_skipped() {
        let trace = json!({
            "from": "0xaaa",
            "to": "0xswap",
            "value": "0x0",
            "type": "CALL",
            "calls": [
                { "from": "0xswap", "to": "0xuser", "value": "0x0", "type": "CALL" }
            ]
        });
        let mut calls = Vec::new();
        collect_internal_calls(&trace, 0, &mut calls);
        assert!(calls.is_empty());
    }
}
