# ORIUM Blockchain Installation Guide

This guide provides detailed instructions for installing and setting up the ORIUM blockchain node on various operating systems.

## System Requirements

### Minimum Requirements
- **CPU**: 2 cores, 2.0 GHz
- **RAM**: 4 GB
- **Storage**: 20 GB available space
- **Network**: Broadband internet connection

### Recommended Requirements
- **CPU**: 4+ cores, 3.0 GHz
- **RAM**: 16 GB
- **Storage**: 100 GB SSD
- **Network**: High-speed internet with low latency

### Supported Operating Systems
- Ubuntu 20.04 LTS or later
- Debian 11 or later
- CentOS 8 or later
- macOS 11.0 or later
- Windows 10/11 (via WSL2)

## Prerequisites

### 1. Install Rust

ORIUM blockchain is built with Rust. Install the latest stable version:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

Verify installation:
```bash
rustc --version
cargo --version
```

Add WebAssembly target:
```bash
rustup target add wasm32-unknown-unknown
```

### 2. Install System Dependencies

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install -y \
    clang \
    libclang-dev \
    cmake \
    build-essential \
    git \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    curl \
    wget
```

#### CentOS/RHEL/Fedora
```bash
sudo dnf update
sudo dnf install -y \
    clang \
    clang-devel \
    cmake \
    gcc \
    gcc-c++ \
    git \
    pkgconfig \
    openssl-devel \
    protobuf-compiler \
    curl \
    wget
```

#### macOS
Install Homebrew if not already installed:
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Install dependencies:
```bash
brew install cmake pkg-config openssl protobuf git
```

#### Windows (WSL2)
1. Install WSL2 following [Microsoft's guide](https://docs.microsoft.com/en-us/windows/wsl/install)
2. Install Ubuntu 20.04 LTS from Microsoft Store
3. Follow Ubuntu installation steps above

## Installation Methods

### Method 1: Build from Source (Recommended)

#### 1. Clone Repository
```bash
git clone https://github.com/your-org/orium-blockchain.git
cd orium-blockchain
```

#### 2. Build Release Binary
```bash
cargo build --release
```

This process may take 30-60 minutes depending on your system.

#### 3. Verify Installation
```bash
./target/release/orium-node --version
```

#### 4. Install Binary (Optional)
```bash
sudo cp target/release/orium-node /usr/local/bin/
```

### Method 2: Pre-built Binaries

Download the latest release from GitHub:

```bash
# Download latest release
wget https://github.com/your-org/orium-blockchain/releases/latest/download/orium-blockchain-release.tar.gz

# Extract
tar -xzf orium-blockchain-release.tar.gz
cd release

# Make executable
chmod +x orium-node

# Install (optional)
sudo cp orium-node /usr/local/bin/
```

### Method 3: Docker Installation

#### Install Docker
Follow the official Docker installation guide for your OS:
- [Ubuntu](https://docs.docker.com/engine/install/ubuntu/)
- [macOS](https://docs.docker.com/desktop/mac/install/)
- [Windows](https://docs.docker.com/desktop/windows/install/)

#### Pull ORIUM Image
```bash
docker pull orium/blockchain-node:latest
```

#### Run Container
```bash
docker run -d \
  --name orium-node \
  -p 9933:9933 \
  -p 9944:9944 \
  -p 30333:30333 \
  -v orium-data:/data \
  orium/blockchain-node:latest \
  --dev --tmp
```

## Configuration

### 1. Create Data Directory
```bash
mkdir -p ~/.local/share/orium-node
```

### 2. Generate Node Key (Optional)
```bash
orium-node key generate-node-key --file ~/.local/share/orium-node/node-key
```

### 3. Create Chain Specification (Advanced)
```bash
orium-node build-spec --chain dev > custom-spec.json
orium-node build-spec --chain custom-spec.json --raw > custom-spec-raw.json
```

## First Run

### Development Mode
```bash
orium-node --dev --tmp
```

### Persistent Development Mode
```bash
orium-node --dev --base-path ~/.local/share/orium-node
```

### Custom Configuration
```bash
orium-node \
  --chain dev \
  --base-path ~/.local/share/orium-node \
  --name "MyOriumNode" \
  --port 30333 \
  --rpc-port 9933 \
  --ws-port 9944 \
  --rpc-cors all \
  --unsafe-rpc-external \
  --unsafe-ws-external
```

## Verification

### 1. Check Node Status
```bash
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params":[]}' \
     http://localhost:9933
```

Expected response:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "isSyncing": false,
    "peers": 0,
    "shouldHavePeers": false
  },
  "id": 1
}
```

### 2. Check Node Info
```bash
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_name", "params":[]}' \
     http://localhost:9933
```

### 3. WebSocket Connection
Test WebSocket connection at `ws://localhost:9944`

## Troubleshooting

### Common Issues

#### 1. Compilation Errors
```bash
# Update Rust
rustup update

# Clean build cache
cargo clean

# Rebuild
cargo build --release
```

#### 2. Missing Dependencies
```bash
# Ubuntu/Debian
sudo apt install --fix-missing

# Reinstall specific packages
sudo apt install --reinstall build-essential
```

#### 3. Port Already in Use
```bash
# Check what's using the port
sudo netstat -tulpn | grep :9933

# Kill process if needed
sudo kill -9 <PID>

# Or use different ports
orium-node --dev --rpc-port 9934 --ws-port 9945
```

#### 4. Permission Denied
```bash
# Fix data directory permissions
sudo chown -R $USER:$USER ~/.local/share/orium-node

# Make binary executable
chmod +x target/release/orium-node
```

#### 5. WebAssembly Target Missing
```bash
rustup target add wasm32-unknown-unknown
```

### Performance Issues

#### 1. Slow Compilation
```bash
# Use more CPU cores for compilation
export CARGO_BUILD_JOBS=4
cargo build --release

# Or use cargo-chef for Docker builds
```

#### 2. High Memory Usage
```bash
# Limit compilation memory
export CARGO_BUILD_RUSTFLAGS="-C link-arg=-Wl,--compress-debug-sections=zlib"

# Use swap if needed
sudo fallocate -l 4G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

### Getting Help

1. **Check Logs**
```bash
# Enable debug logging
RUST_LOG=debug orium-node --dev

# Or specific modules
RUST_LOG=runtime=debug,babe=trace orium-node --dev
```

2. **Community Support**
- GitHub Issues: [Report bugs and ask questions](https://github.com/your-org/orium-blockchain/issues)
- Discord: [Join our community](https://discord.gg/orium)
- Documentation: [Read the docs](https://docs.orium.network)

3. **Professional Support**
Contact the ORIUM team for enterprise support and consulting services.

## Next Steps

After successful installation:

1. **Read the Node Setup Guide**: [NODE_SETUP.md](NODE_SETUP.md)
2. **Learn Token Operations**: [TOKEN_USAGE.md](TOKEN_USAGE.md)
3. **Explore Stablecoins**: [STABLECOIN_GUIDE.md](STABLECOIN_GUIDE.md)
4. **Join the Network**: [VALIDATOR_GUIDE.md](VALIDATOR_GUIDE.md)
5. **Development**: [DEVELOPMENT.md](DEVELOPMENT.md)

## Security Considerations

- Keep your node software updated
- Secure your private keys and node keys
- Use firewall rules to restrict access
- Monitor your node's performance and security
- Regular backups of important data

## License

This installation guide is part of the ORIUM blockchain project, licensed under the MIT License.
