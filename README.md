# can-viewer

A desktop application for viewing and analyzing CAN bus data from MDF4 files and live SocketCAN interfaces.

[![License](https://img.shields.io/crates/l/can-viewer.svg)](LICENSE)

## Features

- **MDF4 File Support** - Load and view CAN data from MDF4 files
- **DBC Decoding** - Decode CAN signals using DBC database files
- **Live Capture** - Capture CAN frames from SocketCAN interfaces (Linux)
- **Cross-platform UI** - Built with Tauri for native performance

## Quick Start

```bash
# Clone the repository
git clone https://github.com/reneherrero/can-viewer.git
cd can-viewer

# Install frontend dependencies
npm install

# Development mode (with devtools)
npm run tauri:dev

# Production build (creates .deb/.dmg/.msi)
npm run tauri:build
```

## Command Line Options

```
can-viewer [OPTIONS]

Options:
  -d, --dbc <PATH>    DBC file to load on startup
  -m, --mdf4 <PATH>   MDF4 file to load on startup
  -h, --help          Print help
```

## Environment Setup

### All Platforms

```bash
# Node.js 20+ (via nvm recommended)
nvm install 20
nvm use 20

# Rust 1.85+ (via rustup)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
```

### Linux (Debian/Ubuntu)

```bash
# Tauri dependencies
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# SocketCAN tools (for live capture)
sudo apt install -y can-utils

# Virtual CAN for testing (optional)
sudo modprobe vcan
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0
```

### macOS

```bash
# Xcode Command Line Tools
xcode-select --install
```

### Windows

- Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Select "Desktop development with C++" workload

Note: SocketCAN live capture is Linux-only. MDF4 viewing and DBC editing work on all platforms.

## Dependencies

- [mdf4-rs](https://github.com/reneherrero/mdf4-rs) - MDF4 file parsing
- [dbc-rs](https://github.com/reneherrero/dbc-rs) - DBC file parsing
- [Tauri](https://tauri.app) - Desktop application framework

## License

MIT OR Apache-2.0. See [LICENSING.md](LICENSING.md).
