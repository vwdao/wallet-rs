# Solana: filter WSOL wrap noise + backfill SPL from

## Goal

Reduce double-counting of SOL wraps and fill missing SPL `from` when possible.

## Decision (option C)

In `solana_token_transfers` (`crates/wallet-chain/src/solana/mod.rs`):

1. **Drop WSOL wrap noise:** if mint is `So11111111111111111111111111111111111111112` and there is no token-balance loser (gainer-only), do not emit those SPL rows. Native SOL rows already cover the lamport movement.
2. **Backfill `from`:** for remaining SPL rows with no loser, set `from` to the transaction fee payer (`accountKeys[0]`).

Native transfer logic unchanged.

## Non-goals

- Instruction-level wrap/unwrap parsing
- Deleting historical rows

## Tests

- WSOL gainer-only → no SPL rows
- Non-WSOL gainer-only → one row with `from` = fee payer
- Normal SPL loser→gainer still uses loser as `from`
