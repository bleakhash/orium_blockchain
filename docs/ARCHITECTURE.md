# ORIUM Blockchain Architecture

This document provides a comprehensive overview of the ORIUM blockchain architecture, including consensus mechanisms, runtime design, and system components.

## Overview

ORIUM is a high-performance Substrate-based blockchain designed for:

- **Native Token**: ORM with "or" address prefix
- **Stablecoins**: dUSD and dEUR with MakerDAO-style collateralization
- **High Throughput**: Target of 50,000+ TPS
- **Fast Finality**: 2-second block time with BABE + GRANDPA consensus

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    ORIUM Blockchain                        │
├─────────────────────────────────────────────────────────────┤
│  Applications & Interfaces                                 │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐          │
│  │   Web UI    │ │  Mobile App │ │   CLI Tool  │          │
│  └─────────────┘ └─────────────┘ └─────────────┘          │
├─────────────────────────────────────────────────────────────┤
│  API Layer                                                 │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐          │
│  │ JSON-RPC    │ │ WebSocket   │ │ GraphQL     │          │
│  └─────────────┘ └─────────────┘ └─────────────┘          │
├─────────────────────────────────────────────────────────────┤
│  Runtime (WASM)                                            │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐          │
│  │ ORM Token   │ │ Stablecoins │ │ Collateral  │          │
│  │   Pallet    │ │   Pallets   │ │   Engine    │          │
│  └─────────────┘ └─────────────┘ └─────────────┘          │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐          │
│  │   System    │ │  Balances   │ │ Benchmarks  │          │
│  │   Pallet    │ │   Pallet    │ │   Pallet    │          │
│  └─────────────┘ └─────────────┘ └─────────────┘          │
├─────────────────────────────────────────────────────────────┤
│  Consensus Layer                                           │
│  ┌─────────────┐ ┌─────────────┐                          │
│  │    BABE     │ │   GRANDPA   │                          │
│  │ (Block Prod)│ │ (Finality)  │                          │
│  └─────────────┘ └─────────────┘                          │
├─────────────────────────────────────────────────────────────┤
│  Network Layer                                             │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐          │
│  │   libp2p    │ │   Gossip    │ │   Discovery │          │
│  └─────────────┘ └─────────────┘ └─────────────┘          │
├─────────────────────────────────────────────────────────────┤
│  Storage Layer                                             │
│  ┌─────────────┐ ┌─────────────┐                          │
│  │   RocksDB   │ │   State     │                          │
│  │  (Backend)  │ │   Trie      │                          │
│  └─────────────┘ └─────────────┘                          │
└─────────────────────────────────────────────────────────────┘
```

## Consensus Mechanism

### BABE (Block Production)

**Blind Assignment for Blockchain Extension**

- **Slot Duration**: 2 seconds
- **Epoch Duration**: 600 blocks (~20 minutes)
- **VRF-based**: Verifiable Random Function for slot assignment
- **Probabilistic**: Multiple validators can produce blocks in same slot

```rust
// BABE Configuration
impl pallet_babe::Config for Runtime {
    type EpochDuration = EpochDuration;
    type ExpectedBlockTime = ExpectedBlockTime;
    type EpochChangeTrigger = pallet_babe::SameAuthoritiesForever;
    type DisabledValidators = ();
    type WeightInfo = ();
    type MaxAuthorities = MaxAuthorities;
    type KeyOwnerProof = sp_session::MembershipProof;
    type EquivocationReportSystem = ();
}
```

### GRANDPA (Finality)

**GHOST-based Recursive ANcestor Deriving Prefix Agreement**

- **Byzantine Fault Tolerant**: Handles up to 1/3 malicious validators
- **Finality Gadget**: Provides deterministic finality
- **Chain-based**: Finalizes chains, not individual blocks
- **Asynchronous**: Operates independently of block production

```rust
// GRANDPA Configuration
impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxAuthorities = MaxAuthorities;
    type MaxNominators = MaxNominators;
    type MaxSetIdSessionEntries = MaxSetIdSessionEntries;
    type KeyOwnerProof = sp_session::MembershipProof;
    type EquivocationReportSystem = ();
}
```

### Consensus Flow

```
1. BABE Slot Assignment
   ├── VRF determines slot winners
   ├── Multiple validators can win same slot
   └── Block production begins

2. Block Production
   ├── Collect transactions from pool
   ├── Execute transactions in runtime
   ├── Create block with state root
   └── Broadcast block to network

3. Block Import
   ├── Validate block structure
   ├── Execute block in runtime
   ├── Update local state
   └── Forward to GRANDPA

4. GRANDPA Finalization
   ├── Validators vote on chains
   ├── Supermajority determines finality
   ├── Finalized blocks are immutable
   └── Prune alternative chains
```

## Runtime Architecture

### FRAME (Framework for Runtime Aggregation of Modularized Entities)

ORIUM runtime is built using Substrate's FRAME framework:

```rust
// Runtime Construction
construct_runtime!(
    pub enum Runtime {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Babe: pallet_babe,
        Grandpa: pallet_grandpa,
        Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,
        Sudo: pallet_sudo,
        
        // ORIUM Custom Pallets
        OriumToken: pallet_orium_token,
        CollateralEngine: pallet_collateral_engine,
        Dusd: pallet_dusd,
        Deur: pallet_deur,
        Benchmarking: pallet_benchmarking,
    }
);
```

### Custom Pallets

#### 1. pallet-orium-token

**Purpose**: Native ORM token management

**Key Features**:
- Token minting and burning (sudo only)
- Balance transfers
- Account management
- Integration with fee payment system

**Storage Items**:
```rust
pub type TotalSupply<T> = StorageValue<_, T::Balance, ValueQuery>;
pub type Accounts<T> = StorageMap<_, Blake2_128Concat, T::AccountId, AccountData<T::Balance>, ValueQuery>;
```

**Extrinsics**:
- `mint(origin, to, amount)`: Mint new ORM tokens
- `burn(origin, from, amount)`: Burn ORM tokens
- `transfer(origin, to, amount)`: Transfer tokens between accounts

#### 2. pallet-collateral-engine

**Purpose**: MakerDAO-style collateralized debt positions

**Key Features**:
- Vault creation and management
- Collateral deposit/withdrawal
- Stablecoin minting/burning
- Liquidation mechanism
- Price oracle integration

**Storage Items**:
```rust
pub type Vaults<T> = StorageMap<_, Blake2_128Concat, VaultId, VaultInfo<T>, OptionQuery>;
pub type Prices<T> = StorageDoubleMap<_, Blake2_128Concat, AssetId, Blake2_128Concat, AssetId, Price, OptionQuery>;
pub type CollateralRatios<T> = StorageMap<_, Blake2_128Concat, AssetId, Ratio, ValueQuery>;
```

**Extrinsics**:
- `create_vault(origin, collateral_asset, collateral_amount)`
- `add_collateral(origin, vault_id, amount)`
- `withdraw_collateral(origin, vault_id, amount)`
- `mint_stablecoin(origin, vault_id, stablecoin_type, amount)`
- `repay_debt(origin, vault_id, amount)`
- `liquidate_vault(origin, vault_id)`

#### 3. pallet-dusd & pallet-deur

**Purpose**: USD and EUR pegged stablecoins

**Key Features**:
- Asset management extending Substrate's Assets pallet
- Integration with collateral engine
- Transfer and balance operations
- Peg maintenance mechanisms

**Storage Items**:
```rust
pub type Asset<T> = StorageValue<_, AssetDetails<T::Balance, T::AccountId, DepositBalanceOf<T>>, OptionQuery>;
pub type Account<T> = StorageDoubleMap<_, Blake2_128Concat, AssetId, Blake2_128Concat, T::AccountId, AssetAccount<T::Balance, DepositBalanceOf<T>, T::Extra, T::BlockNumber>, OptionQuery>;
```

#### 4. pallet-benchmarking

**Purpose**: Performance measurement and TPS benchmarking

**Key Features**:
- Transaction throughput measurement
- Stress testing capabilities
- Performance metrics collection
- Real-time TPS calculation

**Storage Items**:
```rust
pub type TpsMeasurements<T> = StorageValue<_, BoundedVec<u32, ConstU32<100>>, ValueQuery>;
pub type BenchmarkCounter<T> = StorageValue<_, u32, ValueQuery>;
```

## Performance Optimizations

### Block Production Optimizations

```rust
// Runtime Configuration for High TPS
parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    pub const Version: RuntimeVersion = VERSION;
    pub RuntimeBlockLength: BlockLength = BlockLength::max_with_normal_ratio(10 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
        .base_block(BlockExecutionWeight::get())
        .for_class(DispatchClass::all(), |weights| {
            weights.base_extrinsic = ExtrinsicBaseWeight::get();
        })
        .for_class(DispatchClass::Normal, |weights| {
            weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
        })
        .for_class(DispatchClass::Operational, |weights| {
            weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
            weights.reserved = Some(
                MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
            );
        })
        .avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
        .build_or_panic();
}
```

### Transaction Pool Configuration

```rust
// High-throughput transaction pool settings
pub const POOL_LIMIT: usize = 8192;
pub const POOL_KBYTES: usize = 32768;
pub const MAX_RUNTIME_INSTANCES: u32 = 32;
```

### BABE Slot Duration

```rust
parameter_types! {
    pub const ExpectedBlockTime: Moment = 2000; // 2 seconds
    pub const EpochDuration: u64 = EPOCH_DURATION_IN_BLOCKS as u64;
    pub const ReportLongevity: u64 = BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();
}
```

## Stablecoin Mechanism

### Collateralized Debt Position (CDP) System

```
┌─────────────────────────────────────────────────────────────┐
│                 CDP Lifecycle                               │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. Deposit Collateral (ORM)                               │
│     ├── User deposits ORM tokens                           │
│     ├── Vault created with unique ID                       │
│     └── Collateral locked in vault                         │
│                                                             │
│  2. Mint Stablecoins (dUSD/dEUR)                          │
│     ├── Check collateral ratio > 150%                      │
│     ├── Calculate max mintable amount                      │
│     ├── Mint stablecoins to user                          │
│     └── Record debt in vault                               │
│                                                             │
│  3. Maintain Collateral Ratio                              │
│     ├── Monitor ORM price changes                          │
│     ├── Calculate current ratio                            │
│     ├── Alert if ratio < 150%                             │
│     └── Liquidate if ratio < 130%                         │
│                                                             │
│  4. Repay Debt & Withdraw                                  │
│     ├── User repays stablecoin debt                       │
│     ├── Burn repaid stablecoins                           │
│     ├── Release proportional collateral                    │
│     └── Close vault when debt = 0                         │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Security Architecture

### Cryptographic Primitives

- **Hashing**: Blake2b-256 for state trie
- **Signatures**: SR25519 for account signatures
- **VRF**: SR25519-VRF for BABE slot assignment
- **Key Derivation**: BIP39/BIP44 for account generation

### Attack Vectors and Mitigations

#### 1. Consensus Attacks
- **51% Attack**: Mitigated by GRANDPA finality
- **Nothing at Stake**: Prevented by slashing conditions
- **Long Range Attack**: Checkpoints and weak subjectivity

#### 2. Economic Attacks
- **Oracle Manipulation**: Multiple price feeds, time delays
- **Flash Loan Attacks**: Minimum collateral lock periods
- **Governance Attacks**: Time delays, emergency procedures

#### 3. Network Attacks
- **Eclipse Attack**: Diverse peer connections
- **DDoS Attack**: Rate limiting, connection limits
- **Sybil Attack**: Proof of stake requirements

## Performance Benchmarking

### TPS Measurement

The benchmarking pallet provides real-time TPS measurement:

```rust
// TPS calculation in benchmarking pallet
pub fn measure_tps(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
    ensure_signed(origin)?;
    
    let current_block = frame_system::Pallet::<T>::block_number();
    let transaction_count = frame_system::Pallet::<T>::extrinsic_index().unwrap_or(0);
    
    let tps = (transaction_count * 30) / 60; // Approximate TPS calculation
    
    TpsMeasurements::<T>::mutate(|measurements| {
        if measurements.try_push(tps).is_err() {
            measurements.remove(0);
            let _ = measurements.try_push(tps);
        }
    });
    
    Ok(())
}
```

### Stress Testing

The system includes stress testing capabilities to validate performance under load:

```bash
# Run TPS benchmark
./scripts/benchmark-tps.sh 60 20 200
```

## Future Enhancements

### Planned Features

1. **Cross-Chain Bridges**: Integration with other blockchains
2. **Advanced Oracle System**: Decentralized price feeds
3. **Governance Module**: On-chain governance for parameters
4. **Staking Rewards**: Validator and nominator rewards
5. **Privacy Features**: Optional transaction privacy

### Scalability Improvements

1. **Parallel Transaction Processing**: Multi-threaded execution
2. **State Pruning**: Configurable state retention
3. **Light Client Optimization**: Faster sync and verification
4. **Database Optimization**: Custom storage backends

## Conclusion

ORIUM blockchain represents a modern, high-performance blockchain architecture built on Substrate's robust foundation. The combination of BABE + GRANDPA consensus, custom pallets for stablecoin functionality, and performance optimizations positions ORIUM to achieve its ambitious TPS targets while maintaining security and decentralization.

The modular architecture allows for future enhancements and upgrades without compromising the core functionality, ensuring ORIUM can evolve with the rapidly changing blockchain landscape.

## References

- [Substrate Documentation](https://docs.substrate.io/)
- [Polkadot Whitepaper](https://polkadot.network/whitepaper/)
- [BABE Paper](https://research.web3.foundation/Polkadot/protocols/block-production/Babe)
- [GRANDPA Paper](https://github.com/w3f/consensus/blob/master/pdf/grandpa.pdf)
- [MakerDAO Documentation](https://docs.makerdao.com/)
