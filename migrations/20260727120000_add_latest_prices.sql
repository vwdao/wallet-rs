-- Add latest_prices table for real-time price cache
CREATE TABLE IF NOT EXISTS latest_prices (
    symbol TEXT PRIMARY KEY,
    price NUMERIC NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Add dex_quotes table for swap event persistence
CREATE TABLE IF NOT EXISTS dex_quotes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chain_index BIGINT NOT NULL,
    wallet TEXT NOT NULL,
    from_token TEXT NOT NULL,
    to_token TEXT NOT NULL,
    amount TEXT NOT NULL,
    price_impact TEXT NOT NULL DEFAULT '0',
    tx_hash TEXT UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_dex_quotes_chain ON dex_quotes(chain_index, created_at DESC);

-- Add missing columns to gas_pools
ALTER TABLE gas_pools ADD COLUMN IF NOT EXISTS hot_wallet TEXT NOT NULL DEFAULT '';
ALTER TABLE gas_pools ADD COLUMN IF NOT EXISTS cold_wallet TEXT NOT NULL DEFAULT '';

-- Add missing columns to transactions
ALTER TABLE transactions ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ;
