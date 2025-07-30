//!
//!

use frame_support::{
    parameter_types,
    weights::{constants::WEIGHT_REF_TIME_PER_SECOND, Weight},
};
use sp_runtime::Perbill;

parameter_types! {
    pub const MaximumBlockWeight: Weight = Weight::from_parts(
        4u64 * WEIGHT_REF_TIME_PER_SECOND, // 4 seconds of compute time
        u64::MAX, // No proof size limit for high throughput
    );

    pub const MaximumBlockLength: u32 = 10 * 1024 * 1024; // 10MB

    pub const TransactionPoolMaxSize: u32 = 100_000; // 100k transactions in pool
    pub const TransactionPoolMaxPerSender: u32 = 1_000; // 1k per sender to prevent spam

    pub const BlockProductionRatio: Perbill = Perbill::from_percent(75);

    pub const MaxStorageKeyLength: u32 = 128; // Reasonable key length limit
    pub const MaxStorageValueLength: u32 = 16 * 1024 * 1024; // 16MB for large data structures

    pub const MaxBatchSize: u32 = 10_000; // Large batch size for high throughput
    pub const MaxCallsPerBatch: u32 = 1_000; // Reasonable call limit per batch
}

pub mod weights {
    use frame_support::weights::Weight;

    pub const TRANSFER_WEIGHT: Weight = Weight::from_parts(50_000, 0);

    pub const STABLECOIN_MINT_WEIGHT: Weight = Weight::from_parts(100_000, 0);
    pub const STABLECOIN_BURN_WEIGHT: Weight = Weight::from_parts(80_000, 0);

    pub const COLLATERAL_DEPOSIT_WEIGHT: Weight = Weight::from_parts(150_000, 0);
    pub const COLLATERAL_WITHDRAW_WEIGHT: Weight = Weight::from_parts(200_000, 0);
    pub const LIQUIDATION_WEIGHT: Weight = Weight::from_parts(300_000, 0);
}

pub trait HighTpsStorage {
    fn batch_read<K, V>(keys: &[K]) -> Vec<Option<V>>;
    fn batch_write<K, V>(items: &[(K, V)]);
    fn batch_remove<K>(keys: &[K]);
}
