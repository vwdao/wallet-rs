-- Seed TON (The Open Network, SLIP-0044 coin type 607) and Sui (SLIP-0044
-- coin type 784) networks. Both are gated behind the `ton` and `sui` feature
-- flags in wallet-chain and are routed by chain-gateway the same way as the
-- other JSON-RPC families.
INSERT INTO networks (chain_index, name, family, evm_chain_id, enabled) VALUES
    (607, 'TON', 'ton', NULL, TRUE),
    (784, 'SUI', 'sui', NULL, TRUE)
ON CONFLICT (chain_index) DO NOTHING;

-- Per-family block-lag defaults that match the new built-in defaults in
-- chain-gateway (masterchain ~5s, Sui checkpoints ~3s).
INSERT INTO gateway_settings (key, value, description) VALUES
    ('ton_max_block_lag', '16', 'TON RPC 端点允许落后 masterchain 顶端的 seqno 数'),
    ('sui_max_block_lag', '32', 'Sui RPC 端点允许落后最新 checkpoint 的个数')
ON CONFLICT (key) DO NOTHING;
