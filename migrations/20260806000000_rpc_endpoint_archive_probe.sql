-- Track when the gateway last auto-probed an EVM endpoint's archive capability.
-- The is_archive flag is (re)computed from an eth_getBalance probe against a
-- very old block; this column prevents probing on every health cycle and lets
-- the health checker avoid clobbering admin-set values too aggressively.

ALTER TABLE rpc_endpoints ADD COLUMN IF NOT EXISTS archive_checked_at TIMESTAMPTZ;
