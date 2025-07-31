use crate as pallet_dusd;
use frame_support::derive_impl;
use sp_runtime::{traits::IdentityLookup, BuildStorage};

type Block = frame_system::mocking::MockBlock<Test>;

#[frame_support::runtime]
pub mod runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask
    )]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Test>;

    #[runtime::pallet_index(1)]
    pub type Dusd = pallet_dusd::Pallet<Test>;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
}

impl pallet_dusd::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type Balance = u128;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
