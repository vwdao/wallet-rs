-- Solana slots are ~400ms; allow larger lag than EVM/Bitcoin block lag.
INSERT INTO gateway_settings (key, value, description) VALUES
    ('solana_max_block_lag', '64', 'Solana RPC 端点允许落后最高 slot 的数量')
ON CONFLICT (key) DO NOTHING;
