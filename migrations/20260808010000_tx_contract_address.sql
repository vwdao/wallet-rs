-- Add token contract address to synced transactions (ERC20 / SPL / TRC20 transfers).
-- Native transfers keep contract_address NULL; token transfers dedup by (chain_index, hash, contract_address).
ALTER TABLE transactions ADD COLUMN IF NOT EXISTS contract_address TEXT;
ALTER TABLE transactions DROP CONSTRAINT IF EXISTS transactions_chain_index_hash_key;
CREATE UNIQUE INDEX IF NOT EXISTS uq_tx_chain_hash_contract
    ON transactions(chain_index, hash, contract_address);
