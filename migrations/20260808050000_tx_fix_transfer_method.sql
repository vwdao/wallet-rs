-- Token transfer rows derived from `Transfer(address,address,uint256)` logs
-- should carry method='transfer'; previously some inherited the tx calldata
-- selector (e.g. an unknown 8-hex aggregator/swap selector). Fix existing rows
-- whose method is exactly an 8-char hex selector.
UPDATE transactions
SET method = 'transfer'
WHERE contract_address IS NOT NULL
  AND method ~* '^[0-9a-f]{8}$';
