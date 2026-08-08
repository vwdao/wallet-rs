use futures::stream::{FuturesUnordered, StreamExt};
use wallet_error::AppResult;
use wallet_events::{EventBus, SUBJECT_TX_INDEXED};
use wallet_types::{ChainIndex, NormalizedTx};

/// Bounded concurrency for NATS publishes per block. NATS JetStream publish
/// awaits the broker ack, so doing N of them serially becomes the dominant
/// cost on dense blocks; bounded parallelism keeps the network pipeline full
/// without overwhelming the broker.
const REPORT_INFLIGHT: usize = 32;

pub async fn report_txs(
    events: &dyn EventBus,
    chain_index: ChainIndex,
    txs: &[NormalizedTx],
) -> AppResult<()> {
    if txs.is_empty() {
        return Ok(());
    }
    // Build the payloads up front so the publish futures are cheap to poll.
    let payloads: Vec<serde_json::Value> = txs
        .iter()
        .map(|tx| {
            serde_json::json!({
                "chain_index": chain_index.as_i64(),
                "hash": tx.hash.as_str(),
                "block_number": tx.block_number,
                "status": tx.status.as_str(),
                "from": tx.from.as_ref().map(|a| a.as_str()),
                "to": tx.to.as_ref().map(|a| a.as_str()),
                "value": tx.value.raw.to_string(),
                "decimals": tx.value.decimals,
                "gas_fee": tx.gas_fee.as_ref().map(|a| a.raw.to_string()),
                "gas_fee_decimals": tx.gas_fee.as_ref().map(|a| a.decimals),
                "contract_address": tx.contract_address.as_ref().map(|a| a.as_str()),
                "method": tx.method.as_deref(),
                "log_index": tx.log_index,
            })
        })
        .collect();
    let mut pending: FuturesUnordered<_> = FuturesUnordered::new();
    for payload in payloads {
        pending.push(events.publish(SUBJECT_TX_INDEXED, payload));
        if pending.len() >= REPORT_INFLIGHT {
            if let Some(res) = pending.next().await {
                res?;
            }
        }
    }
    while let Some(res) = pending.next().await {
        res?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use wallet_events::MemoryEventBus;
    use wallet_types::{Amount, TxHash, TxStatus};

    fn tx(hash: &str, status: TxStatus) -> NormalizedTx {
        NormalizedTx {
            hash: TxHash::new(hash),
            from: Some(wallet_types::Address::new("0x111")),
            to: Some(wallet_types::Address::new("0x222")),
            value: Amount::new(Decimal::from(100), 18),
            gas_fee: Some(Amount::new(Decimal::from(100), 18)),
            block_number: 42,
            status,
            raw: serde_json::json!({}),
            contract_address: None,
            log_index: None,
            method: None,
        }
    }
    #[tokio::test]
    async fn test_report_includes_status() {
        let bus = MemoryEventBus::new(64);
        let mut sub = bus
            .subscribe(SUBJECT_TX_INDEXED, "test-reporter")
            .await
            .unwrap();

        report_txs(bus.as_ref(), ChainIndex::ETH, &[tx("0xabc", TxStatus::Failed)])
            .await
            .unwrap();

        let ev = sub.next().await.unwrap().unwrap();
        assert_eq!(ev.payload["hash"], "0xabc");
        assert_eq!(ev.payload["status"], "failed");
        assert_eq!(ev.payload["from"], "0x111");
        assert_eq!(ev.payload["to"], "0x222");
        assert_eq!(ev.payload["value"], "100");
        assert_eq!(ev.payload["gas_fee"], "100");
        assert_eq!(ev.payload["block_number"], 42);
        assert_eq!(ev.payload["chain_index"], 60);
    }
}
