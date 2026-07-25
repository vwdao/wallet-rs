# Viva Wallet (Rust)

Multi-chain wallet platform (EVM / Solana / Bitcoin / TRON). See [PLAN.md](PLAN.md).

## Quick start

```bash
docker compose up -d
cargo check --workspace
cargo run -p wallet-api -- --config configs/wallet-api.yaml
```

## Binaries

| Binary | Role |
|--------|------|
| `wallet-api` | gRPC core + admin + cms |
| `wallet-apigw-app` | App HTTP BFF |
| `wallet-apigw-admin` | Admin HTTP BFF |
| `wallet-apigw-webhook` | Webhook ingress |
| `wallet-sync` | Per-chain block sync |
| `wallet-jobs` | Cron + NATS consumers |
| `wallet-ws` | WebSocket fan-out |
| `wallet-chain-gateway` | JSON-RPC proxy |

## Cutover

See [CUTOVER.md](CUTOVER.md).
