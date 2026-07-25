# Cutover checklist (Go → Rust)

## Preconditions

- [ ] `wallet-api` serves full proto surface and passes contract tests against app/admin clients
- [ ] `wallet-sync` caught up for each production chainIndex (cursor lag < confirmations)
- [ ] NATS JetStream `WALLET` stream durable; consumers (`wallet-jobs`, `wallet-ws`) healthy
- [ ] Postgres migrations applied; ClickHouse `kline_1m` / `dex_trade` present
- [ ] chain-gateway API keys migrated; rate limits verified
- [ ] Observability: traces + metrics dashboards green for 48h in staging

## Traffic shift

1. Shadow: route a % of read traffic (networks/tokens/balances) to Rust via gateway canary
2. Dual-write sync cursors off for canary chains; compare tip lag vs Go
3. Flip app BFF DNS/ingress to `wallet-apigw-app`
4. Flip admin BFF to `wallet-apigw-admin`
5. Stop Go `walletcorechain*` for migrated chains
6. Stop Go `walletcore` / async after write path validation

## Rollback

- Keep Go images and Helm values for one release cycle
- Gateway feature flag to force traffic back to Go gRPC
- Do not drop Go DB until checksum reconciliation completes

## Data reconciliation

- Row counts: users, tokens, transactions_{chain}, gas_pools
- Spot-check balances for top wallets vs RPC
- DEX trade volume last 24h: ClickHouse Rust vs Go
