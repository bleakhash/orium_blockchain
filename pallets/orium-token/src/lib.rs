//!
//!
//! ## Overview
//!
//!
//! Each pallet section is annotated with an attribute using the `#[pallet::...]` procedural macro.
//! This macro generates the necessary code for a pallet to be aggregated into a FRAME runtime.
//!
//! Learn more about FRAME macros [here](https://docs.substrate.io/reference/frame-macros/).
//!
//! ### Pallet Sections
//!
//! The pallet sections in this template are:
//!
//! - A **configuration trait** that defines the types and parameters which the pallet depends on
//!   (denoted by the `#[pallet::config]` attribute). See: [`Config`].
//! - A **means to store pallet-specific data** (denoted by the `#[pallet::storage]` attribute).
//!   See: [`storage_types`].
//! - A **declaration of the events** this pallet emits (denoted by the `#[pallet::event]`
//!   attribute). See: [`Event`].
//! - A **declaration of the errors** that this pallet can throw (denoted by the `#[pallet::error]`
//!   attribute). See: [`Error`].
//! - A **set of dispatchable functions** that define the pallet's functionality (denoted by the
//!   `#[pallet::call]` attribute). See: [`dispatchables`].
//!
//! Run `cargo doc --package pallet-template --open` to view this pallet's documentation.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{Get, StorageVersion},
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize, Member, Saturating, Zero};
	use sp_std::fmt::Debug;

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// The pallet's configuration trait.
	///
	/// All our types and constants a pallet depends on must be declared here.
	/// These types are defined generically and made concrete when the pallet is declared in the
	/// `runtime/src/lib.rs` file of your chain.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		#[allow(deprecated)]
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
		type Balance: Parameter + Member + AtLeast32BitUnsigned + Default + Copy + MaybeSerializeDeserialize + MaxEncodedLen;
	}

	#[pallet::storage]
	pub type TotalSupply<T> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	pub type Balances<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::Balance, ValueQuery>;

	#[pallet::storage]
	pub type Allowances<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId, // owner
		Blake2_128Concat,
		T::AccountId, // spender
		T::Balance,
		ValueQuery,
	>;

	/// Events that functions in this pallet can emit.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Transfer {
			from: T::AccountId,
			to: T::AccountId,
			amount: T::Balance,
		},
		Mint {
			to: T::AccountId,
			amount: T::Balance,
		},
		Burn {
			from: T::AccountId,
			amount: T::Balance,
		},
		Approval {
			owner: T::AccountId,
			spender: T::AccountId,
			amount: T::Balance,
		},
	}

	/// Errors that can be returned by this pallet.
	#[pallet::error]
	pub enum Error<T> {
		InsufficientBalance,
		InsufficientAllowance,
		Overflow,
		SelfTransfer,
		NotAuthorized,
	}

	/// The pallet's dispatchable functions for ORM token operations.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: T::Balance,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
			
			ensure!(from != to, Error::<T>::SelfTransfer);
			
			let from_balance = Balances::<T>::get(&from);
			ensure!(from_balance >= amount, Error::<T>::InsufficientBalance);
			
			Balances::<T>::mutate(&from, |balance| *balance = balance.saturating_sub(amount));
			Balances::<T>::mutate(&to, |balance| *balance = balance.saturating_add(amount));
			
			Self::deposit_event(Event::Transfer { from, to, amount });
			
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn mint(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: T::Balance,
		) -> DispatchResult {
			ensure_root(origin)?;
			
			// Update balances
			Balances::<T>::mutate(&to, |balance| *balance = balance.saturating_add(amount));
			TotalSupply::<T>::mutate(|supply| *supply = supply.saturating_add(amount));
			
			Self::deposit_event(Event::Mint { to, amount });
			
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn burn(
			origin: OriginFor<T>,
			amount: T::Balance,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
			
			let from_balance = Balances::<T>::get(&from);
			ensure!(from_balance >= amount, Error::<T>::InsufficientBalance);
			
			Balances::<T>::mutate(&from, |balance| *balance = balance.saturating_sub(amount));
			TotalSupply::<T>::mutate(|supply| *supply = supply.saturating_sub(amount));
			
			Self::deposit_event(Event::Burn { from, amount });
			
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn approve(
			origin: OriginFor<T>,
			spender: T::AccountId,
			amount: T::Balance,
		) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			
			Allowances::<T>::insert(&owner, &spender, amount);
			
			Self::deposit_event(Event::Approval { owner, spender, amount });
			
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn transfer_from(
			origin: OriginFor<T>,
			from: T::AccountId,
			to: T::AccountId,
			amount: T::Balance,
		) -> DispatchResult {
			let spender = ensure_signed(origin)?;
			
			ensure!(from != to, Error::<T>::SelfTransfer);
			
			let allowance = Allowances::<T>::get(&from, &spender);
			ensure!(allowance >= amount, Error::<T>::InsufficientAllowance);
			
			let from_balance = Balances::<T>::get(&from);
			ensure!(from_balance >= amount, Error::<T>::InsufficientBalance);
			
			Balances::<T>::mutate(&from, |balance| *balance = balance.saturating_sub(amount));
			Balances::<T>::mutate(&to, |balance| *balance = balance.saturating_add(amount));
			
			// Update allowance
			Allowances::<T>::mutate(&from, &spender, |allowance| *allowance = allowance.saturating_sub(amount));
			
			Self::deposit_event(Event::Transfer { from, to, amount });
			
			Ok(())
		}
	}
}
