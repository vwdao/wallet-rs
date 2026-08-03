ALTER TABLE chain_gateway_stats ADD COLUMN IF NOT EXISTS client_ip TEXT;
CREATE INDEX IF NOT EXISTS idx_cg_stats_ip ON chain_gateway_stats(client_ip, created_at);
