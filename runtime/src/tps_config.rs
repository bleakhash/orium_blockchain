//! 

use frame_support::{
	parameter_types,
	weights::{Weight, constants::WEIGHT_REF_TIME_PER_SECOND},
};
use sp_runtime::Perbill;

parameter_types! {
	pub const MaximumBlockWeight: Weight = Weight::from_parts(
		4u64 * WEIGHT_REF_TIME_PER_SECOND,
		u64::MAX,
	);
	
	pub const MaximumBlockLength: u32 = 10 * 1024 * 1024;
	
	pub const TransactionPoolMaxSize: u32 = 100_000;
	pub const TransactionPoolMaxPerSender: u32 = 1_000;
	
	pub const BlockProductionRatio: Perbill = Perbill::from_percent(75);
	
	pub const MaxStorageKeyLength: u32 = 128;
	pub const MaxStorageValueLength: u32 = 1024 * 1024; // 1MB
	
	pub const MaxBatchSize: u32 = 10_000;
	pub const MaxCallsPerBatch: u32 = 1_000;
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
