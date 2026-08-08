# BTC 多输出转账拆解

Date: 2026-08-08  
Scope: `crates/wallet-chain` Bitcoin `fetch_block_txs`；存量脏数据需运维清理后 reindex

## Goal

把一笔 BTC 链上交易按**付款输出**拆成多条 `NormalizedTx` 入库：正确填充 `from`/`to`/`value`，忽略找零，跳过 coinbase；使钱包同步能按收款地址匹配到多条转账记录。

## Decisions

| 主题 | 选择 |
|------|------|
| 拆行粒度 | 每个非找零、有地址的 `vout` 一行（方案 A） |
| `from` | 第一个「不在任何输出地址集合中」的输入地址；否则回退第一个有地址的输入 |
| coinbase | 整笔跳过，不入库 |
| 找零 | 输出地址落在任一输入地址集合中则跳过 |
| 唯一键 | 复用现有 `(chain_index, hash, contract, log_index)`；`log_index = vout.n` |
| RPC | `getblock(hash, 3)`，依赖 `vin[].prevout` |
| 存量数据 | 不自动删库；文档约定先 DELETE 再 reindex |
| DB / API | 无 schema、无 API 契约变更 |

## Current State

`BitcoinChain::fetch_block_txs` 当前行为：

- 每笔链上 tx 只产出 **1** 条 `NormalizedTx`
- `from`/`to` 读 `vin[0].address` / `vout[0].address`（Bitcoin Core JSON 中不存在），导致地址为空
- `value` 为全部输出合计（含找零）
- `getblock` 使用 verbosity `2`，`prevout` 通常缺失

样例脏数据：`from_address`/`to_address` 空，`log_index` NULL，`raw` 仅有 `txid`。

## Design Summary

在 `fetch_block_txs` 内把「链上 tx → 多条付款记录」完成；同步侧 `parser` 的 UTXO normalize 保持只校正 status。拆行后靠 `log_index = vout.n` 与现有 upsert 唯一键对齐。

## Parsing Rules

### RPC

```text
getblockhash(height) → hash
getblock(hash, 3)    → block with vin[].prevout
```

### Per-transaction algorithm

1. 若 `vin[0]` 含 `coinbase` → 跳过整笔。
2. 收集输入地址：`vin[].prevout.scriptPubKey.address`，兼容 `addresses[0]`。
3. 收集输出地址：`vout[].scriptPubKey.address`，兼容同上；记录每个有地址输出的 `(n, address, value_sats)`。
4. 选择 `from`：
   - 优先：第一个不在输出地址集合中的输入地址；
   - 否则：第一个有地址的输入地址；
   - 再否则：`from = None`（并 `tracing::warn`）。
5. 手续费：若 `prevout` 齐全，`fee = Σin − Σout`（satoshi）；否则 `None`。
6. 对每个有地址的输出：
   - 若 `to` 属于任一输入地址 → 找零，跳过；
   - 否则 push 一条 `NormalizedTx`：
     - `hash` = txid  
     - `from` / `to`  
     - `value` = 该输出 satoshi，decimals = 8  
     - `log_index` = `vout.n`  
     - `gas_fee` = 本 tx 手续费（所有付款行相同）  
     - `method` = `"native_transfer"`  
     - `contract_address` = `None`  
     - `status` = `Success`  
     - `block_number` = height  
     - `raw` = `{ "txid", "vout": n }`

### Edge cases

| 情况 | 处理 |
|------|------|
| OP_RETURN / 无地址输出 | 跳过该输出 |
| 全部输出被过滤 | 该 tx 不落库 |
| 多输入地址都出现在输出里 | `from` 回退第一个有地址输入 |
| 同地址多个付款输出 | 多行，不同 `log_index` |
| 节点无 verbosity 3 / 无 prevout | 仍按输出拆行；`from`/fee 可能为空；warn |

## Persistence & Dirty Data

唯一索引已存在：

```sql
(chain_index, hash, COALESCE(contract_address, ''), COALESCE(log_index, -1))
```

旧行 `log_index IS NULL` 与新行 `log_index = vout.n` **不会冲突覆盖**，会并存。

运维步骤（代码不自动执行）：

1. 部署含本设计的 wallet-sync / wallet-chain。
2. 清理 BTC 旧行，例如：`DELETE FROM transactions WHERE chain_index = 0;`（或按高度区间）。
3. 对目标高度触发 `wallet.sync.reindex`。

## Files

| 文件 | 变更 |
|------|------|
| `crates/wallet-chain/src/bitcoin/mod.rs` | verbosity 3；`expand_bitcoin_tx`；单测 |
| `bins/wallet-sync/src/parser.rs` | 无行为变更（可选注释） |
| migrations / proto / API | 无 |

## Tests

纯函数单测（JSON fixture），至少覆盖：

1. 多输出 + 找零 → 只保留付款行，`log_index` 正确  
2. coinbase → 空列表  
3. OP_RETURN + 付款 → 只保留付款  
4. 无 `prevout` → 仍拆行，`from`/`gas_fee` 为空或 None  
5. 多付款输出 → 多行，每行 `gas_fee` 相同

## Out of Scope

- 输入金额与输出的精确配对（方案 C）
- 按 `(from, to)` 聚合（方案 B）
- 启动时自动清理历史 BTC 行
- 修改对外 API / protobuf

## Acceptance

1. 单测通过。  
2. 对含找零的真实块 reindex 后：`from`/`to` 有值，找零不入库，同 txid 多行靠 `log_index` 区分。  
3. 配置仍指向 HTTP 网关口（`8545`），不误用 gRPC `50051`。
