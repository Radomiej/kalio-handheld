#!/usr/bin/env bash
# qemu-run.sh — Boot an ARM64 Linux VM in QEMU and run the runtime inside it.
#
# Prerequisites:
#   apt install qemu-system-aarch64    (or brew install qemu on macOS)
#   Download a minimal ARM64 rootfs image, e.g. Alpine Linux or a dArkOS image.
#
# The script bind-mounts the repo via virtfs so the runtime binary is accessible.
set -euo pipefail

KERNEL="${QEMU_KERNEL:-docker/qemu/vmlinuz-arm64}"
INITRD="${QEMU_INITRD:-docker/qemu/initrd.img}"
DRIVE="${QEMU_DRIVE:-docker/qemu/rootfs.qcow2}"

# Check binary exists
BIN="target/aarch64-unknown-linux-gnu/release/kalio-runtime"
if [[ ! -f "$BIN" ]]; then
    echo "[qemu-run] Binary not found: $BIN"
    echo "           Run ./scripts/build-arm.sh first."
    exit 1
fi

if [[ ! -f "$KERNEL" ]]; then
    echo "[qemu-run] No kernel at $KERNEL"
    echo "           Place vmlinuz-arm64 and initrd.img under docker/qemu/"
    echo "           or set QEMU_KERNEL / QEMU_INITRD env vars."
    exit 1
fi

echo "▶ Starting QEMU ARM64…  (Ctrl-A X to exit)"
qemu-system-aarch64 \
    -machine virt \
    -cpu cortex-a72 \
    -smp 2 \
    -m 512M \
    -kernel "$KERNEL" \
    -initrd "$INITRD" \
    -nographic \
    -append "console=ttyAMA0 root=/dev/ram rw" \
    -device virtio-net-pci,netdev=net0 \
    -netdev  user,id=net0,hostfwd=tcp::8765-:8765 \
    -virtfs  local,path="$(pwd)",mount_tag=host0,security_model=passthrough,id=fsdev0
