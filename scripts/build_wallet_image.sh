#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"

IMAGE_REPOSITORY="${IMAGE_REPOSITORY:-hub.awalletdev.com/wt-dev/wallet-rs}"
IMAGE_TAG="${IMAGE_TAG:-$(git -C "${REPO_ROOT}" rev-parse --short HEAD)}"
IMAGE_NAME="${IMAGE_REPOSITORY}:${IMAGE_TAG}"

# Default linux/amd64. Override: BUILD_PLATFORM=linux/arm64 ...
BUILD_PLATFORM="${BUILD_PLATFORM:-linux/amd64}"

export DOCKER_BUILDKIT=1
docker buildx build \
  --platform "${BUILD_PLATFORM}" \
  --file "${REPO_ROOT}/Dockerfile" \
  --tag "${IMAGE_NAME}" \
  --load \
  "${REPO_ROOT}"

printf 'built image %s (platform %s)\n' "${IMAGE_NAME}" "${BUILD_PLATFORM}"
printf 'ops handoff:\n'
printf '  image: %s\n' "${IMAGE_NAME}"
printf '  helm image.repository: %s\n' "${IMAGE_REPOSITORY}"
printf '  helm image.tag: %s\n' "${IMAGE_TAG}"
