#!/usr/bin/env bash
# ── Local Development & Service Runner ──────────────────────────────────────
# Usage: ./scripts/dev.sh [command] [options]

set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${ROOT_DIR}"

COLOR_RESET="\033[0m"
COLOR_INFO="\033[36m"
COLOR_SUCCESS="\033[32m"
COLOR_WARN="\033[33m"
COLOR_ERR="\033[31m"

log_info() { echo -e "${COLOR_INFO}[INFO]${COLOR_RESET} $*"; }
log_ok() { echo -e "${COLOR_SUCCESS}[OK]${COLOR_RESET} $*"; }
log_warn() { echo -e "${COLOR_WARN}[WARN]${COLOR_RESET} $*"; }
log_err() { echo -e "${COLOR_ERR}[ERR]${COLOR_RESET} $*"; }

usage() {
    cat <<EOF
Wallet-rs 本地开发与启动管理脚本

用法:
  ./scripts/dev.sh [命令]

基础设施命令:
  infra             启动底层中间件 (PostgreSQL, Redis, NATS, ClickHouse)
  stop-infra        停止底层中间件
  status            检查中间件与本地微服务运行端口状态

单一微服务启动:
  api               启动核心 gRPC 服务 (wallet-api, 监听 :9000)
  app-gw            启动客户端 HTTP API 网关 (wallet-apigw-app, 监听 :8080)
  admin-gw          启动管理端 HTTP API 网关 (wallet-apigw-admin, 监听 :8081)
  chain-gw          启动链 RPC 聚合网关 (wallet-chain-gateway, 监听 :8545, gRPC :50051)
  jobs              启动后台消费与定时任务 (wallet-jobs)
  sync [chain]      启动链区块同步器 (默认: eth, 可选: btc, solana, tron, arb, doge, sui, ton, zec)
  ws                启动 WebSocket 推送服务 (wallet-ws)

一键启动/管理:
  all               先启动中间件，再一键启动核心服务组 (api + chain-gw + app-gw + jobs)
  build             构建全部二进制 (cargo build --bins)
  check             运行语法与类型检查 (cargo check --workspace)
  help              显示本帮助信息
EOF
}

start_infra() {
    log_info "正在启动底层依赖中间件 (PostgreSQL, Redis, NATS, ClickHouse)..."
    docker compose up -d
    log_ok "基础设施容器已启动！"
    log_info "正在等待 PostgreSQL (:5432) 就绪..."
    local retries=15
    while [ $retries -gt 0 ]; do
        if nc -z 127.0.0.1 5432 2>/dev/null || (echo > /dev/tcp/127.0.0.1/5432) 2>/dev/null; then
            break
        fi
        sleep 1
        retries=$((retries - 1))
    done
    if [ $retries -eq 0 ]; then
        log_warn "PostgreSQL 连接检测超时，请检查 docker compose ps"
    else
        log_ok "PostgreSQL 就绪！"
    fi
}

stop_infra() {
    log_info "正在停止底层依赖中间件..."
    docker compose down
    log_ok "基础设施已停止！"
}

check_status() {
    echo "================ [ Docker 中间件状态 ] ================"
    docker compose ps
    echo ""
    echo "================ [ 本地服务端口监听情况 ] ================"
    local ports=(5432 6379 4222 8123 9000 8080 8081 8545 50051)
    local names=("PostgreSQL" "Redis" "NATS" "ClickHouse" "wallet-api" "apigw-app" "apigw-admin" "chain-gateway-http" "chain-gateway-grpc")
    for i in "${!ports[@]}"; do
        local p="${ports[$i]}"
        local n="${names[$i]}"
        if lsof -iTCP:"$p" -sTCP:LISTEN -n -P >/dev/null 2>&1; then
            echo -e "  $n (:$p): \033[32mRUNNING\033[0m"
        else
            echo -e "  $n (:$p): \033[90mSTOPPED\033[0m"
        fi
    done
}

run_api() {
    log_info "启动核心 gRPC 服务 (wallet-api)..."
    cargo run -p wallet-api -- --config configs/wallet-api.yaml
}

run_app_gw() {
    log_info "启动客户端 HTTP API 网关 (wallet-apigw-app)..."
    cargo run -p wallet-apigw-app -- --config configs/apigw-app.yaml
}

run_admin_gw() {
    log_info "启动管理后台 HTTP API 网关 (wallet-apigw-admin)..."
    cargo run -p wallet-apigw-admin -- --config configs/apigw-admin.yaml
}

run_chain_gw() {
    log_info "启动链 RPC 聚合网关 (wallet-chain-gateway)..."
    cargo run -p wallet-chain-gateway -- --config configs/chain-gateway.yaml
}

run_jobs() {
    log_info "启动后台任务与 NATS 消费者 (wallet-jobs)..."
    cargo run -p wallet-jobs -- --config configs/wallet-jobs.yaml
}

run_sync() {
    local chain="${1:-eth}"
    local cfg="configs/wallet-sync-${chain}.yaml"
    if [ ! -f "$cfg" ]; then
        log_err "未找到配置文件: $cfg"
        echo "可用的 sync 配置文件列表:"
        ls configs/wallet-sync-*.yaml | sed 's/configs\/wallet-sync-/  - /' | sed 's/\.yaml//'
        exit 1
    fi
    log_info "启动区块同步器 (wallet-sync: ${chain})..."
    cargo run -p wallet-sync -- --config "$cfg"
}

run_ws() {
    log_info "启动 WebSocket 推送服务 (wallet-ws)..."
    cargo run -p wallet-ws -- --config configs/wallet-ws.yaml
}

run_all() {
    start_infra

    log_info "正在编译全部二进制文件..."
    cargo build --bins

    local pids=()
    cleanup() {
        echo ""
        log_warn "捕获到终止信号，正在关闭所有后台子进程..."
        for pid in "${pids[@]}"; do
            if kill -0 "$pid" 2>/dev/null; then
                kill "$pid" 2>/dev/null || true
            fi
        done
        wait 2>/dev/null || true
        log_ok "所有本地子服务已平稳退出。"
        exit 0
    }
    trap cleanup SIGINT SIGTERM EXIT

    echo ""
    log_info "==> 启动 wallet-api (gRPC :9000)..."
    target/debug/wallet-api --config configs/wallet-api.yaml &
    pids+=($!)
    sleep 2

    log_info "==> 启动 wallet-chain-gateway (:8545)..."
    target/debug/wallet-chain-gateway --config configs/chain-gateway.yaml &
    pids+=($!)
    sleep 1

    log_info "==> 启动 wallet-apigw-app (:8080)..."
    target/debug/wallet-apigw-app --config configs/apigw-app.yaml &
    pids+=($!)

    log_info "==> 启动 wallet-jobs (后台工作进程)..."
    target/debug/wallet-jobs --config configs/wallet-jobs.yaml &
    pids+=($!)

    log_ok "核心本地微服务组已全部就绪！按 Ctrl+C 可一键停止全部微服务。"
    wait
}

CMD="${1:-help}"
shift || true

case "$CMD" in
    infra)
        start_infra
        ;;
    stop-infra)
        stop_infra
        ;;
    status|ps)
        check_status
        ;;
    api)
        run_api
        ;;
    app-gw)
        run_app_gw
        ;;
    admin-gw)
        run_admin_gw
        ;;
    chain-gw)
        run_chain_gw
        ;;
    jobs)
        run_jobs
        ;;
    sync)
        run_sync "$@"
        ;;
    ws)
        run_ws
        ;;
    all)
        run_all
        ;;
    build)
        cargo build --bins "$@"
        ;;
    check)
        cargo check --workspace "$@"
        ;;
    help|--help|-h)
        usage
        ;;
    *)
        log_err "未知命令: $CMD"
        usage
        exit 1
        ;;
esac
