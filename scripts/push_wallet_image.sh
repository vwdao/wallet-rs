#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"

IMAGE_REPOSITORY="${IMAGE_REPOSITORY:-hub.awalletdev.com/wt-dev/wallet-rs}"
IMAGE_TAG="${IMAGE_TAG:-$(git -C "${REPO_ROOT}" rev-parse --short HEAD)}"
IMAGE_NAME="${IMAGE_REPOSITORY}:${IMAGE_TAG}"

docker push "${IMAGE_NAME}"

printf 'pushed image %s\n' "${IMAGE_NAME}"
printf 'ops handoff:\n'
printf '  image: %s\n' "${IMAGE_NAME}"
printf '  helm image.repository: %s\n' "${IMAGE_REPOSITORY}"
printf '  helm image.tag: %s\n' "${IMAGE_TAG}"
