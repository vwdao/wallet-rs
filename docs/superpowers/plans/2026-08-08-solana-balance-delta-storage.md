# Solana Balance-Delta Storage Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Persist only Solana native/SPL balance-delta transfers; drop shell rows.

**Architecture:** In `fetch_block_txs`, after vote filter, emit `solana_native_transfers` + existing `solana_token_transfers` only. Native rows come from fee-adjusted `preBalances`/`postBalances` matched like SPL losers/gainers.

**Tech Stack:** Rust, `serde_json`, `wallet-chain` unit tests.

## Global Constraints

- No shell `accountKeys[0]/[1]` row
- Fee added back to fee payer before matching
- `method=native_transfer` for native rows
- No historical DB cleanup

---

### Task 1: Native transfer helper + drop shell (TDD)

**Files:**
- Modify: `crates/wallet-chain/src/solana/mod.rs`

- [x] Write failing tests for `solana_native_transfers` (SOL move, fee-neutral, create/close net-zero)
- [x] Confirm RED
- [x] Implement helper; remove shell push; extend native + token rows
- [x] Confirm GREEN: `cargo test -p wallet-chain solana::`
