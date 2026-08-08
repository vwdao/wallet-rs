-- Multiple transfers can share one tx hash; disambiguate rows with log_index.
-- `method` records the function selector/name that produced the transfer.
ALTER TABLE transactions ADD COLUMN IF NOT EXISTS log_index BIGINT;
ALTER TABLE transactions ADD COLUMN IF NOT EXISTS method TEXT;
DROP INDEX IF EXISTS uq_tx_chain_hash_contract;
CREATE UNIQUE INDEX IF NOT EXISTS uq_tx_chain_hash_contract_log
    ON transactions(chain_index, hash, contract_address, log_index);
