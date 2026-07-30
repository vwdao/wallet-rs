-- Initial schema

CREATE TABLE IF NOT EXISTS networks (
    chain_index BIGINT PRIMARY KEY,
    name TEXT NOT NULL,
    family TEXT NOT NULL,
    evm_chain_id BIGINT,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS rpc_endpoints (
    id UUID PRIMARY KEY,
    chain_index BIGINT NOT NULL REFERENCES networks(chain_index),
    url TEXT NOT NULL,
    weight INT NOT NULL DEFAULT 1,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    external_id TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS user_addresses (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    chain_index BIGINT NOT NULL,
    address TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(chain_index, address)
);

CREATE TABLE IF NOT EXISTS tokens (
    id UUID PRIMARY KEY,
    chain_index BIGINT NOT NULL,
    address TEXT NOT NULL,
    symbol TEXT NOT NULL,
    name TEXT NOT NULL,
    decimals INT NOT NULL,
    logo_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(chain_index, address)
);

CREATE TABLE IF NOT EXISTS sync_cursors (
    chain_index BIGINT PRIMARY KEY,
    height BIGINT NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY,
    chain_index BIGINT NOT NULL,
    hash TEXT NOT NULL,
    from_address TEXT,
    to_address TEXT,
    value NUMERIC NOT NULL DEFAULT 0,
    block_number BIGINT NOT NULL,
    status TEXT NOT NULL,
    raw JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(chain_index, hash)
);

CREATE INDEX IF NOT EXISTS idx_tx_address ON transactions(chain_index, from_address);
CREATE INDEX IF NOT EXISTS idx_tx_to ON transactions(chain_index, to_address);

CREATE TABLE IF NOT EXISTS swap_providers (
    name TEXT PRIMARY KEY,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    config_json JSONB NOT NULL DEFAULT '{}'
);

CREATE TABLE IF NOT EXISTS gas_pools (
    chain_index BIGINT PRIMARY KEY,
    balance NUMERIC NOT NULL DEFAULT 0,
    enabled BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS dapps (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    url TEXT NOT NULL,
    logo_url TEXT,
    chain_indexes BIGINT[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS app_configs (
    platform TEXT PRIMARY KEY,
    min_version TEXT NOT NULL,
    latest_version TEXT NOT NULL,
    force_update_url TEXT,
    features_json JSONB NOT NULL DEFAULT '{}'
);

CREATE TABLE IF NOT EXISTS guides (
    id UUID PRIMARY KEY,
    locale TEXT NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS chain_gateway_keys (
    id UUID PRIMARY KEY,
    api_key TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    rate_limit_per_min INT NOT NULL DEFAULT 60,
    enabled BOOLEAN NOT NULL DEFAULT TRUE
);
