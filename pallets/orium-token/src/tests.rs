use super::*;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, traits::Get};
use sp_runtime::traits::BadOrigin;

#[test]
fn mint_works() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		
		assert_ok!(OriumToken::mint(RuntimeOrigin::root(), 1, 1000));
		
		// Check that the correct event was deposited
		System::assert_last_event(Event::Mint { to: 1, amount: 1000 }.into());
		
		assert_eq!(OriumToken::balance_of(&1), 1000);
		assert_eq!(OriumToken::total_supply(), 1000);
	});
}

#[test]
fn mint_fails_for_non_root() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			OriumToken::mint(RuntimeOrigin::signed(1), 1, 1000),
			BadOrigin
		);
	});
}

#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		assert_ok!(OriumToken::mint(RuntimeOrigin::root(), 1, 1000));
		
		assert_ok!(OriumToken::transfer(RuntimeOrigin::signed(1), 2, 500));
		
		assert_eq!(OriumToken::balance_of(&1), 500);
		assert_eq!(OriumToken::balance_of(&2), 500);
		
		System::assert_last_event(Event::Transfer { from: 1, to: 2, amount: 500 }.into());
	});
}

#[test]
fn transfer_fails_insufficient_balance() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			OriumToken::transfer(RuntimeOrigin::signed(1), 2, 500),
			Error::<Test>::InsufficientBalance
		);
	});
}

#[test]
fn burn_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		assert_ok!(OriumToken::mint(RuntimeOrigin::root(), 1, 1000));
		
		assert_ok!(OriumToken::burn(RuntimeOrigin::signed(1), 300));
		
		assert_eq!(OriumToken::balance_of(&1), 700);
		assert_eq!(OriumToken::total_supply(), 700);
		
		System::assert_last_event(Event::Burn { from: 1, amount: 300 }.into());
	});
}

#[test]
fn burn_fails_insufficient_balance() {
	new_test_ext().execute_with(|| {
		assert_ok!(OriumToken::mint(RuntimeOrigin::root(), 1, 100));
		
		assert_noop!(
			OriumToken::burn(RuntimeOrigin::signed(1), 200),
			Error::<Test>::InsufficientBalance
		);
	});
}

#[test]
fn approve_and_transfer_from_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		assert_ok!(OriumToken::mint(RuntimeOrigin::root(), 1, 1000));
		
		assert_ok!(OriumToken::approve(RuntimeOrigin::signed(1), 2, 500));
		
		assert_eq!(OriumToken::allowance(&1, &2), 500);
		
		assert_ok!(OriumToken::transfer_from(RuntimeOrigin::signed(2), 1, 3, 300));
		
		assert_eq!(OriumToken::balance_of(&1), 700);
		assert_eq!(OriumToken::balance_of(&3), 300);
		assert_eq!(OriumToken::allowance(&1, &2), 200);
	});
}

#[test]
fn transfer_from_fails_insufficient_allowance() {
	new_test_ext().execute_with(|| {
		assert_ok!(OriumToken::mint(RuntimeOrigin::root(), 1, 1000));
		
		assert_noop!(
			OriumToken::transfer_from(RuntimeOrigin::signed(2), 1, 3, 300),
			Error::<Test>::InsufficientAllowance
		);
	});
}

#[test]
fn multiple_operations_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		assert_ok!(OriumToken::mint(RuntimeOrigin::root(), 1, 1000));
		assert_ok!(OriumToken::mint(RuntimeOrigin::root(), 2, 500));
		
		assert_ok!(OriumToken::transfer(RuntimeOrigin::signed(1), 3, 200));
		assert_ok!(OriumToken::approve(RuntimeOrigin::signed(2), 1, 100));
		assert_ok!(OriumToken::transfer_from(RuntimeOrigin::signed(1), 2, 3, 50));
		
		assert_eq!(OriumToken::balance_of(&1), 800);
		assert_eq!(OriumToken::balance_of(&2), 450);
		assert_eq!(OriumToken::balance_of(&3), 250);
		assert_eq!(OriumToken::total_supply(), 1500);
		assert_eq!(OriumToken::allowance(&2, &1), 50);
	});
}
