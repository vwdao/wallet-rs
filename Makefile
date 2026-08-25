# ── Wallet Makefile ──────────────────────────────────────────────────────
# Usage: make <target> [ENV=dev|test|prod]

REGISTRY ?= ghcr.io/yourorg
IMAGE    ?= wallet
TAG      ?= latest
FULL_TAG = $(REGISTRY)/$(IMAGE):$(TAG)
ENV      ?= dev
DEPLOY   = ./deploy/deploy.sh

.PHONY: help build build-image push push-image image deploy deploy-infra \
        deploy-apps teardown status logs port-forward scale proto check test \
        release diff validate

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

# ── Build ────────────────────────────────────────────────────────────────

build: ## Build Docker image
	docker build -t $(IMAGE):$(TAG) .

release: ## Cross-compile all binaries (no Docker)
	cargo build --release --bins

check: ## Run cargo check
	cargo check

test: ## Run tests
	cargo test

proto: ## Regenerate protobuf code
	protoc --proto_path=proto --tonic_out=crates/wallet-proto/src/proto \
		--include_imports proto/wallet/v1/*.proto

# ── Docker ───────────────────────────────────────────────────────────────

push: build ## Build and push image
	docker tag $(IMAGE):$(TAG) $(FULL_TAG)
	docker push $(FULL_TAG)

# Image build/push via buildx (mirrors wallet-core-go). Override:
#   make image IMAGE_REPOSITORY=hub.awalletdev.com/wt-dev/wallet-rs IMAGE_TAG=short-sha
#   BUILD_PLATFORM=linux/amd64 (default)
IMAGE_REPOSITORY ?= hub.awalletdev.com/wt-dev/wallet-rs
IMAGE_TAG        ?= $(shell git rev-parse --short HEAD)

build-image: ## Build Docker image (override IMAGE_REPOSITORY / IMAGE_TAG / BUILD_PLATFORM)
	IMAGE_REPOSITORY=$(IMAGE_REPOSITORY) \
	IMAGE_TAG=$(IMAGE_TAG) \
	BUILD_PLATFORM=$(BUILD_PLATFORM) \
	./scripts/build_wallet_image.sh

push-image: ## Push Docker image to registry
	IMAGE_REPOSITORY=$(IMAGE_REPOSITORY) \
	IMAGE_TAG=$(IMAGE_TAG) \
	./scripts/push_wallet_image.sh

image: build-image push-image ## Build and push Docker image

# ── Deploy (per-environment) ─────────────────────────────────────────────

deploy: build ## Build image and deploy to ENV (default: dev)
	ENV=$(ENV) $(DEPLOY) deploy

deploy-infra: ## Deploy infrastructure only
	ENV=$(ENV) $(DEPLOY) deploy-infra

deploy-apps: ## Deploy application services only
	ENV=$(ENV) $(DEPLOY) deploy-apps

teardown: ## Remove all k8s resources for ENV
	ENV=$(ENV) $(DEPLOY) teardown

status: ## Show k8s status for ENV
	ENV=$(ENV) $(DEPLOY) status

logs: ## Tail logs (make logs SVC=wallet-api)
	ENV=$(ENV) $(DEPLOY) logs $(SVC)

port-forward: ## Forward ports for ENV
	ENV=$(ENV) $(DEPLOY) port-forward

scale: ## Scale (make scale SVC=wallet-apigw-app N=3)
	ENV=$(ENV) $(DEPLOY) scale $(SVC) $(N)

diff: ## Preview changes for ENV
	ENV=$(ENV) $(DEPLOY) diff

validate: ## Validate kustomize output for ENV
	ENV=$(ENV) $(DEPLOY) validate

# ── Environment shortcuts ────────────────────────────────────────────────

dev:     ## Deploy to dev
	$(MAKE) deploy ENV=dev

test:    ## Deploy to test
	$(MAKE) deploy ENV=test

prod:    ## Deploy to prod
	$(MAKE) deploy ENV=prod

dev-infra:   ## Deploy dev infrastructure
	$(MAKE) deploy-infra ENV=dev

test-infra:  ## Deploy test infrastructure
	$(MAKE) deploy-infra ENV=test

prod-infra:  ## Deploy prod infrastructure
	$(MAKE) deploy-infra ENV=prod

# ── Quick commands (current ENV) ─────────────────────────────────────────

logs-api:       ## Tail wallet-api logs
	ENV=$(ENV) $(DEPLOY) logs wallet-api

logs-app-gw:    ## Tail app gateway logs
	ENV=$(ENV) $(DEPLOY) logs wallet-apigw-app

logs-sync:      ## Tail ETH sync logs
	ENV=$(ENV) $(DEPLOY) logs wallet-sync-eth

logs-jobs:      ## Tail jobs worker logs
	ENV=$(ENV) $(DEPLOY) logs wallet-jobs

restart: ## Rolling restart all deployments
	kubectl -n $(shell ENV=$(ENV) $(DEPLOY) status 2>/dev/null | grep -o 'wallet[a-z-]*' | head -1 | xargs echo) rollout restart deployment 2>/dev/null || \
	kubectl -n wallet-dev rollout restart deployment

ps: ## Alias for status
	$(MAKE) status
