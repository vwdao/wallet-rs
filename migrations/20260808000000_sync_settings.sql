-- Dynamic per-chain sync settings. Overrides the values in wallet-sync.yaml.
-- When no row exists for a chain, wallet-sync falls back to the YAML config.
CREATE TABLE IF NOT EXISTS sync_settings (
    chain_index BIGINT PRIMARY KEY,
    poll_interval_ms BIGINT NOT NULL DEFAULT 2000,
    confirmations BIGINT NOT NULL DEFAULT 12,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
