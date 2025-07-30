use super::*;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		assert_ok!(DusdPallet::mint_to(&1, 1000));
		
		assert_ok!(DusdPallet::transfer(RuntimeOrigin::signed(1), 2, 500));
		
		assert_eq!(DusdPallet::balance_of(&1), 500);
		assert_eq!(DusdPallet::balance_of(&2), 500);
		
		System::assert_last_event(Event::Transfer { from: 1, to: 2, amount: 500 }.into());
	});
}

#[test]
fn transfer_fails_insufficient_balance() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			DusdPallet::transfer(RuntimeOrigin::signed(1), 2, 500),
			Error::<Test>::InsufficientBalance
		);
	});
}

#[test]
fn approve_and_transfer_from_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		assert_ok!(DusdPallet::mint_to(&1, 1000));
		
		assert_ok!(DusdPallet::approve(RuntimeOrigin::signed(1), 2, 500));
		
		assert_ok!(DusdPallet::transfer_from(RuntimeOrigin::signed(2), 1, 3, 300));
		
		assert_eq!(DusdPallet::balance_of(&1), 700);
		assert_eq!(DusdPallet::balance_of(&3), 300);
		
		System::assert_last_event(Event::Transfer { from: 1, to: 3, amount: 300 }.into());
	});
}

#[test]
fn transfer_from_fails_insufficient_allowance() {
	new_test_ext().execute_with(|| {
		assert_ok!(DusdPallet::mint_to(&1, 1000));
		
		assert_noop!(
			DusdPallet::transfer_from(RuntimeOrigin::signed(2), 1, 3, 300),
			Error::<Test>::InsufficientAllowance
		);
	});
}

#[test]
fn mint_and_burn_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		assert_ok!(DusdPallet::mint_to(&1, 1000));
		assert_eq!(DusdPallet::balance_of(&1), 1000);
		assert_eq!(DusdPallet::total_supply(), 1000);
		
		assert_ok!(DusdPallet::burn_from(&1, 300));
		assert_eq!(DusdPallet::balance_of(&1), 700);
		assert_eq!(DusdPallet::total_supply(), 700);
	});
}
