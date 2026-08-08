-- Record the native gas fee paid for a transaction.
-- Raw integer (wei / sun / lamports / sats); decimals are chain-specific.
ALTER TABLE transactions ADD COLUMN IF NOT EXISTS gas_fee NUMERIC;
