use std::sync::Arc;
use wallet_events::{EventBus, SUBJECT_DEX_TRADE, SUBJECT_TX_INDEXED};

use crate::hub::Hub;

pub async fn fanout(events: Arc<dyn EventBus>, hub: Arc<Hub>) {
    for subject in [SUBJECT_TX_INDEXED, SUBJECT_DEX_TRADE] {
        let events = events.clone();
        let hub = hub.clone();
        tokio::spawn(async move {
            let Ok(mut sub) = events.subscribe(subject, &format!("wallet-ws-{subject}")).await else {
                return;
            };
            loop {
                match sub.next().await {
                    Ok(Some(ev)) => {
                        hub.publish(&ev.subject, ev.payload);
                        let _ = sub.ack_last().await;
                    }
                    Ok(None) => break,
                    Err(_) => tokio::time::sleep(std::time::Duration::from_secs(1)).await,
                }
            }
        });
    }
}
