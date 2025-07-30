//! 

use frame_support::{assert_ok, traits::Get};
use sp_runtime::traits::Zero;

mod common;
use common::*;

#[test]
fn end_to_end_stablecoin_minting_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		assert_ok!(OriumToken::mint(RuntimeOrigin::root(), 1, 10_000));
		assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), 100_000)); // $1.00
		assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/EUR".to_vec(), 80_000)); // €0.80
		
		assert_ok!(CollateralEngine::create_cdp(RuntimeOrigin::signed(1), 5_000));
		
		assert_ok!(CollateralEngine::mint_dusd(RuntimeOrigin::signed(1), 3_000));
		
		assert_eq!(Dusd::balance_of(&1), 3_000);
		
		let cdp = CollateralEngine::cdps(&1).unwrap();
		assert_eq!(cdp.collateral, 5_000);
		assert_eq!(cdp.dusd_debt, 3_000);
		assert_eq!(cdp.deur_debt, 0);
		
		assert_eq!(CollateralEngine::total_collateral(), 5_000);
		assert_eq!(CollateralEngine::total_dusd_debt(), 3_000);
		assert_eq!(CollateralEngine::total_deur_debt(), 0);
	});
}

#[test]
fn liquidation_scenario_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		assert_ok!(OriumToken::mint(RuntimeOrigin::root(), 1, 10_000));
		assert_ok!(OriumToken::mint(RuntimeOrigin::root(), 2, 10_000)); // Liquidator
		assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), 100_000));
		assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/EUR".to_vec(), 80_000));
		
		assert_ok!(CollateralEngine::create_cdp(RuntimeOrigin::signed(1), 5_000));
		assert_ok!(CollateralEngine::mint_dusd(RuntimeOrigin::signed(1), 3_000));
		
		assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), 50_000));
		
		
		assert_ok!(CollateralEngine::liquidate(RuntimeOrigin::signed(2), 1));
		
		assert!(CollateralEngine::cdps(&1).is_none());
		
	});
}

#[test]
fn multi_currency_cdp_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		assert_ok!(OriumToken::mint(RuntimeOrigin::root(), 1, 20_000));
		assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), 100_000)); // $1.00
		assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/EUR".to_vec(), 80_000)); // €0.80
		
		assert_ok!(CollateralEngine::create_cdp(RuntimeOrigin::signed(1), 15_000));
		
		assert_ok!(CollateralEngine::mint_dusd(RuntimeOrigin::signed(1), 2_000));
		assert_ok!(CollateralEngine::mint_deur(RuntimeOrigin::signed(1), 1_500));
		
		assert_eq!(Dusd::balance_of(&1), 2_000);
		assert_eq!(Deur::balance_of(&1), 1_500);
		
		let cdp = CollateralEngine::cdps(&1).unwrap();
		assert_eq!(cdp.dusd_debt, 2_000);
		assert_eq!(cdp.deur_debt, 1_500);
		
	});
}

#[test]
fn stablecoin_transfers_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		assert_ok!(OriumToken::mint(RuntimeOrigin::root(), 1, 10_000));
		assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), 100_000));
		assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/EUR".to_vec(), 80_000));
		
		assert_ok!(CollateralEngine::create_cdp(RuntimeOrigin::signed(1), 5_000));
		assert_ok!(CollateralEngine::mint_dusd(RuntimeOrigin::signed(1), 2_000));
		
		assert_ok!(Dusd::transfer(RuntimeOrigin::signed(1), 2, 500));
		
		assert_eq!(Dusd::balance_of(&1), 1_500);
		assert_eq!(Dusd::balance_of(&2), 500);
		
		assert_ok!(Dusd::approve(RuntimeOrigin::signed(2), 3, 200));
		assert_ok!(Dusd::transfer_from(RuntimeOrigin::signed(3), 2, 4, 150));
		
		assert_eq!(Dusd::balance_of(&2), 350);
		assert_eq!(Dusd::balance_of(&4), 150);
		assert_eq!(Dusd::allowance(&2, &3), 50);
	});
}

#[test]
fn price_oracle_updates_affect_liquidations() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		assert_ok!(OriumToken::mint(RuntimeOrigin::root(), 1, 10_000));
		assert_ok!(OriumToken::mint(RuntimeOrigin::root(), 2, 10_000)); // Liquidator
		assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), 100_000));
		assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/EUR".to_vec(), 80_000));
		
		assert_ok!(CollateralEngine::create_cdp(RuntimeOrigin::signed(1), 4_000));
		assert_ok!(CollateralEngine::mint_dusd(RuntimeOrigin::signed(1), 2_600)); // ~154% ratio
		
		assert!(CollateralEngine::liquidate(RuntimeOrigin::signed(2), 1).is_err());
		
		assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), 85_000)); // $0.85
		
		assert!(CollateralEngine::liquidate(RuntimeOrigin::signed(2), 1).is_err());
		
		assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), 80_000)); // $0.80
		
		assert_ok!(CollateralEngine::liquidate(RuntimeOrigin::signed(2), 1));
	});
}
