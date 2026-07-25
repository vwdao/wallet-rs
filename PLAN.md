# Viva Wallet — Rust Multi-Chain Wallet Platform

Greenfield rewrite of the Go `viva_wallet` stack. Shared libraries own the domain;
binaries only assemble. Known chains use **capability traits + `ChainFamily` enum
dispatch**, not a single fat `dyn Chain`.

## Goals & Constraints

- **Strategy**: Full rewrite alongside Go; cut over, then retire Go.
- **Style**: Thick crates, thin binaries; no god-biz package; no second domain proto on gateways.
- **Scope**: EVM family / Solana / Bitcoin / TRON; app + admin gateways; sync / jobs / ws;
  swap, gaspool, market, dapp, rent; webhook; chain-gateway. Former `client-backend`
  lives as the **cms** module inside `wallet-api`.

## Corrections vs the First Draft

| Earlier draft | This plan |
|---|---|
| Fat `Chain` + `HashMap<dyn Chain>` | Capability traits + `ChainHandle` / registry by `ChainIndex` |
| Toasty | **sqlx** + SQL migrations |
| Salvo | **axum** |
| Middleware in `common` | `wallet-types` / `wallet-error` / `wallet-config`; HTTP/gRPC middleware in binaries |
| Kafka→NATS hard-wired in domain | `wallet-events::EventBus`; **NATS JetStream** impl |
| Three proto families | One `proto/wallet/v1` (incl. `admin/`); gateways use HTTP DTOs only |

## Topology

```text
App / Website          Admin SPA           Partners / Webhooks
     │                     │                        │
     ▼                     ▼                        ▼
wallet-apigw-app    wallet-apigw-admin    wallet-apigw-webhook
     │                     │                        │
     └──────────► wallet-api (gRPC) ◄───────────────┘
                       │
         wallet-domain │ wallet-db │ wallet-chain │ wallet-events
                       │
     wallet-sync   wallet-jobs   wallet-ws   wallet-chain-gateway
```

## Workspace Layout

```text
wallet/
├── Cargo.toml
├── Makefile
├── docker-compose.yaml
├── PLAN.md
├── CUTOVER.md
├── proto/wallet/v1/           # sole gRPC contract
├── migrations/                # sqlx
├── configs/
├── crates/
│   ├── wallet-types/
│   ├── wallet-error/
│   ├── wallet-config/
│   ├── wallet-proto/
│   ├── wallet-chain/          # features: evm, solana, bitcoin, tron, full
│   ├── wallet-db/
│   ├── wallet-events/
│   ├── wallet-domain/         # user token tx network swap gaspool market dapp rent cms
│   └── wallet-test-support/
└── bins/
    ├── wallet-api/
    ├── wallet-apigw-app/
    ├── wallet-apigw-admin/
    ├── wallet-apigw-webhook/
    ├── wallet-sync/
    ├── wallet-jobs/
    ├── wallet-ws/
    └── wallet-chain-gateway/
```

## Chain Capability Traits

```rust
#[async_trait]
pub trait BalanceReader: Send + Sync {
    async fn native_balance(&self, addr: &Address) -> Result<Amount>;
}

#[async_trait]
pub trait TokenBalance: Send + Sync {
    async fn token_balance(&self, wallet: &Address, token: &Address) -> Result<Amount>;
}

#[async_trait]
pub trait TxBroadcaster: Send + Sync {
    async fn send_raw(&self, raw: &[u8]) -> Result<TxHash>;
}

#[async_trait]
pub trait BlockSource: Send + Sync {
    async fn tip(&self) -> Result<u64>;
    async fn fetch_block_txs(&self, height: u64) -> Result<Vec<NormalizedTx>>;
}

#[async_trait]
pub trait GasEstimator: Send + Sync {
    async fn estimate_gas(&self, tx: &GasEstimateRequest) -> Result<GasEstimate>;
}

#[async_trait]
pub trait NonceProvider: Send + Sync {
    async fn nonce(&self, addr: &Address) -> Result<u64>;
}
```

- EVM networks share `EvmChain`; differences live in config (chain id, RPC, confirmations).
- Features: `evm`, `solana`, `bitcoin`, `tron`, `full`.

## Domain Modules (`wallet-domain`)

Submodules (not separate crates): `user`, `token`, `transaction`, `network`, `swap`,
`gaspool`, `market`, `dapp`, `rent`, `cms`.

Each module exposes application services that depend on repos / chain / `EventBus`.
gRPC and HTTP types must not leak into domain.

## Tech Stack

| Layer | Choice |
|-------|--------|
| gRPC | tonic + prost |
| HTTP | axum |
| OLTP | sqlx (Postgres) |
| Analytics | clickhouse-rs |
| Cache | fred |
| Events | async-nats JetStream via `EventBus` |
| EVM | alloy |
| Auth | jsonwebtoken; admin RBAC casbin |
| Observability | tracing (+ OTel hooks) |
| Config | serde_yaml |

**Explicitly not used**: Toasty, Salvo, domain code calling NATS APIs directly,
duplicate domain protos on gateways.

## Phases

### P0 — Workspace foundation
Workspace, Makefile, docker-compose, `wallet-types` / `wallet-error` / `wallet-config`,
proto skeleton + `wallet-proto`.

### P1 — Chain & events
Capability traits; EVM family + Solana / Bitcoin / TRON; `EventBus` + JetStream subjects.

### P2 — Data
Migrations and repos; ClickHouse wrappers for kline / dex.

### P3 — Domain + API
Full proto surface; domain services; `wallet-api` (core + admin + cms).

### P4 — HTTP edge
`wallet-apigw-app`, `wallet-apigw-admin` (JWT + casbin), webhook, chain-gateway.

### P5 — Workers & realtime
`wallet-sync` (one chainIndex per process), `wallet-jobs`, `wallet-ws`.

### P6 — Business depth
Swap adapters (trait), gaspool paymasters, market feeds, rent, dapp/cms jobs.

### P7 — Hardening
Integration tests, RPC failover, metrics, [CUTOVER.md](CUTOVER.md).

## Success Criteria

- New chain ≈ new `wallet-chain` impl + config/feature; domain core untouched.
- New API ≈ proto + domain service + optional gateway handler; no cross-layer models.
- Sync scale-out ≈ more `wallet-sync` replicas/config only.
- Proto/service surface complete by P3; Go feature depth by P6.
