# Solana sync: store balance-delta transfers only

## Goal

Stop writing meaningless Solana “shell” rows (`feePayer → accountKeys[1]`, `value=0`). Persist only rows that reflect real SOL or SPL balance movement.

## Decision

In `SolanaChain::fetch_block_txs` (`crates/wallet-chain/src/solana/mod.rs`):

1. Skip pure vote txs (existing).
2. **Do not** emit the shell `NormalizedTx` from `accountKeys[0]/[1]`.
3. Emit native SOL transfers derived from `meta.preBalances` / `meta.postBalances`.
4. Emit SPL transfers derived from `meta.preTokenBalances` / `meta.postTokenBalances` (existing `solana_token_transfers`).

## Native SOL derivation

- For each account index `i`: `delta[i] = postBalances[i] - preBalances[i]`.
- Add `meta.fee` back onto the fee payer (`accountKeys[0]`) so fee is not treated as a transfer.
- Resolve addresses via `accountKeys` (string or `{pubkey}`), same helper as today.
- Skip zero deltas.
- Match losers → gainers like SPL (largest loser as primary sender; each gainer gets a row; losers-only → burn-style rows with `to=None`).
- Row shape:
  - `method`: `native_transfer`
  - `contract_address`: `None`
  - `value`: lamports, decimals `9`
  - `gas_fee`: same attachment style as current SPL rows
  - `log_index`: sequence within native rows for the signature
  - `raw`: `{ signature, type: "native_transfer" }`

## Create / close accounts

No explicit blacklist. Temporary accounts that are created and closed in the same tx typically net to zero lamports and produce no native row. Noise WSOL token-balance rows from wrap/unwrap remain possible under this scope (out of scope for further filtering).

## Non-goals

- Instruction-level parsing of System/Token programs
- Filtering small WSOL rent/close token deltas
- Migrating or deleting already-synced shell rows

## Tests

1. Shell row is not produced for a tx that only has accountKeys + empty token balances.
2. Native SOL transfer: fee-adjusted pre/post balances yield correct from/to/value.
3. Create+close temp account with net-zero balances → no native row for that address.
4. Existing SPL transfer unit tests still pass.

## Success criteria

- Rows like `to=DVmtwNCk...` with `value=0` and empty method no longer appear for create/close patterns.
- Real SOL and SPL movements still appear with non-zero `value` where applicable.
