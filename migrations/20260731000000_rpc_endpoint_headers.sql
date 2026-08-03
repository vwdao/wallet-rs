-- RPC endpoint custom request headers (JSON object of name -> value)

ALTER TABLE rpc_endpoints ADD COLUMN IF NOT EXISTS headers JSONB NOT NULL DEFAULT '{}';
