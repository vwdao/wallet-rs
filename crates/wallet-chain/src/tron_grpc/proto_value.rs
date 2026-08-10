//! Convert tron-rs protobuf responses to a `serde_json::Value` that matches the
//! shape of the TRON JSON-RPC API.
//!
//! The chain's [`TronChain`](crate::tron::TronChain) keeps its block/transaction
//! parser in terms of `serde_json::Value` (the same shape the legacy HTTP path
//! produces). To keep that parser working on top of the typed gRPC client we
//! marshal the relevant protobuf types into a `Value` that looks like what the
//! HTTP endpoint would have returned.
//!
//! The conversions are intentionally limited to the fields the parser uses —
//! no attempt is made to round-trip every protobuf field.

use prost::Message;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

use super::protocol;

/// Convert raw bytes to a `0x`-prefixed lower-case hex string.
fn bytes_to_hex(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}

/// Compute the TRON `txID` from a `Transaction::Raw` payload. The TRON
/// protocol uses `sha256(serialized raw_data)` as the canonical transaction
/// id, so the legacy JSON-RPC `txID` field equals it.
fn tx_id_from_raw(raw: &protocol::transaction::Raw) -> Option<String> {
    let mut buf = Vec::with_capacity(raw.encoded_len());
    raw.encode(&mut buf).ok()?;
    Some(hex::encode(Sha256::digest(&buf)))
}

/// Decode a TRON contract `Any` payload into a JSON object matching the
/// `parameter.value` shape used by the JSON-RPC API.
///
/// `Any.type_url` is of the form `type.googleapis.com/protocol.<Type>`;
/// the protobuf value is then deserialized as `protocol::<Type>` and each
/// field is mapped to a JSON representation (bytes → hex, int → number, …).
fn unpack_contract_parameter(any: &prost_types::Any) -> Option<Map<String, Value>> {
    let type_name = any
        .type_url
        .rsplit_once('/')
        .map(|(_, name)| name)
        .unwrap_or(any.type_url.as_str());
    let bytes = &any.value;
    let mut out = Map::new();
    match type_name {
        "protocol.TransferContract" => {
            let msg = protocol::TransferContract::decode(bytes.as_slice()).ok()?;
            out.insert("owner_address".into(), json!(bytes_to_hex(&msg.owner_address)));
            out.insert("to_address".into(), json!(bytes_to_hex(&msg.to_address)));
            out.insert("amount".into(), json!(msg.amount));
        }
        "protocol.TransferAssetContract" => {
            let msg = protocol::TransferAssetContract::decode(bytes.as_slice()).ok()?;
            out.insert("owner_address".into(), json!(bytes_to_hex(&msg.owner_address)));
            out.insert("to_address".into(), json!(bytes_to_hex(&msg.to_address)));
            out.insert("amount".into(), json!(msg.amount));
            out.insert("asset_name".into(), json!(bytes_to_hex(&msg.asset_name)));
        }
        "protocol.TriggerSmartContract" => {
            let msg = protocol::TriggerSmartContract::decode(bytes.as_slice()).ok()?;
            out.insert("owner_address".into(), json!(bytes_to_hex(&msg.owner_address)));
            out.insert(
                "contract_address".into(),
                json!(bytes_to_hex(&msg.contract_address)),
            );
            out.insert("data".into(), json!(bytes_to_hex(&msg.data)));
            out.insert("call_value".into(), json!(msg.call_value));
        }
        "protocol.FreezeBalanceContract" => {
            let msg = protocol::FreezeBalanceContract::decode(bytes.as_slice()).ok()?;
            out.insert("owner_address".into(), json!(bytes_to_hex(&msg.owner_address)));
            out.insert(
                "receiver_address".into(),
                json!(bytes_to_hex(&msg.receiver_address)),
            );
            out.insert("frozen_balance".into(), json!(msg.frozen_balance));
        }
        "protocol.UnfreezeBalanceContract" => {
            let msg = protocol::UnfreezeBalanceContract::decode(bytes.as_slice()).ok()?;
            out.insert("owner_address".into(), json!(bytes_to_hex(&msg.owner_address)));
            out.insert(
                "receiver_address".into(),
                json!(bytes_to_hex(&msg.receiver_address)),
            );
        }
        "protocol.VoteWitnessContract" => {
            let msg = protocol::VoteWitnessContract::decode(bytes.as_slice()).ok()?;
            out.insert("owner_address".into(), json!(bytes_to_hex(&msg.owner_address)));
            let votes: Vec<Value> = msg
                .votes
                .iter()
                .map(|v| {
                    json!({
                        "vote_address": bytes_to_hex(&v.vote_address),
                        "vote_count": v.vote_count,
                    })
                })
                .collect();
            out.insert("votes".into(), json!(votes));
        }
        _ => {
            // Unknown contract type — preserve the raw `type_url` and the
            // value bytes as hex so the caller can still identify the tx.
            out.insert("type_url".into(), json!(any.type_url));
            out.insert("value_hex".into(), json!(bytes_to_hex(bytes)));
        }
    }
    Some(out)
}

/// Resolve a `ContractType` integer to its JSON-RPC name (e.g.
/// `"TransferContract"`). Unknown values map to `"Unknown"` so we never
/// panic on a future TRON contract type.
fn contract_type_name(t: i32) -> &'static str {
    use protocol::transaction::contract::ContractType as T;
    match T::try_from(t) {
        Ok(t) => t.as_str_name(),
        Err(_) => "Unknown",
    }
}

/// Resolve a `ContractResult` enum value to its JSON-RPC name.
fn contract_result_name(c: i32) -> &'static str {
    use protocol::transaction::result::ContractResult as R;
    match R::try_from(c) {
        Ok(R::Default) => "DEFAULT",
        Ok(R::Success) => "SUCCESS",
        Ok(R::Revert) => "REVERT",
        Ok(R::BadJumpDestination) => "BAD_JUMP_DESTINATION",
        Ok(R::OutOfMemory) => "OUT_OF_MEMORY",
        Ok(R::PrecompiledContract) => "PRECOMPILED_CONTRACT",
        Ok(R::StackTooSmall) => "STACK_TOO_SMALL",
        Ok(R::StackTooLarge) => "STACK_TOO_LARGE",
        Ok(R::IllegalOperation) => "ILLEGAL_OPERATION",
        Ok(R::StackOverflow) => "STACK_OVERFLOW",
        Ok(R::OutOfEnergy) => "OUT_OF_ENERGY",
        Ok(R::OutOfTime) => "OUT_OF_TIME",
        Ok(R::JvmStackOverFlow) => "JVM_STACK_OVER_FLOW",
        Ok(R::Unknown) => "UNKNOWN",
        Ok(R::TransferFailed) => "TRANSFER_FAILED",
        Ok(R::InvalidCode) => "INVALID_CODE",
        _ => "UNKNOWN",
    }
}

/// Build the JSON for a `protocol::Transaction.Contract`, matching the shape
/// the legacy HTTP `/wallet/getblockbynum` response used:
/// `{"type": "<name>", "parameter": {"value": {...}}, ...}`.
fn contract_to_value(c: &protocol::transaction::Contract) -> Value {
    let mut parameter = Map::new();
    if let Some(any) = c.parameter.as_ref() {
        if let Some(value) = unpack_contract_parameter(any) {
            parameter.insert("value".into(), Value::Object(value));
        }
        parameter.insert("type_url".into(), json!(any.type_url));
    }
    json!({
        "type": contract_type_name(c.r#type),
        "parameter": parameter,
        "provider": bytes_to_hex(&c.provider),
        "ContractName": bytes_to_hex(&c.contract_name),
        "Permission_id": c.permission_id,
    })
}

/// Build the JSON for a single `protocol::Transaction`.
pub(crate) fn transaction_to_value(tx: &protocol::Transaction) -> Value {
    let mut raw_data = Map::new();
    let tx_id = if let Some(rd) = tx.raw_data.as_ref() {
        raw_data.insert(
            "contract".into(),
            json!(rd.contract.iter().map(contract_to_value).collect::<Vec<_>>()),
        );
        raw_data.insert("data".into(), json!(bytes_to_hex(&rd.data)));
        raw_data.insert(
            "ref_block_bytes".into(),
            json!(bytes_to_hex(&rd.ref_block_bytes)),
        );
        raw_data.insert(
            "ref_block_hash".into(),
            json!(bytes_to_hex(&rd.ref_block_hash)),
        );
        raw_data.insert("ref_block_num".into(), json!(rd.ref_block_num));
        raw_data.insert("expiration".into(), json!(rd.expiration));
        raw_data.insert("timestamp".into(), json!(rd.timestamp));
        raw_data.insert("fee_limit".into(), json!(rd.fee_limit));
        tx_id_from_raw(rd)
    } else {
        None
    }
    .unwrap_or_default();
    let ret: Vec<Value> = tx
        .ret
        .iter()
        .map(|r| {
            json!({
                "contractRet": contract_result_name(r.contract_ret),
                "fee": r.fee,
            })
        })
        .collect();
    json!({
        "txID": tx_id,
        "raw_data": raw_data,
        "signature": tx
            .signature
            .iter()
            .map(|s| bytes_to_hex(s))
            .collect::<Vec<_>>(),
        "ret": ret,
    })
}

/// Build the JSON for a `protocol::Block`.
pub(crate) fn block_to_value(b: &protocol::Block) -> Value {
    let block_header = b.block_header.as_ref().map(|h| {
        let raw = h.raw_data.as_ref().map(|r| {
            json!({
                "number": r.number,
                "txTrieRoot": bytes_to_hex(&r.tx_trie_root),
                "parentHash": bytes_to_hex(&r.parent_hash),
                "timestamp": r.timestamp,
                "version": r.version,
                "witness_address": bytes_to_hex(&r.witness_address),
                "witness_id": r.witness_id,
                "accountStateRoot": bytes_to_hex(&r.account_state_root),
            })
        });
        json!({
            "raw_data": raw,
            "witness_signature": bytes_to_hex(&h.witness_signature),
        })
    });
    json!({
        "block_header": block_header,
        "transactions": b.transactions.iter().map(transaction_to_value).collect::<Vec<_>>(),
    })
}

/// Build the JSON for a `protocol::TransactionInfo`, matching the shape the
/// legacy `/wallet/gettransactioninfobyid` response produced.
pub(crate) fn transaction_info_to_value(info: &protocol::TransactionInfo) -> Value {
    let logs: Vec<Value> = info
        .log
        .iter()
        .enumerate()
        .map(|(idx, l)| {
            json!({
                "address": bytes_to_hex(&l.address),
                "topics": l.topics.iter().map(|t| bytes_to_hex(t)).collect::<Vec<_>>(),
                "data": bytes_to_hex(&l.data),
                "logIndex": idx as i64,
            })
        })
        .collect();
    let contract_result: Vec<String> = info
        .contract_result
        .iter()
        .map(|c| bytes_to_hex(c))
        .collect();
    json!({
        "id": bytes_to_hex(&info.id),
        "fee": info.fee,
        "blockNumber": info.block_number,
        "blockTimeStamp": info.block_time_stamp,
        "contractResult": contract_result,
        "contract_address": bytes_to_hex(&info.contract_address),
        "log": logs,
        "result": info.result,
        "resMessage": bytes_to_hex(&info.res_message),
        "assetIssueID": info.asset_issue_id,
        "withdrawAmount": info.withdraw_amount,
        "unfreezeAmount": info.unfreeze_amount,
    })
}

/// Build the JSON for a `protocol::Account`, matching the legacy
/// `/wallet/getaccount` response.
pub(crate) fn account_to_value(a: &protocol::Account) -> Value {
    let frozen: Vec<Value> = a
        .frozen
        .iter()
        .map(|f| {
            json!({
                "frozen_balance": f.frozen_balance,
                "expire_time": f.expire_time,
            })
        })
        .collect();
    let votes: Vec<Value> = a
        .votes
        .iter()
        .map(|v| {
            json!({
                "vote_address": bytes_to_hex(&v.vote_address),
                "vote_count": v.vote_count,
            })
        })
        .collect();
    let asset: Map<String, Value> = a.asset.iter().map(|(k, v)| (k.clone(), json!(v))).collect();
    let asset_v2: Map<String, Value> = a
        .asset_v2
        .iter()
        .map(|(k, v)| (k.clone(), json!(v)))
        .collect();
    let account_name = String::from_utf8(a.account_name.clone()).unwrap_or_default();
    json!({
        "account_name": account_name,
        "type": a.r#type,
        "address": bytes_to_hex(&a.address),
        "balance": a.balance,
        "votes": votes,
        "asset": asset,
        "assetV2": asset_v2,
        "frozen": frozen,
        "create_time": a.create_time,
        "latest_opration_time": a.latest_opration_time,
        "allowance": a.allowance,
    })
}

/// Build the JSON for a `protocol::TransactionExtention`, matching the
/// legacy `/wallet/triggerconstantcontract` response. Only `constant_result`
/// (hex) and `result` (default-true to mean "no error" when missing) are
/// surfaced — that's all the wallet-sync parser needs.
pub(crate) fn transaction_extention_to_value(te: &protocol::TransactionExtention) -> Value {
    json!({
        "constant_result": te
            .constant_result
            .iter()
            .map(|v| bytes_to_hex(v))
            .collect::<Vec<_>>(),
        "result": te.result.as_ref().map(|r| r.result).unwrap_or(true),
        "transaction": te.transaction.as_ref().map(transaction_to_value),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_transfer_contract() -> protocol::TransferContract {
        protocol::TransferContract {
            owner_address: vec![0x41; 21],
            to_address: vec![0x42; 21],
            amount: 1_000_000,
        }
    }

    #[test]
    fn unpack_transfer_contract_to_value() {
        let tc = sample_transfer_contract();
        let mut buf = Vec::with_capacity(tc.encoded_len());
        tc.encode(&mut buf).unwrap();
        let any = prost_types::Any {
            type_url: "type.googleapis.com/protocol.TransferContract".into(),
            value: buf,
        };
        let v = unpack_contract_parameter(&any).unwrap();
        assert_eq!(v.get("amount").unwrap(), &json!(1_000_000));
        assert!(
            v.get("owner_address")
                .unwrap()
                .as_str()
                .unwrap()
                .starts_with("0x")
        );
    }

    #[test]
    fn contract_type_name_covers_known_types() {
        let c = protocol::transaction::Contract {
            r#type: protocol::transaction::contract::ContractType::TransferContract as i32,
            parameter: None,
            provider: vec![],
            contract_name: vec![],
            permission_id: 0,
        };
        let v = contract_to_value(&c);
        assert_eq!(v.get("type").unwrap(), &json!("TransferContract"));
    }

    #[test]
    fn block_to_value_includes_transactions() {
        let mut tx = protocol::Transaction::default();
        tx.raw_data = Some(protocol::transaction::Raw::default());
        let block = protocol::Block {
            transactions: vec![tx],
            block_header: None,
        };
        let v = block_to_value(&block);
        assert!(v.get("transactions").unwrap().is_array());
    }
}
