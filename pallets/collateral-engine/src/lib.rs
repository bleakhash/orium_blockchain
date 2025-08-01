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
#[allow(clippy::manual_inspect)]
pub mod pallet {
    // Import various useful types required by all FRAME pallets.
    use super::*;
    use frame_support::{
        pallet_prelude::*,
        traits::{Get, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{
        AtLeast32BitUnsigned, MaybeSerializeDeserialize, Member, Saturating, Zero,
    };
    use sp_runtime::SaturatedConversion;
    use sp_std::vec::Vec;

    // The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
    // (`Call`s) in this pallet.
    #[allow(clippy::manual_inspect)]
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        #[allow(deprecated)]
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;
        type Balance: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + Saturating;
        type Currency: ReservableCurrency<Self::AccountId, Balance = Self::Balance>;
        type MinCollateralRatio: Get<u32>;
        type LiquidationRatio: Get<u32>;
        type StabilityFee: Get<u32>;
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Cdp<Balance> {
        pub collateral: Balance,
        pub dusd_debt: Balance,
        pub deur_debt: Balance,
        pub last_update: u32,
    }

    #[pallet::storage]
    pub type Cdps<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Cdp<T::Balance>, OptionQuery>;

    #[pallet::storage]
    pub type TotalCollateral<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

    #[pallet::storage]
    pub type TotalDusdDebt<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

    #[pallet::storage]
    pub type TotalDeurDebt<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

    #[pallet::storage]
    pub type OrmUsdPrice<T> = StorageValue<_, u128, ValueQuery>;

    #[pallet::storage]
    pub type OrmEurPrice<T> = StorageValue<_, u128, ValueQuery>;

    #[pallet::storage]
    pub type LiquidationQueue<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

    /// Events that functions in this pallet can emit.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CdpCreated {
            owner: T::AccountId,
            collateral: T::Balance,
        },
        CollateralDeposited {
            owner: T::AccountId,
            amount: T::Balance,
        },
        CollateralWithdrawn {
            owner: T::AccountId,
            amount: T::Balance,
        },
        DusdMinted {
            owner: T::AccountId,
            amount: T::Balance,
        },
        DeurMinted {
            owner: T::AccountId,
            amount: T::Balance,
        },
        DusdRepaid {
            owner: T::AccountId,
            amount: T::Balance,
        },
        DeurRepaid {
            owner: T::AccountId,
            amount: T::Balance,
        },
        CdpLiquidated {
            owner: T::AccountId,
            liquidator: T::AccountId,
            collateral_seized: T::Balance,
        },
        PriceUpdated {
            asset: Vec<u8>,
            price: u128,
        },
    }

    /// Errors that can be returned by this pallet.
    #[pallet::error]
    pub enum Error<T> {
        CdpNotFound,
        CdpAlreadyExists,
        InsufficientCollateral,
        CollateralRatioTooLow,
        CdpUndercollateralized,
        InsufficientDebt,
        PriceNotAvailable,
        Overflow,
        NotAuthorized,
        CdpNotLiquidatable,
    }

    /// The pallet's dispatchable functions for CDP management.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn create_cdp(origin: OriginFor<T>, collateral_amount: T::Balance) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(!Cdps::<T>::contains_key(&who), Error::<T>::CdpAlreadyExists);

            T::Currency::reserve(&who, collateral_amount)?;

            let cdp = Cdp {
                collateral: collateral_amount,
                dusd_debt: Zero::zero(),
                deur_debt: Zero::zero(),
                last_update: frame_system::Pallet::<T>::block_number().saturated_into(),
            };

            Cdps::<T>::insert(&who, &cdp);
            TotalCollateral::<T>::mutate(|total| *total = total.saturating_add(collateral_amount));

            Self::deposit_event(Event::CdpCreated {
                owner: who,
                collateral: collateral_amount,
            });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::cause_error())]
        pub fn deposit_collateral(origin: OriginFor<T>, amount: T::Balance) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let mut cdp = Cdps::<T>::get(&who).ok_or(Error::<T>::CdpNotFound)?;

            T::Currency::reserve(&who, amount)?;

            // Update CDP
            cdp.collateral = cdp.collateral.saturating_add(amount);
            cdp.last_update = frame_system::Pallet::<T>::block_number().saturated_into();

            Cdps::<T>::insert(&who, &cdp);
            TotalCollateral::<T>::mutate(|total| *total = total.saturating_add(amount));

            Self::deposit_event(Event::CollateralDeposited { owner: who, amount });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::cause_error())]
        pub fn withdraw_collateral(origin: OriginFor<T>, amount: T::Balance) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let mut cdp = Cdps::<T>::get(&who).ok_or(Error::<T>::CdpNotFound)?;

            ensure!(cdp.collateral >= amount, Error::<T>::InsufficientCollateral);

            let new_collateral = cdp.collateral.saturating_sub(amount);

            if !cdp.dusd_debt.is_zero() || !cdp.deur_debt.is_zero() {
                ensure!(
                    Self::check_collateral_ratio(
                        &who,
                        new_collateral,
                        cdp.dusd_debt,
                        cdp.deur_debt
                    )?,
                    Error::<T>::CollateralRatioTooLow
                );
            }

            // Update CDP
            cdp.collateral = new_collateral;
            cdp.last_update = frame_system::Pallet::<T>::block_number().saturated_into();

            T::Currency::unreserve(&who, amount);
            Cdps::<T>::insert(&who, &cdp);
            TotalCollateral::<T>::mutate(|total| *total = total.saturating_sub(amount));

            Self::deposit_event(Event::CollateralWithdrawn { owner: who, amount });

            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn mint_dusd(origin: OriginFor<T>, amount: T::Balance) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let mut cdp = Cdps::<T>::get(&who).ok_or(Error::<T>::CdpNotFound)?;

            let new_dusd_debt = cdp.dusd_debt.saturating_add(amount);

            ensure!(
                Self::check_collateral_ratio(&who, cdp.collateral, new_dusd_debt, cdp.deur_debt)?,
                Error::<T>::CollateralRatioTooLow
            );

            // Update CDP
            cdp.dusd_debt = new_dusd_debt;
            cdp.last_update = frame_system::Pallet::<T>::block_number().saturated_into();

            Cdps::<T>::insert(&who, &cdp);
            TotalDusdDebt::<T>::mutate(|total| *total = total.saturating_add(amount));

            Self::deposit_event(Event::DusdMinted { owner: who, amount });

            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::cause_error())]
        pub fn update_price(origin: OriginFor<T>, asset: Vec<u8>, price: u128) -> DispatchResult {
            ensure_root(origin)?;

            // Update price based on asset
            if asset == b"ORM/USD" {
                OrmUsdPrice::<T>::put(price);
            } else if asset == b"ORM/EUR" {
                OrmEurPrice::<T>::put(price);
            }

            Self::deposit_event(Event::PriceUpdated { asset, price });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn check_collateral_ratio(
            _who: &T::AccountId,
            collateral: T::Balance,
            dusd_debt: T::Balance,
            deur_debt: T::Balance,
        ) -> Result<bool, Error<T>> {
            let orm_usd_price = OrmUsdPrice::<T>::get();
            let orm_eur_price = OrmEurPrice::<T>::get();

            ensure!(!orm_usd_price.is_zero(), Error::<T>::PriceNotAvailable);
            ensure!(!orm_eur_price.is_zero(), Error::<T>::PriceNotAvailable);

            let collateral_value_usd = (collateral.saturated_into::<u128>())
                .saturating_mul(orm_usd_price)
                .saturating_div(1_000_000_000_000_000_000u128); // Scale down from 1e18

            let dusd_debt_value = dusd_debt.saturated_into::<u128>();
            let deur_debt_value_usd = (deur_debt.saturated_into::<u128>())
                .saturating_mul(orm_eur_price)
                .saturating_div(orm_usd_price); // Convert EUR to USD

            let total_debt_usd = dusd_debt_value.saturating_add(deur_debt_value_usd);

            if total_debt_usd.is_zero() {
                return Ok(true);
            }

            let ratio = collateral_value_usd
                .saturating_mul(10000u128)
                .saturating_div(total_debt_usd);

            Ok(ratio >= T::MinCollateralRatio::get() as u128)
        }
    }
}
