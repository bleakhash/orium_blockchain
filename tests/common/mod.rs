
use frame_support::{
	derive_impl, parameter_types,
	traits::{ConstU32, ConstU64, ConstU128},
	weights::Weight,
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

pub use pallet_balances as Balances;
pub use pallet_collateral_engine as CollateralEngine;
pub use pallet_dusd as Dusd;
pub use pallet_deur as Deur;
pub use pallet_orium_token as OriumToken;
pub use frame_system as System;

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		OriumToken: pallet_orium_token,
		CollateralEngine: pallet_collateral_engine,
		Dusd: pallet_dusd,
		Deur: pallet_deur,
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u128;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<500>;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type DoneSlashHandler = ();
}

impl pallet_orium_token::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_orium_token::weights::SubstrateWeight<Test>;
	type Balance = u128;
}

impl pallet_collateral_engine::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_collateral_engine::weights::SubstrateWeight<Test>;
	type Balance = u128;
	type Currency = Balances;
	type MinCollateralRatio = ConstU32<15000>; // 150%
	type LiquidationRatio = ConstU32<13000>; // 130%
	type StabilityFee = ConstU32<500>; // 5%
}

impl pallet_dusd::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_dusd::weights::SubstrateWeight<Test>;
	type Balance = u128;
}

impl pallet_deur::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_deur::weights::SubstrateWeight<Test>;
	type Balance = u128;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
