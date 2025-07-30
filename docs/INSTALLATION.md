# Installation Guide

This guide covers different methods to install and run the ORIUM blockchain.

## Prerequisites

### System Requirements

- **OS**: Linux (Ubuntu 20.04+ recommended), macOS, or Windows (WSL2)
- **RAM**: 8GB minimum, 16GB recommended
- **Storage**: 50GB free space
- **CPU**: 4+ cores recommended

### Required Software

#### Rust Toolchain

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version

# Add WebAssembly target (required for Substrate)
rustup target add wasm32-unknown-unknown
```

#### Additional Dependencies

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y \
    build-essential \
    git \
    clang \
    curl \
    libssl-dev \
    llvm \
    libudev-dev \
    protobuf-compiler \
    pkg-config
```

**macOS:**
```bash
# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install openssl cmake llvm protobuf
```

**Windows (WSL2):**
```bash
# Enable WSL2 and install Ubuntu
wsl --install -d Ubuntu

# Follow Ubuntu instructions above
```

## Installation Methods

### Method 1: Build from Source (Recommended)

#### 1. Clone Repository

```bash
git clone https://github.com/bleakhash/orium_blockchain.git
cd orium_blockchain
```

#### 2. Build the Project

```bash
# Build in release mode (optimized)
cargo build --release

# This will take 15-30 minutes on first build
```

#### 3. Verify Installation

```bash
# Check if binary was created
ls -la target/release/solochain-template-node

# Test the node
./target/release/solochain-template-node --version
```

### Method 2: Docker Installation

#### 1. Install Docker

**Ubuntu:**
```bash
# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Add user to docker group
sudo usermod -aG docker $USER
newgrp docker

# Install Docker Compose
sudo apt install docker-compose-plugin
```

**macOS:**
```bash
# Install Docker Desktop
brew install --cask docker
```

#### 2. Build Docker Image

```bash
git clone https://github.com/bleakhash/orium_blockchain.git
cd orium_blockchain

# Build the Docker image
docker build -t orium-node .
```

#### 3. Run Single Node

```bash
# Run development node
docker run -p 9944:9944 -p 9933:9933 \
    orium-node --dev --rpc-external --rpc-cors all
```

### Method 3: Pre-built Binaries (Coming Soon)

Pre-built binaries will be available for download from GitHub Releases.

## Configuration

### Environment Variables

Create a `.env` file in the project root:

```bash
# Node configuration
RUST_LOG=info
ORIUM_BASE_PATH=/tmp/orium-data
ORIUM_CHAIN=local

# RPC configuration
ORIUM_RPC_PORT=9933
ORIUM_WS_PORT=9944
ORIUM_P2P_PORT=30333

# Performance tuning
ORIUM_MAX_PEERS=50
ORIUM_POOL_LIMIT=8192
```

### Chain Specifications

#### Development Chain

```bash
# Generate development chain spec
./target/release/solochain-template-node build-spec \
    --dev --disable-default-bootnode > dev-spec.json

# Convert to raw format
./target/release/solochain-template-node build-spec \
    --chain dev-spec.json --raw > dev-spec-raw.json
```

#### Local Testnet

```bash
# Generate local testnet spec
./target/release/solochain-template-node build-spec \
    --chain local --disable-default-bootnode > local-spec.json

# Convert to raw format
./target/release/solochain-template-node build-spec \
    --chain local-spec.json --raw > local-spec-raw.json
```

## Running the Node

### Development Mode

```bash
# Run single development node
./target/release/solochain-template-node --dev

# With custom base path
./target/release/solochain-template-node --dev \
    --base-path /tmp/orium-dev

# With RPC access
./target/release/solochain-template-node --dev \
    --rpc-external --rpc-cors all
```

### Validator Mode

```bash
# Run as validator
./target/release/solochain-template-node \
    --chain local \
    --validator \
    --base-path /tmp/orium-validator \
    --port 30333 \
    --rpc-port 9933 \
    --name "ORIUM-Validator-1"
```

### Archive Node

```bash
# Run archive node (keeps all historical data)
./target/release/solochain-template-node \
    --chain local \
    --pruning archive \
    --base-path /tmp/orium-archive \
    --rpc-external \
    --rpc-cors all
```

## Verification

### Check Node Status

```bash
# Check if node is running
curl -H "Content-Type: application/json" -d '{
    "id":1, 
    "jsonrpc":"2.0", 
    "method":"system_health"
}' http://localhost:9933
```

Expected response:
```json
{
    "jsonrpc": "2.0",
    "result": {
        "peers": 0,
        "isSyncing": false,
        "shouldHavePeers": false
    },
    "id": 1
}
```

### Check Chain Info

```bash
curl -H "Content-Type: application/json" -d '{
    "id":1, 
    "jsonrpc":"2.0", 
    "method":"system_chain"
}' http://localhost:9933
```

### Check Node Version

```bash
curl -H "Content-Type: application/json" -d '{
    "id":1, 
    "jsonrpc":"2.0", 
    "method":"system_version"
}' http://localhost:9933
```

## Troubleshooting

### Common Issues

#### Build Errors

**Error: `wasm32-unknown-unknown` target not found**
```bash
rustup target add wasm32-unknown-unknown
```

**Error: `protoc` not found**
```bash
# Ubuntu/Debian
sudo apt install protobuf-compiler

# macOS
brew install protobuf
```

**Error: Linking errors on Ubuntu**
```bash
sudo apt install build-essential clang libssl-dev
```

#### Runtime Issues

**Error: Database corruption**
```bash
# Remove corrupted database
rm -rf /tmp/orium-data/chains/*/db/

# Restart node
./target/release/solochain-template-node --dev
```

**Error: Port already in use**
```bash
# Check what's using the port
sudo lsof -i :9944

# Kill the process or use different ports
./target/release/solochain-template-node --dev \
    --port 30334 --rpc-port 9934 --ws-port 9945
```

### Performance Tuning

#### System Limits

```bash
# Increase file descriptor limits
echo "* soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "* hard nofile 65536" | sudo tee -a /etc/security/limits.conf

# Increase memory map areas
echo "vm.max_map_count=262144" | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

#### Node Configuration

```bash
# High-performance configuration
./target/release/solochain-template-node \
    --dev \
    --pool-limit 8192 \
    --pool-kbytes 32768 \
    --max-runtime-instances 32 \
    --runtime-cache-size 64
```

### Logs and Debugging

#### Enable Debug Logs

```bash
# Set log level
export RUST_LOG=debug

# Or specific modules
export RUST_LOG=sc_consensus_babe=debug,sc_consensus_grandpa=debug

# Run node
./target/release/solochain-template-node --dev
```

#### Log to File

```bash
# Redirect logs to file
./target/release/solochain-template-node --dev 2>&1 | tee orium.log

# Or use systemd journal
journalctl -u orium-node -f
```

## Next Steps

After successful installation:

1. [Set up a development network](NODE_SETUP.md)
2. [Learn about token usage](TOKEN_USAGE.md)
3. [Explore the API](API_REFERENCE.md)
4. [Understand the architecture](ARCHITECTURE.md)

## Support

If you encounter issues:

1. Check the [troubleshooting section](#troubleshooting)
2. Search existing [GitHub issues](https://github.com/bleakhash/orium_blockchain/issues)
3. Create a new issue with detailed error logs
4. Join our community discussions
