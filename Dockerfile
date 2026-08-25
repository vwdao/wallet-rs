# ── Stage 0: Build the Vue admin console ───────────────────────────────
FROM node:22-alpine AS admin-ui-builder

WORKDIR /app
COPY bins/wallet-chain-gateway/admin-ui/package*.json bins/wallet-chain-gateway/admin-ui/
RUN cd bins/wallet-chain-gateway/admin-ui && npm ci
COPY bins/wallet-chain-gateway/admin-ui/next.config.mjs bins/wallet-chain-gateway/admin-ui/
COPY bins/wallet-chain-gateway/admin-ui/app bins/wallet-chain-gateway/admin-ui/app
COPY bins/wallet-chain-gateway/admin-ui/components bins/wallet-chain-gateway/admin-ui/components
COPY bins/wallet-chain-gateway/admin-ui/lib bins/wallet-chain-gateway/admin-ui/lib
COPY bins/wallet-chain-gateway/admin-ui/scripts bins/wallet-chain-gateway/admin-ui/scripts
RUN cd bins/wallet-chain-gateway/admin-ui && npm run build

# ── Stage 1: Build ──────────────────────────────────────────────────────
FROM rust:1.97-bookworm AS builder

RUN apt-get update && apt-get install -y protobuf-compiler pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache dependencies: copy manifests first
COPY Cargo.toml Cargo.lock ./
COPY crates/wallet-types/Cargo.toml crates/wallet-types/Cargo.toml
COPY crates/wallet-error/Cargo.toml crates/wallet-error/Cargo.toml
COPY crates/wallet-config/Cargo.toml crates/wallet-config/Cargo.toml
COPY crates/wallet-proto/Cargo.toml crates/wallet-proto/Cargo.toml
COPY crates/wallet-chain/Cargo.toml crates/wallet-chain/Cargo.toml
COPY crates/wallet-db/Cargo.toml crates/wallet-db/Cargo.toml
COPY crates/wallet-events/Cargo.toml crates/wallet-events/Cargo.toml
COPY crates/wallet-domain/Cargo.toml crates/wallet-domain/Cargo.toml
COPY crates/wallet-test-support/Cargo.toml crates/wallet-test-support/Cargo.toml
COPY bins/wallet-api/Cargo.toml bins/wallet-api/Cargo.toml
COPY bins/wallet-apigw-app/Cargo.toml bins/wallet-apigw-app/Cargo.toml
COPY bins/wallet-apigw-admin/Cargo.toml bins/wallet-apigw-admin/Cargo.toml
COPY bins/wallet-chain-gateway/Cargo.toml bins/wallet-chain-gateway/Cargo.toml
COPY bins/wallet-sync/Cargo.toml bins/wallet-sync/Cargo.toml
COPY bins/wallet-jobs/Cargo.toml bins/wallet-jobs/Cargo.toml
COPY bins/wallet-ws/Cargo.toml bins/wallet-ws/Cargo.toml

# Create dummy src files for dependency caching
RUN for dir in crates/wallet-types crates/wallet-error crates/wallet-config crates/wallet-proto \
    crates/wallet-chain crates/wallet-db crates/wallet-events crates/wallet-domain crates/wallet-test-support \
    bins/wallet-api bins/wallet-apigw-app bins/wallet-apigw-admin \
    bins/wallet-chain-gateway bins/wallet-sync bins/wallet-jobs bins/wallet-ws; do \
    mkdir -p "$dir/src" && echo "fn main() {}" > "$dir/src/main.rs" 2>/dev/null || true; \
    echo "" > "$dir/src/lib.rs" 2>/dev/null || true; \
done

# Build dependencies (cached layer)
RUN cargo build --release 2>/dev/null || true
RUN rm -rf crates/*/src bins/*/src

# Copy real source
COPY proto proto
COPY crates crates
COPY bins bins
COPY migrations migrations

# The gateway embeds the generated admin UI at compile time (#[folder = "static/admin"]).
RUN rm -rf bins/wallet-chain-gateway/static && mkdir -p bins/wallet-chain-gateway/static/admin
COPY --from=admin-ui-builder /app/bins/wallet-chain-gateway/admin-ui/out/ bins/wallet-chain-gateway/static/admin/

# Touch to invalidate cache for actual source
RUN find crates bins -name "*.rs" -exec touch {} +

# Build all binaries
RUN cargo build --release --bins

# ── Stage 2: Runtime ────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates curl && \
    rm -rf /var/lib/apt/lists/* && \
    useradd -r -s /bin/false wallet

WORKDIR /app

COPY --from=builder /app/target/release/wallet-api /app/wallet-api
COPY --from=builder /app/target/release/wallet-apigw-app /app/wallet-apigw-app
COPY --from=builder /app/target/release/wallet-apigw-admin /app/wallet-apigw-admin
COPY --from=builder /app/target/release/wallet-chain-gateway /app/wallet-chain-gateway
COPY --from=builder /app/target/release/wallet-sync /app/wallet-sync
COPY --from=builder /app/target/release/wallet-jobs /app/wallet-jobs
COPY --from=builder /app/target/release/wallet-ws /app/wallet-ws

RUN chown -R wallet:wallet /app

# SQL migrations are read at runtime from <crate>/../../migrations (i.e. /app/migrations)
COPY --from=builder /app/migrations /app/migrations

# Default: show available binaries
CMD ["ls", "-la", "/app/"]
