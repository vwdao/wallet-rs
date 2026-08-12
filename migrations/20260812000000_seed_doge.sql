-- Seed Dogecoin (DOGE, SLIP-0044 coin type 3). DOGE runs a
-- Bitcoin-Core-compatible JSON-RPC, so it is registered under the `bitcoin`
-- family protocol (getblockcount / getblock verbosity 3 / estimatesmartfee).
-- The chain-gateway resolves `/rpc/doge/...` by network name and probes it
-- with the Bitcoin-style health check, so no per-family settings are needed.
INSERT INTO networks (chain_index, name, family, evm_chain_id, enabled) VALUES
    (3, 'DOGE', 'bitcoin', NULL, TRUE)
ON CONFLICT (chain_index) DO NOTHING;
