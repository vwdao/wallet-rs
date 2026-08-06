ALTER TABLE chain_gateway_stats ADD COLUMN IF NOT EXISTS protocol TEXT;
CREATE INDEX IF NOT EXISTS idx_cg_stats_ip_protocol ON chain_gateway_stats(client_ip, protocol, created_at);
