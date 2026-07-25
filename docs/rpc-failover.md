# RPC failover

`wallet-chain::RpcPool` rotates endpoints round-robin and trips a per-endpoint
circuit breaker after consecutive failures.

## Rules (aligned with Go `rpc-auto-failover-rules`)

1. Prefer highest `weight` endpoints first when recovering from open circuit
2. Open circuit after 3 consecutive transport/RPC errors
3. Cool-down 30s before retrying an open endpoint
4. If all endpoints open → return `Unavailable` to callers (API maps to gRPC Unavailable)
5. Admin can upsert endpoints via `AdminRpcEndpointService`; sync/api rebuild registry on restart

## Future

- Hot reload registry from DB poller (Go `WalletRPCPollHandle` parity)
- Health probe job publishing endpoint scores to Redis
