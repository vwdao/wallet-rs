use wallet_events::{EventBus, SUBJECT_TX_INDEXED};
use wallet_error::AppResult;
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
                }),
            )
            .await?;
    }
    Ok(())
}
