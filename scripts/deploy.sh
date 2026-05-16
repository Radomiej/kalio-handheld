#!/usr/bin/env bash
# deploy.sh — Push the ARM64 binary to the handheld device over WiFi (SSH/SCP).
#
# Set env vars before running:
#   KALIO_HOST  (e.g. 192.168.1.100)
#   KALIO_USER  (default: root)
#   KALIO_PATH  (default: /opt/kalio)
set -euo pipefail

HOST="${KALIO_HOST:-}"
if [[ -z "$HOST" ]]; then
    echo "Error: KALIO_HOST is not set. Example: export KALIO_HOST=192.168.1.100"
    exit 1
fi

USER="${KALIO_USER:-root}"
DEST="${KALIO_PATH:-/opt/kalio}"
BIN="target/aarch64-unknown-linux-gnu/release/kalio-runtime"

if [[ ! -f "$BIN" ]]; then
    echo "Binary not found. Run ./scripts/build-arm.sh first."
    exit 1
fi

echo "▶ Deploying to $USER@$HOST:$DEST…"
ssh "$USER@$HOST" "mkdir -p $DEST"
scp "$BIN"       "$USER@$HOST:$DEST/kalio-runtime"
scp -r assets    "$USER@$HOST:$DEST/"
ssh "$USER@$HOST" "chmod +x $DEST/kalio-runtime"
echo "✔ Deployed. Run on device: ssh $USER@$HOST $DEST/kalio-runtime"
