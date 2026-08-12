-- Chain Gateway: per-network supported RPC protocols.
-- Empty array means auto-detect from the chain's RPC endpoints.

ALTER TABLE networks ADD COLUMN IF NOT EXISTS supported_protocols TEXT[] NOT NULL DEFAULT '{}';
