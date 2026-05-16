# Kalio Handheld Runtime

Host-first development stack for a handheld game console OS.

**Target device:** RK3326 ARM64 (dArkOS)  
**Dev machine:** PC — Linux / WSL / macOS / Windows via Docker

## Stack

| Layer         | Tech                         |
|---------------|------------------------------|
| Language      | Rust 1.88+                   |
| Renderer      | SDL2 2.30+                   |
| Layout        | Taffy (flexbox/grid)         |
| Scripts       | Lua 5.4                      |
| Async         | Tokio                        |
| Networking    | reqwest + tokio-tungstenite  |
| Serialization | serde + RON                  |
| Storage       | SQLite (bundled)             |
| Logging       | tracing                      |
| Dev server    | Go (WebSocket hot-reload)    |

## Quick Start

### Option 1: Docker (recommended)

```bash
docker compose -f docker/docker-compose.yml up runtime-dev
```

### Option 2: Native Linux

```bash
sudo apt install libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev libsdl2-mixer-dev cmake pkg-config
cargo run -p kalio-runtime --features desktop-debug
```

### Option 3: Windows (native)

1. Install [Rust](https://rustup.rs)
2. Install [SDL2 dev libs for MSVC/MinGW](https://github.com/libsdl-org/SDL/releases)
3. `cargo run -p kalio-runtime --features desktop-debug`

## Keyboard Emulation (`desktop-debug` mode)

| Key           | Gamepad action |
|---------------|----------------|
| W / ↑         | D-Pad Up       |
| S / ↓         | D-Pad Down     |
| A / ←         | D-Pad Left     |
| D / →         | D-Pad Right    |
| J / Enter     | A (Confirm)    |
| K / Escape    | B (Cancel)     |
| U             | X (Select)     |
| I             | Y (Menu)       |

## Cross-compile for ARM64

```bash
./scripts/build-arm.sh
```

## Deploy to device

```bash
export KALIO_HOST=192.168.1.100
./scripts/deploy.sh
```

## Dev server (hot-reload over WiFi)

```bash
cd tools/dev-server
go mod tidy
go run . ../apps 8765
```

## QEMU ARM64 testing

```bash
./scripts/qemu-run.sh
```

## Architecture

```
App State
    ↓
UI Tree (RON DSL)
    ↓
Taffy Layout
    ↓
Render Commands (command buffer)
    ↓
SDL2 Renderer
```

## Project Structure

```
kalio-handheld/
├── runtime/           # Core runtime (Rust binary)
│   └── src/
│       ├── app/       # Main loop + app lifecycle
│       ├── renderer/  # Command-buffer SDL2 renderer
│       ├── input/     # GameController + keyboard emulation
│       ├── layout/    # Taffy flex/grid layout engine
│       ├── scripting/ # Lua 5.4 script engine
│       ├── network/   # WebSocket dev client
│       └── storage/   # SQLite KV store
├── launcher/          # Launcher app (uses SDK)
├── sdk/               # API for app developers
├── apps/
│   └── hello/         # Demo app
├── shared/            # Shared types (RenderCommand, InputAction, …)
├── tools/
│   └── dev-server/    # Go WebSocket hot-reload server
├── docker/            # Dockerfiles + compose
├── scripts/           # build.sh / build-arm.sh / deploy.sh / qemu-run.sh
├── assets/            # Fonts, images, audio
└── config/            # RON config files
```
