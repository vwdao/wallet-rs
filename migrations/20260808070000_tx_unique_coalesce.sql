-- Make the transaction dedup key NULL-safe.
--
-- The previous unique index on (chain_index, hash, contract_address, log_index)
-- treats NULLs as distinct in PostgreSQL, so re-fetching a block could insert
-- duplicate native-transfer rows (contract_address NULL, log_index NULL).
--
-- Indexing COALESCE'd expressions gives `ON CONFLICT` a matching arbiter and
-- enforces a single native row per (chain, hash). Existing data has no
-- duplicates under the new key, so the index can be replaced in place.
DROP INDEX IF EXISTS uq_tx_chain_hash_contract_log;
CREATE UNIQUE INDEX IF NOT EXISTS uq_tx_chain_hash_contract_log
    ON transactions(
        chain_index,
        hash,
        COALESCE(contract_address, ''),
        COALESCE(log_index, -1)
    );
