-- Seed Zcash (ZEC, SLIP-0044 coin type 133). Zcash is gated behind the
-- `zcash` feature flag in wallet-chain and uses zcashd-compatible JSON-RPC.
-- Unlike the Bitcoin family, its `getblock` only supports verbosity 2 (no
-- `prevout`) and shielded transactions are not addressable, so the gateway
-- health probe still uses the Bitcoin-style `getblockcount`.
INSERT INTO networks (chain_index, name, family, evm_chain_id, enabled) VALUES
    (133, 'ZEC', 'zcash', NULL, TRUE)
ON CONFLICT (chain_index) DO NOTHING;

-- Zcash blocks are ~75s apart, so the generic `max_block_lag` budget (10
-- blocks) is more than enough; no dedicated setting is needed.
