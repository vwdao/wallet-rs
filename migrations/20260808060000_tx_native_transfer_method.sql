-- Native value transfer rows carry method='transfer'; previously some native
-- rows inherited the tx calldata selector (e.g. an opaque 8-hex aggregator
-- selector). Fix existing rows whose method is exactly an 8-char hex selector.
UPDATE transactions
SET method = 'transfer'
WHERE contract_address IS NULL
  AND method ~* '^[0-9a-f]{8}$';
