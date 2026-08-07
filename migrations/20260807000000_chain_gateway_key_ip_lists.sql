-- Chain Gateway: per-key IP whitelist/blacklist
-- ip_whitelist: non-empty means only these IPs/CIDRs are allowed
-- ip_blacklist: requests from these IPs/CIDRs are always rejected

ALTER TABLE chain_gateway_keys ADD COLUMN IF NOT EXISTS ip_whitelist TEXT[] NOT NULL DEFAULT '{}';
ALTER TABLE chain_gateway_keys ADD COLUMN IF NOT EXISTS ip_blacklist TEXT[] NOT NULL DEFAULT '{}';
