-- Chain Gateway v2: enhanced RPC endpoint metadata, key restrictions, request stats

ALTER TABLE rpc_endpoints ADD COLUMN IF NOT EXISTS tier TEXT NOT NULL DEFAULT 'free';
ALTER TABLE rpc_endpoints ADD COLUMN IF NOT EXISTS is_archive BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE rpc_endpoints ADD COLUMN IF NOT EXISTS priority INT NOT NULL DEFAULT 0;
ALTER TABLE rpc_endpoints ADD COLUMN IF NOT EXISTS last_health_check TIMESTAMPTZ;
ALTER TABLE rpc_endpoints ADD COLUMN IF NOT EXISTS healthy BOOLEAN NOT NULL DEFAULT TRUE;
ALTER TABLE rpc_endpoints ADD COLUMN IF NOT EXISTS avg_latency_ms INT;
ALTER TABLE rpc_endpoints ADD COLUMN IF NOT EXISTS error_count INT NOT NULL DEFAULT 0;

ALTER TABLE chain_gateway_keys ADD COLUMN IF NOT EXISTS allowed_chains BIGINT[] NOT NULL DEFAULT '{}';
ALTER TABLE chain_gateway_keys ADD COLUMN IF NOT EXISTS allowed_tier TEXT NOT NULL DEFAULT 'all';
ALTER TABLE chain_gateway_keys ADD COLUMN IF NOT EXISTS total_requests BIGINT NOT NULL DEFAULT 0;
ALTER TABLE chain_gateway_keys ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

CREATE TABLE IF NOT EXISTS chain_gateway_stats (
    id UUID PRIMARY KEY,
    api_key TEXT NOT NULL,
    chain_index BIGINT NOT NULL,
    method TEXT,
    status_code INT NOT NULL,
    latency_ms INT NOT NULL,
    error_msg TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_cg_stats_key ON chain_gateway_stats(api_key, created_at);
CREATE INDEX IF NOT EXISTS idx_cg_stats_chain ON chain_gateway_stats(chain_index, created_at);
