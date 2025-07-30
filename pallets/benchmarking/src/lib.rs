#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::ConstU32,
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::{
		traits::{Saturating, SaturatedConversion},
		BoundedVec,
	};

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn benchmark_counter)]
	pub type BenchmarkCounter<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn tps_measurements)]
	pub type TpsMeasurements<T> = StorageValue<_, BoundedVec<u32, ConstU32<100>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn last_block_transactions)]
	pub type LastBlockTransactions<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		BenchmarkExecuted { iterations: u32 },
		TpsMeasured { tps: u32, block_number: BlockNumberFor<T> },
		StressTestCompleted { total_transactions: u32, duration_blocks: u32 },
	}

	#[pallet::error]
	pub enum Error<T> {
		BenchmarkFailed,
		InvalidParameters,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::run_benchmark())]
		pub fn run_benchmark(
			origin: OriginFor<T>,
			iterations: u32,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			let mut counter = 0u32;
			for i in 0..iterations {
				counter = counter.saturating_add(i);
			}

			BenchmarkCounter::<T>::put(counter);
			Self::deposit_event(Event::BenchmarkExecuted { iterations });

			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::measure_tps())]
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

			LastBlockTransactions::<T>::put(transaction_count);
			Self::deposit_event(Event::TpsMeasured { tps, block_number: current_block });

			Ok(().into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::stress_test())]
		pub fn stress_test(
			origin: OriginFor<T>,
			transaction_count: u32,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			let start_block = frame_system::Pallet::<T>::block_number();
			
			for i in 0..transaction_count {
				let _result = i.saturating_mul(2).saturating_add(1);
			}

			let end_block = frame_system::Pallet::<T>::block_number();
			let duration = end_block.saturating_sub(start_block);

			Self::deposit_event(Event::StressTestCompleted {
				total_transactions: transaction_count,
				duration_blocks: duration.saturated_into(),
			});

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn get_average_tps() -> u32 {
			let measurements = TpsMeasurements::<T>::get();
			if measurements.is_empty() {
				return 0;
			}
			
			let sum: u32 = measurements.iter().sum();
			sum / measurements.len() as u32
		}

		pub fn get_latest_tps() -> u32 {
			let measurements = TpsMeasurements::<T>::get();
			measurements.last().copied().unwrap_or(0)
		}
	}

	pub trait WeightInfo {
		fn run_benchmark() -> Weight;
		fn measure_tps() -> Weight;
		fn stress_test() -> Weight;
	}

	impl WeightInfo for () {
		fn run_benchmark() -> Weight {
			Weight::from_parts(10_000, 0)
		}
		fn measure_tps() -> Weight {
			Weight::from_parts(5_000, 0)
		}
		fn stress_test() -> Weight {
			Weight::from_parts(50_000, 0)
		}
	}
}

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking {
	use super::*;
	use frame_benchmarking::{benchmarks, whitelisted_caller};
	use frame_system::RawOrigin;

	benchmarks! {
		run_benchmark {
			let caller: T::AccountId = whitelisted_caller();
		}: _(RawOrigin::Signed(caller), 1000u32)

		measure_tps {
			let caller: T::AccountId = whitelisted_caller();
		}: _(RawOrigin::Signed(caller))

		stress_test {
			let caller: T::AccountId = whitelisted_caller();
		}: _(RawOrigin::Signed(caller), 10000u32)

		impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
	}
}

#[cfg(test)]
mod mock {
	use super::*;
	use frame_support::{derive_impl, traits::ConstU32};
	use sp_runtime::traits::SaturatedConversion;
	use sp_runtime::BuildStorage;

	type Block = frame_system::mocking::MockBlock<Test>;

	frame_support::construct_runtime!(
		pub enum Test
		{
			System: frame_system,
			BenchmarkingPallet: crate,
		}
	);

	#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
	impl frame_system::Config for Test {
		type Block = Block;
	}

	impl Config for Test {
		type RuntimeEvent = RuntimeEvent;
		type WeightInfo = ();
	}

	pub fn new_test_ext() -> sp_io::TestExternalities {
		frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::*;
	use frame_support::{assert_ok, assert_err};

	#[test]
	fn benchmark_works() {
		new_test_ext().execute_with(|| {
			assert_ok!(BenchmarkingPallet::run_benchmark(
				RuntimeOrigin::signed(1),
				100
			));
			assert!(BenchmarkCounter::<Test>::get() > 0);
		});
	}

	#[test]
	fn tps_measurement_works() {
		new_test_ext().execute_with(|| {
			assert_ok!(BenchmarkingPallet::measure_tps(RuntimeOrigin::signed(1)));
			assert!(!TpsMeasurements::<Test>::get().is_empty());
		});
	}

	#[test]
	fn stress_test_works() {
		new_test_ext().execute_with(|| {
			assert_ok!(BenchmarkingPallet::stress_test(
				RuntimeOrigin::signed(1),
				1000
			));
		});
	}
}
