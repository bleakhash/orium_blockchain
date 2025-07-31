//! Benchmarking setup for pallet-orium-token

use super::*;

#[allow(unused)]
use crate::Pallet as OriumToken;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn transfer() {
        let caller: T::AccountId = whitelisted_caller();
        let to: T::AccountId = account("to", 0, 0);
        let amount = 1000u32.into();

        Balances::<T>::insert(&caller, amount * 2u32.into());

        #[extrinsic_call]
        transfer(RawOrigin::Signed(caller.clone()), to.clone(), amount);

        assert_eq!(Balances::<T>::get(&to), amount);
    }

    #[benchmark]
    fn approve() {
        let caller: T::AccountId = whitelisted_caller();
        let spender: T::AccountId = account("spender", 0, 0);
        let amount = 1000u32.into();

        #[extrinsic_call]
        approve(RawOrigin::Signed(caller.clone()), spender.clone(), amount);

        assert_eq!(Allowances::<T>::get(&caller, &spender), amount);
    }

    impl_benchmark_test_suite!(OriumToken, crate::mock::new_test_ext(), crate::mock::Test);
}
