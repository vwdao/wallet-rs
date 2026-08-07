use wallet_error::AppResult;
use wallet_events::{EventBus, SUBJECT_TX_INDEXED};
use wallet_types::{ChainIndex, NormalizedTx};

pub async fn report_txs(
    events: &dyn EventBus,
    chain_index: ChainIndex,
    txs: &[NormalizedTx],
) -> AppResult<()> {
    for tx in txs {
        events
            .publish(
                SUBJECT_TX_INDEXED,
                serde_json::json!({
                    "chain_index": chain_index.as_i64(),
                    "hash": tx.hash.as_str(),
                    "block_number": tx.block_number,
                    "status": tx.status.as_str(),
                    "from": tx.from.as_ref().map(|a| a.as_str()),
                    "to": tx.to.as_ref().map(|a| a.as_str()),
                    "value": tx.value.raw.to_string(),
                    "decimals": tx.value.decimals,
                }),
            )
            .await?;
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
            block_number: 42,
            status,
            raw: serde_json::json!({}),
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
        assert_eq!(ev.payload["block_number"], 42);
        assert_eq!(ev.payload["chain_index"], 60);
    }
}
