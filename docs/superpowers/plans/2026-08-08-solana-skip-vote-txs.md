# Solana Skip Pure Vote Txs Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Skip pure Vote-program transactions when syncing Solana blocks via `fetch_block_txs`.

**Architecture:** Add `is_pure_vote_tx` in `crates/wallet-chain/src/solana/mod.rs` and filter inside `SolanaChain::fetch_block_txs` before building `NormalizedTx` rows. Classify using top-level instructions + `accountKeys[programIdIndex]` against the Vote program id.

**Tech Stack:** Rust, `serde_json::Value`, existing `wallet-chain` unit tests.

## Global Constraints

- Pure vote = every top-level instruction targets `Vote111111111111111111111111111111111111111`
- Empty instructions → not a vote (keep)
- Do not filter in `wallet-sync` parser
- No historical DB cleanup

---

## File map

| File | Responsibility |
|------|----------------|
| `crates/wallet-chain/src/solana/mod.rs` | `is_pure_vote_tx`, filter in `fetch_block_txs`, unit tests |

---

### Task 1: Detector + filter (TDD)

**Files:**
- Modify: `crates/wallet-chain/src/solana/mod.rs`

- [x] **Step 1: Write failing tests** for `is_pure_vote_tx`:
  - all instructions → Vote → `true`
  - Vote + System → `false`
  - no instructions → `false`
  - `accountKeys` as `{pubkey}` objects → still `true` for pure vote

- [x] **Step 2: Run tests, confirm fail**

```bash
cargo test -p wallet-chain is_pure_vote
```

- [x] **Step 3: Implement** `VOTE_PROGRAM_ID`, `account_key_str`, `is_pure_vote_tx`, and `continue` in `fetch_block_txs` when pure vote

- [x] **Step 4: Run tests, confirm pass**

```bash
cargo test -p wallet-chain is_pure_vote
cargo test -p wallet-chain solana
```
