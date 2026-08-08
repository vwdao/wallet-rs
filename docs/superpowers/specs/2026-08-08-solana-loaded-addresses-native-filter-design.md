# Solana: loadedAddresses + skip token-account native rows

## Problem

Jupiter / Pump swaps use v0 transactions with Address Lookup Tables. Sync only read static `message.accountKeys`, so `pre/postBalances` indices past the static list were dropped. That produced native rows with empty `from`, and double-counted WSOL ATA lamports that Solscan shows only as SPL transfers.

## Fix

1. `resolved_account_keys` = static keys + `meta.loadedAddresses.writable` + `readonly`.
2. Native derivation uses the resolved list.
3. Skip native **gainers** (and loser-only emits) whose address appears as a token account in `pre/postTokenBalances` — those lamport moves are covered by SPL rows.

## Expected for the sample swap

- jobcoin SPL + real WSOL SPL fee/tip legs kept
- No native row to WSOL ATA fee recipients
- User net SOL redeem has a non-empty `from` (largest lamport loser, typically pool/vault after ALT resolution)
