//! 

use frame_support::{assert_ok, traits::Get};
use sp_runtime::traits::Zero;

mod common;
use common::*;

fn generate_test_scenarios() -> Vec<(u128, u128, u128, u128)> {
	vec![
		(10_000, 5_000, 2_000, 100_000), // Normal case
		(1_000, 500, 200, 100_000),      // Small amounts
		(100_000, 50_000, 20_000, 100_000), // Large amounts
		(5_000, 3_000, 1_000, 150_000), // High price
		(5_000, 3_000, 1_000, 50_000),  // Low price
		(1_000, 0, 0, 100_000),          // No debt
		(10_000, 6_000, 0, 100_000),     // Only dUSD debt
		(10_000, 0, 4_000, 100_000),     // Only dEUR debt
	]
}

#[test]
fn property_collateral_ratio_invariant() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		for (collateral, dusd_debt, deur_debt, orm_price) in generate_test_scenarios() {
			assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), orm_price));
			assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/EUR".to_vec(), 80_000)); // €0.80
			
			let account_id = 1u64;
			let _ = Balances::deposit_creating(&account_id, collateral * 2);
			
			if CollateralEngine::cdps(&account_id).is_none() {
				assert_ok!(CollateralEngine::create_cdp(RuntimeOrigin::signed(account_id), collateral));
			}
			
			if dusd_debt > 0 {
				let mint_result = CollateralEngine::mint_dusd(RuntimeOrigin::signed(account_id), dusd_debt);
				if mint_result.is_ok() {
					let cdp = CollateralEngine::cdps(&account_id).unwrap();
					let collateral_value = (cdp.collateral as u128) * orm_price / 1_000_000_000_000_000_000u128;
					let debt_value = cdp.dusd_debt as u128;
					
					if debt_value > 0 {
						let ratio = (collateral_value * 10000) / debt_value;
						assert!(ratio >= 15000, "Collateral ratio {} < 150% for collateral={}, debt={}, price={}", 
							ratio, cdp.collateral, cdp.dusd_debt, orm_price);
					}
				}
			}
		}
	});
}

#[test]
fn property_total_supply_conservation() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), 100_000));
		assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/EUR".to_vec(), 80_000));
		
		let initial_orm_supply = OriumToken::total_supply();
		let initial_dusd_supply = Dusd::total_supply();
		let initial_deur_supply = Deur::total_supply();
		
		let _ = Balances::deposit_creating(&1, 20_000);
		let _ = Balances::deposit_creating(&2, 20_000);
		
		assert_ok!(OriumToken::mint(RuntimeOrigin::root(), 1, 10_000));
		assert_ok!(OriumToken::mint(RuntimeOrigin::root(), 2, 5_000));
		
		assert_ok!(CollateralEngine::create_cdp(RuntimeOrigin::signed(1), 8_000));
		assert_ok!(CollateralEngine::create_cdp(RuntimeOrigin::signed(2), 4_000));
		
		assert_ok!(CollateralEngine::mint_dusd(RuntimeOrigin::signed(1), 4_000));
		assert_ok!(CollateralEngine::mint_dusd(RuntimeOrigin::signed(2), 2_000));
		
		let final_orm_supply = OriumToken::total_supply();
		let final_dusd_supply = Dusd::total_supply();
		let final_deur_supply = Deur::total_supply();
		
		assert_eq!(final_orm_supply, initial_orm_supply + 15_000);
		
		assert_eq!(final_dusd_supply, initial_dusd_supply + 6_000);
		
		assert_eq!(final_deur_supply, initial_deur_supply);
		
		let total_orm_balances = OriumToken::balance_of(&1) + OriumToken::balance_of(&2);
		let total_dusd_balances = Dusd::balance_of(&1) + Dusd::balance_of(&2);
		
		let total_collateral = CollateralEngine::total_collateral();
		assert_eq!(total_orm_balances + total_collateral, final_orm_supply);
		assert_eq!(total_dusd_balances, final_dusd_supply);
	});
}

#[test]
fn property_liquidation_conditions() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		let price_scenarios = vec![
			(100_000, false), // $1.00 - healthy
			(90_000, false),  // $0.90 - still healthy
			(80_000, false),  // $0.80 - borderline
			(70_000, true),   // $0.70 - should be liquidatable
			(60_000, true),   // $0.60 - definitely liquidatable
		];
		
		for (price, should_be_liquidatable) in price_scenarios {
			let account_id = 100 + (price / 1000); // Unique account per price
			let liquidator = account_id + 1000;
			
			let _ = Balances::deposit_creating(&account_id, 20_000);
			let _ = Balances::deposit_creating(&liquidator, 20_000);
			
			assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), 100_000));
			assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/EUR".to_vec(), 80_000));
			
			assert_ok!(CollateralEngine::create_cdp(RuntimeOrigin::signed(account_id), 5_000));
			assert_ok!(CollateralEngine::mint_dusd(RuntimeOrigin::signed(account_id), 3_000));
			
			assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), price));
			
			let liquidation_result = CollateralEngine::liquidate(RuntimeOrigin::signed(liquidator), account_id);
			
			if should_be_liquidatable {
				assert!(liquidation_result.is_ok(), 
					"CDP should be liquidatable at price {} but liquidation failed", price);
				
				assert!(CollateralEngine::cdps(&account_id).is_none(),
					"CDP should be removed after liquidation");
			} else {
				assert!(liquidation_result.is_err(),
					"CDP should not be liquidatable at price {} but liquidation succeeded", price);
				
				assert!(CollateralEngine::cdps(&account_id).is_some(),
					"CDP should still exist when not liquidatable");
			}
		}
	});
}
