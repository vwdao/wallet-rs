#!/usr/bin/env bash
# ──────────────────────────────────────────────────────────────────────────
# wallet k3s deploy helper — supports dev/test/prod overlays
# ──────────────────────────────────────────────────────────────────────────
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
K8S_DIR="$SCRIPT_DIR/k8s"
IMAGE="${WALLET_IMAGE:-wallet:latest}"
ENV="${ENV:-dev}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

log()  { echo -e "${GREEN}[deploy]${NC} $*"; }
warn() { echo -e "${YELLOW}[deploy]${NC} $*"; }
err()  { echo -e "${RED}[deploy]${NC} $*" >&2; }

ns_for_env() {
    case "$ENV" in
        dev)  echo "wallet-dev" ;;
        test) echo "wallet-test" ;;
        prod) echo "wallet" ;;
        *)    err "Unknown ENV: $ENV"; exit 1 ;;
    esac
}

kustomize_dir() {
    echo "$K8S_DIR/overlays/$ENV"
}

usage() {
    cat <<EOF
Usage: $0 <command> [options]

Commands:
  build                       Build Docker image
  push                        Push image to registry
  deploy                      Deploy all resources
  deploy-infra                Deploy only infrastructure (postgres, redis, nats)
  deploy-apps                 Deploy only application services
  teardown                    Remove all resources
  status                      Show deployment status
  logs <service>              Tail logs for a service
  port-forward                Forward key ports to localhost
  scale <deployment> <n>      Scale a deployment
  diff                        Show what would change
  validate                    Validate kustomize output

Options:
  ENV=dev|test|prod           Target environment (default: dev)

Environments:
  dev    Single replica, debug logging, low resources, no TLS
  test   2 replicas for API services, info logging, medium resources
  prod   3+ replicas, warn logging, high resources, TLS, PDBs

Examples:
  $0 deploy                          # deploy to dev (default)
  ENV=test $0 deploy                 # deploy to test
  ENV=prod $0 deploy                 # deploy to prod
  ENV=prod $0 deploy-infra           # prod infrastructure only
  $0 logs wallet-api                 # tail dev logs
  ENV=prod $0 logs wallet-api        # tail prod logs
  $0 scale wallet-apigw-app 5        # scale in current env
  $0 diff                            # preview dev changes
  ENV=prod $0 diff                   # preview prod changes
EOF
}

# ── Build ────────────────────────────────────────────────────────────────

cmd_build() {
    log "Building image: $IMAGE"
    docker build -t "$IMAGE" "$SCRIPT_DIR/.."
    log "Build complete: $IMAGE"
}

cmd_push() {
    if [ -z "${REGISTRY:-}" ]; then
        err "REGISTRY env var not set. Example: REGISTRY=ghcr.io/myorg"
        exit 1
    fi
    local tagged="$REGISTRY/$IMAGE"
    docker tag "$IMAGE" "$tagged"
    docker push "$tagged"
    log "Pushed: $tagged"
}

# ── Deploy ───────────────────────────────────────────────────────────────

cmd_deploy() {
    local ns
    ns=$(ns_for_env)
    local dir
    dir=$(kustomize_dir)

    log "Deploying to ${CYAN}$ENV${NC} (namespace: $ns)"
    kubectl apply -k "$dir"
    log "Apply complete. Waiting for rollouts..."
    for deploy in wallet-api wallet-apigw-app wallet-ws; do
        kubectl -n "$ns" rollout status deployment/"$deploy" --timeout=180s 2>/dev/null || true
    done
    log "Deployment to $ENV complete."
}

cmd_deploy_infra() {
    local ns
    ns=$(ns_for_env)
    local dir
    dir=$(kustomize_dir)

    log "Deploying infrastructure to ${CYAN}$ENV${NC}"
    kubectl apply -k "$dir" -l 'app in (postgres,redis,nats)' 2>/dev/null || \
        kubectl apply -k "$dir"
    log "Waiting for infrastructure readiness..."
    kubectl -n "$ns" rollout status deployment/postgres --timeout=120s 2>/dev/null || true
    kubectl -n "$ns" rollout status deployment/redis --timeout=60s 2>/dev/null || true
    kubectl -n "$ns" rollout status deployment/nats --timeout=60s 2>/dev/null || true
    log "Infrastructure ready."
}

cmd_deploy_apps() {
    local ns
    ns=$(ns_for_env)
    local dir
    dir=$(kustomize_dir)

    log "Deploying applications to ${CYAN}$ENV${NC}"
    kubectl apply -k "$dir"
    log "Waiting for rollouts..."
    for deploy in wallet-api wallet-apigw-app wallet-apigw-admin wallet-ws; do
        kubectl -n "$ns" rollout status deployment/"$deploy" --timeout=180s 2>/dev/null || true
    done
    log "Applications ready."
}

cmd_teardown() {
    local ns
    ns=$(ns_for_env)
    warn "Removing ALL resources in namespace: $ns (env: $ENV)"
    read -rp "Are you sure? [y/N] " confirm
    if [[ ! "$confirm" =~ ^[yY]$ ]]; then
        log "Aborted."
        exit 0
    fi
    kubectl delete namespace "$ns" --ignore-not-found
    log "Teardown complete."
}

cmd_status() {
    local ns
    ns=$(ns_for_env)
    echo ""
    echo -e "${CYAN}=== Environment: $ENV | Namespace: $ns ===${NC}"
    echo ""
    echo "=== Pods ==="
    kubectl -n "$ns" get pods -o wide 2>/dev/null || echo "(no pods found)"
    echo ""
    echo "=== Services ==="
    kubectl -n "$ns" get svc 2>/dev/null || echo "(no services found)"
    echo ""
    echo "=== Ingress ==="
    kubectl -n "$ns" get ingress 2>/dev/null || echo "(none)"
    echo ""
    echo "=== Deployments ==="
    kubectl -n "$ns" get deployments 2>/dev/null || echo "(no deployments found)"
    echo ""
    echo "=== PDBs ==="
    kubectl -n "$ns" get pdb 2>/dev/null || echo "(none)"
}

cmd_logs() {
    local ns
    ns=$(ns_for_env)
    local svc="${1:-wallet-api}"
    kubectl -n "$ns" logs -f deployment/"$svc" --tail=100
}

cmd_port_forward() {
    local ns
    ns=$(ns_for_env)
    log "Port-forwarding (env=$ENV, ns=$ns):"
    echo "  App gateway:    http://localhost:8080"
    echo "  Admin gateway:  http://localhost:8081"
    echo "  Webhook:        http://localhost:8082"
    echo "  Chain gateway:  http://localhost:8545"
    echo "  WebSocket:      http://localhost:8090"
    echo "  gRPC (wallet):  localhost:9000"
    echo "  Swagger UI:     http://localhost:8080/swagger-ui"
    echo ""
    kubectl -n "$ns" port-forward svc/wallet-apigw-app 8080:8080 &
    kubectl -n "$ns" port-forward svc/wallet-apigw-admin 8081:8081 &
    kubectl -n "$ns" port-forward svc/wallet-apigw-webhook 8082:8082 &
    kubectl -n "$ns" port-forward svc/wallet-chain-gateway 8545:8545 &
    kubectl -n "$ns" port-forward svc/wallet-ws 8090:8090 &
    kubectl -n "$ns" port-forward svc/wallet-api 9000:9000 &
    log "Port-forwarding active. Press Ctrl+C to stop."
    wait
}

cmd_scale() {
    local ns
    ns=$(ns_for_env)
    local svc="${1:-}"
    local n="${2:-1}"
    if [ -z "$svc" ]; then
        err "Usage: $0 scale <deployment> <replicas>"
        exit 1
    fi
    kubectl -n "$ns" scale deployment/"$svc" --replicas="$n"
    log "Scaled $svc to $n replicas (env=$ENV)."
}

cmd_diff() {
    local dir
    dir=$(kustomize_dir)
    log "Diff for ${CYAN}$ENV${NC}:"
    kubectl diff -k "$dir" 2>/dev/null || true
}

cmd_validate() {
    local dir
    dir=$(kustomize_dir)
    log "Validating kustomize output for ${CYAN}$ENV${NC}:"
    kubectl kustomize "$dir" | head -20
    echo "..."
    kubectl kustomize "$dir" | wc -l | xargs -I{} echo "Total lines: {}"
    log "Validation complete."
}

# ── Main ─────────────────────────────────────────────────────────────────

case "${1:-}" in
    build)          cmd_build ;;
    push)           cmd_push ;;
    deploy)         cmd_deploy ;;
    deploy-infra)   cmd_deploy_infra ;;
    deploy-apps)    cmd_deploy_apps ;;
    teardown)       cmd_teardown ;;
    status)         cmd_status ;;
    logs)           cmd_logs "${2:-}" ;;
    port-forward)   cmd_port_forward ;;
    scale)          cmd_scale "${2:-}" "${3:-1}" ;;
    diff)           cmd_diff ;;
    validate)       cmd_validate ;;
    *)              usage ;;
esac
