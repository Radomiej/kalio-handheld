#!/usr/bin/env bash
# build-arm.sh — Cross-compile for aarch64-unknown-linux-gnu via Docker.
set -euo pipefail

echo "▶ Cross-compiling for ARM64…"
docker compose -f docker/docker-compose.yml run --rm cross-build
echo "✔ target/aarch64-unknown-linux-gnu/release/kalio-runtime"
