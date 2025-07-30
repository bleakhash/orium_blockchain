# ORIUM Blockchain

A high-performance Substrate-based L1 blockchain with native ORM token and dUSD/dEUR stablecoins.

## 🚀 Features

- **Native Token**: ORIUM (ORM) with "or" address prefix
- **Stablecoins**: dUSD (USD-pegged) and dEUR (EUR-pegged) with MakerDAO-style collateralization
- **High Performance**: Optimized for 50,000+ TPS with 2-second block time
- **Consensus**: BABE + GRANDPA for fast finality and security
- **Comprehensive Testing**: Unit tests, property tests, and fuzzing for financial invariants
- **Docker Devnet**: 4-validator development network with monitoring

## 📋 Table of Contents

- [Quick Start](#quick-start)
- [Installation](#installation)
- [Running the Devnet](#running-the-devnet)
- [Performance Benchmarking](#performance-benchmarking)
- [Token Usage](#token-usage)
- [Development](#development)
- [Architecture](#architecture)
- [Contributing](#contributing)

## 🏃 Quick Start

### Prerequisites

- Docker and Docker Compose
- Rust (for building from source)

### Start the 4-Validator Devnet

```bash
# Clone the repository
git clone https://github.com/bleakhash/orium_blockchain.git
cd orium_blockchain

# Start the devnet
./scripts/start-devnet.sh
```

### Access Points

- **Validator 1**: http://localhost:9933 (RPC), ws://localhost:9944 (WebSocket)
- **Validator 2**: http://localhost:9934 (RPC), ws://localhost:9945 (WebSocket)
- **Validator 3**: http://localhost:9935 (RPC), ws://localhost:9946 (WebSocket)
- **Validator 4**: http://localhost:9936 (RPC), ws://localhost:9947 (WebSocket)
- **Prometheus**: http://localhost:9090
- **Grafana**: http://localhost:3000 (admin/admin)

## 🔧 Installation

### From Source

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Clone and build
git clone https://github.com/bleakhash/orium_blockchain.git
cd orium_blockchain
cargo build --release
```

### Using Docker

```bash
# Build Docker image
docker build -t orium-node .

# Run single node
docker run -p 9944:9944 -p 9933:9933 orium-node --dev --rpc-external --rpc-cors all
```

## 🌐 Running the Devnet

### Start Devnet

```bash
./scripts/start-devnet.sh
```

### Stop Devnet

```bash
docker-compose down
```

### View Logs

```bash
docker-compose logs -f
```

### Clean Restart

```bash
docker-compose down -v
docker-compose up -d
```

## 📊 Performance Benchmarking

### Run TPS Benchmark

```bash
# Default: 60 seconds, 10 concurrent users, 100 transactions per user
./scripts/benchmark-tps.sh

# Custom: 120 seconds, 20 concurrent users, 200 transactions per user
./scripts/benchmark-tps.sh 120 20 200
```

### Expected Performance

- **Target TPS**: 50,000+
- **Block Time**: 2 seconds
- **Finality**: ~6 seconds (3 blocks)

## 💰 Token Usage

### ORIUM (ORM) Native Token

```bash
# Check balance
curl -H "Content-Type: application/json" -d '{
  "id":1, "jsonrpc":"2.0", "method": "system_account",
  "params": ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
}' http://localhost:9933

# Transfer tokens
curl -H "Content-Type: application/json" -d '{
  "id":1, "jsonrpc":"2.0", "method": "author_submitExtrinsic",
  "params": ["0x..."] 
}' http://localhost:9933
```

### dUSD/dEUR Stablecoins

#### Create Collateral Vault

```bash
# Create vault with ORM collateral
curl -H "Content-Type: application/json" -d '{
  "id":1, "jsonrpc":"2.0", "method": "author_submitExtrinsic",
  "params": ["0x..."] 
}' http://localhost:9933
```

#### Mint Stablecoins

```bash
# Mint dUSD against collateral
curl -H "Content-Type: application/json" -d '{
  "id":1, "jsonrpc":"2.0", "method": "author_submitExtrinsic",
  "params": ["0x..."] 
}' http://localhost:9933
```

For detailed token usage examples, see [docs/TOKEN_USAGE.md](docs/TOKEN_USAGE.md).

## 🛠 Development

### Build and Test

```bash
# Build
cargo build --release

# Run tests
cargo test --all

# Run clippy
cargo clippy --all-targets --all-features

# Security audit
cargo audit
```

### Custom Pallets

- **pallet-orium-token**: Native ORM token management
- **pallet-collateral-engine**: MakerDAO-style CDP system
- **pallet-dusd**: USD-pegged stablecoin
- **pallet-deur**: EUR-pegged stablecoin
- **pallet-benchmarking**: Performance measurement tools

### Runtime Configuration

Key optimizations for high TPS:
- Block weight limit: 2,000,000,000,000
- Block length limit: 10MB
- Normal dispatch ratio: 90%
- BABE slot duration: 2000ms

## 🏗 Architecture

### Consensus

- **Block Production**: BABE (Blind Assignment for Blockchain Extension)
- **Finality**: GRANDPA (GHOST-based Recursive ANcestor Deriving Prefix Agreement)
- **Block Time**: 2 seconds
- **Epoch Duration**: 600 blocks (~20 minutes)

### Stablecoin Mechanism

1. **Collateral Deposit**: Users deposit ORM tokens as collateral
2. **Vault Creation**: System creates a Collateralized Debt Position (CDP)
3. **Stablecoin Minting**: Users can mint dUSD/dEUR up to collateral ratio limits
4. **Liquidation**: Under-collateralized positions are liquidated automatically
5. **Stability Fees**: Ongoing fees maintain the peg and system stability

### Performance Optimizations

- Optimized block weights and transaction batching
- Efficient storage patterns
- Minimal runtime overhead
- Parallel transaction processing via BABE consensus

## 🧪 Testing

### Test Suite

```bash
# Unit tests
cargo test --package pallet-orium-token
cargo test --package pallet-collateral-engine
cargo test --package pallet-dusd
cargo test --package pallet-deur

# Integration tests
cargo test --test integration_tests

# Property tests
cargo test --test property_tests

# Fuzzing tests
cargo test --test fuzzing_tests
```

### CI/CD Pipeline

- **Continuous Integration**: GitHub Actions
- **Security Scanning**: cargo audit
- **Code Quality**: clippy linting
- **Performance Regression**: Automated benchmarking

## 📚 Documentation

- [Installation Guide](docs/INSTALLATION.md)
- [Node Setup](docs/NODE_SETUP.md)
- [Token Usage](docs/TOKEN_USAGE.md)
- [API Reference](docs/API_REFERENCE.md)
- [Architecture](docs/ARCHITECTURE.md)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

### Development Guidelines

- Follow Substrate FRAME standards
- Avoid unsafe Rust code
- Ensure all tests pass
- Add comprehensive documentation
- Maintain backwards compatibility

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- **Repository**: https://github.com/bleakhash/orium_blockchain
- **Substrate Documentation**: https://docs.substrate.io/
- **Polkadot SDK**: https://github.com/paritytech/polkadot-sdk

## ⚠️ Disclaimer

This is experimental software. Use at your own risk. Not audited for production use.

---

**Built with ❤️ using Substrate and Polkadot SDK**
