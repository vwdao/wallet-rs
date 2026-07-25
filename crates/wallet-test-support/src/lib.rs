//! Test helpers and fixtures.

use std::sync::Arc;
use wallet_config::{ChainRuntimeConfig, RpcEndpoint};
use wallet_events::{EventBus, MemoryEventBus};

pub fn memory_bus() -> Arc<dyn EventBus> {
    MemoryEventBus::new(64)
}

pub fn sample_evm_config() -> ChainRuntimeConfig {
    ChainRuntimeConfig {
        chain_index: 60,
        family: "evm".into(),
        evm_chain_id: Some(1),
        endpoints: vec![RpcEndpoint {
            url: "https://eth.llamarpc.com".into(),
            weight: 1,
        }],
        confirmations: 12,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wallet_chain::build_registry;

    #[tokio::test]
    async fn builds_evm_registry() {
        let reg = build_registry(&[sample_evm_config()]).expect("registry");
        assert!(reg.get(wallet_types::ChainIndex::ETH).is_ok());
    }

    #[tokio::test]
    async fn memory_bus_roundtrip() {
        let bus = memory_bus();
        let mut sub = bus.subscribe("wallet.test", "t").await.unwrap();
        bus.publish("wallet.test", serde_json::json!({"ok": true}))
            .await
            .unwrap();
        let ev = tokio::time::timeout(std::time::Duration::from_secs(2), sub.next())
            .await
            .expect("timeout")
            .unwrap()
            .unwrap();
        assert_eq!(ev.subject, "wallet.test");
    }
}
