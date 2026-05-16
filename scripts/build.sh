#!/usr/bin/env bash
# build.sh — Build kalio-runtime for the host machine (desktop-debug).
set -euo pipefail

echo "▶ Building kalio-runtime (desktop-debug)…"
cargo build -p kalio-runtime --features desktop-debug "$@"
echo "✔ target/debug/kalio-runtime"
