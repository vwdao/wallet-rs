-- Track latest block height per RPC endpoint for consistency filtering

ALTER TABLE rpc_endpoints ADD COLUMN IF NOT EXISTS block_height BIGINT;
