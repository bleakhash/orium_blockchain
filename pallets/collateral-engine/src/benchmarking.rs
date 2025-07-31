//! Benchmarking setup for pallet-collateral-engine

use super::*;

#[allow(unused)]
use crate::Pallet as CollateralEngine;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_cdp() {
        let caller: T::AccountId = whitelisted_caller();
        let collateral_amount = 1000u32.into();
        #[extrinsic_call]
        create_cdp(RawOrigin::Signed(caller.clone()), collateral_amount);

        assert!(Cdps::<T>::contains_key(&caller));
    }

    #[benchmark]
    fn deposit_collateral() {
        let caller: T::AccountId = whitelisted_caller();
        let collateral_amount = 1000u32.into();
        let _ = CollateralEngine::<T>::create_cdp(
            RawOrigin::Signed(caller.clone()).into(),
            collateral_amount,
        );

        let additional_amount = 500u32.into();
        #[extrinsic_call]
        deposit_collateral(RawOrigin::Signed(caller.clone()), additional_amount);

        let cdp = Cdps::<T>::get(&caller).unwrap();
        assert!(cdp.collateral >= (collateral_amount + additional_amount));
    }

    impl_benchmark_test_suite!(
        CollateralEngine,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}
