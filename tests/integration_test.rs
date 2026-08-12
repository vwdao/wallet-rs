//! Integration tests for wallet-domain services

#[cfg(test)]
mod tests {
    use wallet_error::AppResult;
    use wallet_types::{Address, ChainIndex};

    #[test]
    fn test_address_creation() {
        let addr = Address::new("0x742d35Cc6634C0532925a3b844Bc9e7595f2bD3e".to_string());
        assert_eq!(addr.as_str(), "0x742d35Cc6634C0532925a3b844Bc9e7595f2bD3e");
    }

    #[test]
    fn test_chain_index() {
        let eth = ChainIndex::ETH;
        assert_eq!(eth.as_i64(), 60);
        
        let sol = ChainIndex::SOL;
        assert_eq!(sol.as_i64(), 501);
    }

    #[test]
    fn test_chain_family_detection() {
        use wallet_types::ChainFamily;
        
        assert_eq!(
            ChainFamily::for_index(ChainIndex::ETH),
            Some(ChainFamily::Evm)
        );
        assert_eq!(
            ChainFamily::for_index(ChainIndex::SOL),
            Some(ChainFamily::Solana)
        );
        assert_eq!(
            ChainFamily::for_index(ChainIndex::TRON),
            Some(ChainFamily::Tron)
        );
        assert_eq!(
            ChainFamily::for_index(ChainIndex::BTC),
            Some(ChainFamily::Bitcoin)
        );
        assert_eq!(
            ChainFamily::for_index(ChainIndex::ZCASH),
            Some(ChainFamily::Zcash)
        );
        assert_eq!(
            ChainFamily::for_index(ChainIndex::DOGE),
            Some(ChainFamily::UtxoOther)
        );
        assert_eq!(ChainFamily::Zcash.as_str(), "zcash");
    }

    #[test]
    fn test_swap_provider_trait_object_safety() {
        // Verify that SwapProvider is object-safe
        fn _assert_object_safe(_: &dyn wallet_domain::swap::SwapProvider) {}
    }

    #[test]
    fn test_paymaster_trait_object_safety() {
        // Verify that Paymaster is object-safe
        fn _assert_object_safe(_: &dyn wallet_domain::gaspool::Paymaster) {}
    }

    #[test]
    fn test_error_conversion() {
        use wallet_error::AppError;
        
        let err = AppError::NotFound("test".into());
        assert!(matches!(err, AppError::NotFound(_)));
        
        let err = AppError::Unauthorized;
        assert!(matches!(err, AppError::Unauthorized));
        
        let err = AppError::InvalidArgument("bad input".into());
        assert!(matches!(err, AppError::InvalidArgument(_)));
    }

    #[tokio::test]
    async fn test_memory_event_bus() {
        use wallet_events::{EventBus, MemoryEventBus};
        use serde_json::json;
        
        let bus = MemoryEventBus::new(100);
        let mut sub = bus.subscribe("test.>", "test-durable").await.unwrap();
        
        bus.publish("test.hello", json!({"msg": "hello"}))
            .await
            .unwrap();
        
        let ev = sub.next().await.unwrap();
        assert!(ev.is_some());
        assert_eq!(ev.unwrap().subject, "test.hello");
        
        sub.ack_last().await.unwrap();
    }
}
