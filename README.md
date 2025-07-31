# ORIUM Blockchain

A high-performance L1 blockchain built with [Substrate](https://substrate.io/), featuring native ORM token and dUSD/dEUR stablecoins with MakerDAO-style collateralization :rocket:

## Features

- **Native Token**: ORIUM (ORM) with "or" address prefix
- **Stablecoins**: dUSD (USD-pegged) and dEUR (EUR-pegged) with collateral backing
- **Consensus**: BABE + GRANDPA for fast finality and 2-second block time
- **Performance**: Optimized for 50,000+ TPS on 4-validator devnet
- **Governance**: Simple sudo for MVP (upgradeable to democracy)
- **Testing**: Comprehensive unit, property, and fuzzing test suite
- **DevOps**: CI/CD pipeline with Docker Compose 4-node devnet

## Quick Start

### Prerequisites

- Rust 1.75+ with `wasm32-unknown-unknown` target
- Docker and Docker Compose for devnet deployment
- Node.js (optional, for advanced benchmarking)

### Clone and Build

```sh
git clone https://github.com/bleakhash/orium_blockchain.git
cd orium_blockchain
cargo build --release
```

### Development Node

Start a single-node development chain:

```sh
./target/release/orium-node --dev
```

With detailed logging:

```sh
RUST_BACKTRACE=1 ./target/release/orium-node -ldebug --dev
```

Purge development chain state:

```sh
./target/release/orium-node purge-chain --dev
```

### Docker Devnet (4 Validators)

Start the 4-validator devnet with monitoring:

```sh
cd docker/devnet
./start-devnet.sh
```

This launches:
- 4 validator nodes (Alice, Bob, Charlie, Dave)
- Prometheus monitoring (http://localhost:9090)
- Grafana dashboards (http://localhost:3000, admin/admin)

Stop the devnet:

```sh
docker-compose down
```

### Performance Benchmarking

Run TPS benchmarks against the devnet:

```sh
cd docker/devnet
./benchmark-tps.sh
```

Target: ≥50,000 TPS with 100 concurrent users

## Token Operations

### ORIUM (ORM) Token

The native token with "or" address prefix:

```sh
# Check balance
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_account", "params": ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]}' http://localhost:9933

# Transfer tokens (via Polkadot-JS Apps)
# Navigate to: http://localhost:9944 -> Extrinsics -> oriumToken.transfer
```

### Stablecoins (dUSD/dEUR)

Create Collateralized Debt Position (CDP):

```sh
# 1. Deposit ORM collateral
# Extrinsics -> collateralEngine.createCdp(collateral_amount)

# 2. Mint dUSD stablecoin
# Extrinsics -> collateralEngine.mintDusd(amount)

# 3. Mint dEUR stablecoin  
# Extrinsics -> collateralEngine.mintDeur(amount)

# 4. Repay debt
# Extrinsics -> collateralEngine.repayDusd(amount)
# Extrinsics -> collateralEngine.repayDeur(amount)

# 5. Withdraw collateral
# Extrinsics -> collateralEngine.withdrawCollateral(amount)
```

**Collateral Requirements:**
- Minimum 150% collateralization ratio
- Liquidation threshold: 130%
- Stability fee: 2% annually

### Web Interface

Connect to your node using [Polkadot-JS Apps](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944):

1. **Development Node**: ws://localhost:9944
2. **Devnet Validators**:
   - Validator 1: ws://localhost:9944
   - Validator 2: ws://localhost:9945  
   - Validator 3: ws://localhost:9946
   - Validator 4: ws://localhost:9947

### Testing

Run the complete test suite:

```sh
# Unit and integration tests
cargo test --workspace

# Property-based fuzzing tests
cargo test --package orium-blockchain --test fuzzing_tests

# Lint and security checks
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo audit
```

## Architecture

The ORIUM blockchain consists of several key components:

### Node (`/node`)

The blockchain node implementation with:

- **Networking**: libp2p for peer-to-peer communication
- **Consensus**: BABE (block production) + GRANDPA (finality)
- **RPC Server**: JSON-RPC interface for external interactions

Key files:
- [`chain_spec.rs`](./node/src/chain_spec.rs): Genesis configuration with pre-funded accounts
- [`service.rs`](./node/src/service.rs): Node service implementation and consensus setup


### Runtime (`/runtime`)

The blockchain's state transition function built with [FRAME](https://docs.substrate.io/learn/runtime-development/#frame):

- **Block Time**: 2 seconds for fast finality
- **Address Format**: "or" prefix for ORIUM addresses
- **Governance**: Sudo-based (upgradeable to democracy)

See [`runtime/src/lib.rs`](./runtime/src/lib.rs) for pallet configuration.

### Custom Pallets (`/pallets`)

**ORIUM Token** (`pallet-orium-token`):
- Native ORM token with standard transfer functionality
- Integrated with collateral engine for CDP operations

**Stablecoins** (`pallet-dusd`, `pallet-deur`):
- USD and EUR-pegged stablecoins
- Minted through collateralized debt positions

**Collateral Engine** (`pallet-collateral-engine`):
- MakerDAO-style CDP system
- 150% minimum collateralization, 130% liquidation threshold
- Supports ORM collateral for dUSD/dEUR minting

Each pallet includes:
- **Storage**: Efficient key-value state management
- **Dispatchables**: External callable functions
- **Events**: State change notifications  
- **Errors**: Comprehensive error handling

## Documentation

- **[Security Audit](./docs/SECURITY_AUDIT.md)**: Comprehensive security analysis and vulnerability assessment
- **[Performance Report](./docs/PERFORMANCE_REPORT.md)**: TPS benchmarking results and optimization recommendations
- **[Installation Guide](./docs/INSTALLATION.md)**: Detailed setup instructions for all platforms
- **[Token Usage](./docs/TOKEN_USAGE.md)**: Complete guide to ORM, dUSD, and dEUR operations
- **[Architecture Overview](./docs/ARCHITECTURE.md)**: Technical deep-dive into system design

## Development

### CI/CD Pipeline

GitHub Actions automatically:
- Builds and tests all code changes
- Runs security audits and lint checks
- Builds Docker images for deployment
- Executes integration tests on devnet

### Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and add tests
4. Run the test suite: `cargo test --workspace`
5. Submit a pull request

## License

Licensed under the MIT License. See [LICENSE](./LICENSE) for details.

## Support

- **Issues**: [GitHub Issues](https://github.com/bleakhash/orium_blockchain/issues)
- **Discussions**: [GitHub Discussions](https://github.com/bleakhash/orium_blockchain/discussions)
- **Documentation**: [Substrate Documentation](https://docs.substrate.io/)
