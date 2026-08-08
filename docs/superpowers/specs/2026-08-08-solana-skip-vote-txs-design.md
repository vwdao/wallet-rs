# Solana sync: skip pure vote transactions

## Goal

When syncing Solana blocks via `getBlock`, do not emit `NormalizedTx` rows for pure validator vote transactions. Keep all other transactions (transfers, SPL, mixed program txs).

## Decision

Filter inside `SolanaChain::fetch_block_txs` (`crates/wallet-chain/src/solana/mod.rs`) before constructing normalized rows (approach 1).

Detection rule (option A): a transaction is skipped only if it has at least one top-level instruction and **every** top-level instruction targets the Vote program:

`Vote111111111111111111111111111111111111111`

## Behavior

- Resolve each instruction’s program id via `message.accountKeys[programIdIndex]`.
- `accountKeys` entries may be plain strings or `{ "pubkey": "..." }` objects.
- Empty instruction list → **not** treated as a vote (keep).
- Any non-Vote program among top-level instructions → keep the full tx (and existing SPL transfer derivation).
- Inner instructions are ignored for classification (vote txs are identified by top-level Vote instructions).
- Slot tip / height advancement unchanged; only persisted tx volume decreases.

## Non-goals

- No RPC-provider-specific “exclude votes” flags.
- No filtering in `wallet-sync` parser.
- No deletion/migration of historically synced vote rows.

## Tests

Unit tests for `is_pure_vote_tx` (or equivalent):

1. All instructions → Vote program → `true`
2. Mix of Vote + System (or other) → `false`
3. No instructions → `false`
4. `accountKeys` as `{pubkey}` objects still classify correctly

## Success criteria

- Pure vote txs from `getBlock` do not appear in `fetch_block_txs` output.
- Non-vote and mixed txs still produce the same normalized rows as today (including SPL deltas).
