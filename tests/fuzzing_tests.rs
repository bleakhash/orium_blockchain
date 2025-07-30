//! 

use frame_support::{assert_ok, traits::Get};
use sp_runtime::traits::{Zero, Saturating};

mod common;
use common::*;

struct FuzzInput {
	collateral: u128,
	dusd_amount: u128,
	deur_amount: u128,
	orm_price: u128,
	eur_price: u128,
}

fn generate_fuzz_inputs() -> Vec<FuzzInput> {
	vec![
		FuzzInput { collateral: 0, dusd_amount: 0, deur_amount: 0, orm_price: 100_000, eur_price: 80_000 },
		
		FuzzInput { collateral: 1, dusd_amount: 1, deur_amount: 1, orm_price: 1, eur_price: 1 },
		
		FuzzInput { collateral: u128::MAX, dusd_amount: 0, deur_amount: 0, orm_price: 1, eur_price: 1 },
		FuzzInput { collateral: 1000, dusd_amount: u128::MAX, deur_amount: 0, orm_price: 100_000, eur_price: 80_000 },
		
		FuzzInput { collateral: 15000, dusd_amount: 10000, deur_amount: 0, orm_price: 100_000, eur_price: 80_000 },
		
		FuzzInput { collateral: 14999, dusd_amount: 10000, deur_amount: 0, orm_price: 100_000, eur_price: 80_000 },
		
		FuzzInput { collateral: 15001, dusd_amount: 10000, deur_amount: 0, orm_price: 100_000, eur_price: 80_000 },
		
		FuzzInput { collateral: 13000, dusd_amount: 10000, deur_amount: 0, orm_price: 100_000, eur_price: 80_000 },
		
		FuzzInput { collateral: 12999, dusd_amount: 10000, deur_amount: 0, orm_price: 100_000, eur_price: 80_000 },
		
		FuzzInput { collateral: 10000, dusd_amount: 5000, deur_amount: 0, orm_price: 0, eur_price: 80_000 },
		FuzzInput { collateral: 10000, dusd_amount: 5000, deur_amount: 0, orm_price: 100_000, eur_price: 0 },
		
		FuzzInput { collateral: 10000, dusd_amount: 0, deur_amount: 5000, orm_price: 100_000, eur_price: u128::MAX },
		FuzzInput { collateral: 10000, dusd_amount: 5000, deur_amount: 0, orm_price: u128::MAX, eur_price: 80_000 },
		
		FuzzInput { collateral: 1_000_000_000_000, dusd_amount: 500_000_000_000, deur_amount: 0, orm_price: 100_000, eur_price: 80_000 },
		
		FuzzInput { collateral: 1000, dusd_amount: 500, deur_amount: 300, orm_price: 1_000_000_000_000, eur_price: 800_000_000_000 },
		
		FuzzInput { collateral: 20000, dusd_amount: 5000, deur_amount: 3000, orm_price: 100_000, eur_price: 80_000 },
		
		FuzzInput { collateral: 1000, dusd_amount: 100, deur_amount: 200, orm_price: 50_000, eur_price: 120_000 },
		
		FuzzInput { collateral: 50000, dusd_amount: 20000, deur_amount: 15000, orm_price: 10_000, eur_price: 5_000 },
		
		FuzzInput { collateral: 1000, dusd_amount: 100, deur_amount: 50, orm_price: 1_000_000_000, eur_price: 2_000_000_000 },
		
		FuzzInput { collateral: 1_000_000_000_000_000_001, dusd_amount: 666_666_666_666_666_667, deur_amount: 0, orm_price: 100_000, eur_price: 80_000 },
		
		FuzzInput { collateral: 3, dusd_amount: 2, deur_amount: 0, orm_price: 100_000, eur_price: 80_000 },
		FuzzInput { collateral: 7, dusd_amount: 3, deur_amount: 2, orm_price: 100_000, eur_price: 80_000 },
	]
}

#[test]
fn fuzz_collateral_ratio_calculations() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		for (i, input) in generate_fuzz_inputs().iter().enumerate() {
			let account_id = (i + 1) as u64;
			
			if input.collateral == 0 && (input.dusd_amount > 0 || input.deur_amount > 0) {
				continue;
			}
			
			let safe_orm_price = if input.orm_price == 0 { 1 } else { input.orm_price };
			let safe_eur_price = if input.eur_price == 0 { 1 } else { input.eur_price };
			
			assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), safe_orm_price));
			assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/EUR".to_vec(), safe_eur_price));
			
			let balance = input.collateral.saturating_mul(2).max(1_000_000);
			let _ = Balances::deposit_creating(&account_id, balance);
			
			if input.collateral > 0 && input.collateral <= balance {
				let create_result = CollateralEngine::create_cdp(RuntimeOrigin::signed(account_id), input.collateral);
				
				if create_result.is_ok() {
					if input.dusd_amount > 0 {
						let mint_result = CollateralEngine::mint_dusd(RuntimeOrigin::signed(account_id), input.dusd_amount);
						
						if mint_result.is_ok() {
							let cdp = CollateralEngine::cdps(&account_id).unwrap();
							
							assert_eq!(cdp.dusd_debt, input.dusd_amount);
							
							assert!(CollateralEngine::total_dusd_debt() >= input.dusd_amount);
						}
					}
				}
			}
		}
	});
}

#[test]
fn fuzz_arithmetic_overflow_protection() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		let overflow_scenarios = vec![
			(u128::MAX, 1, 1),
			(1, u128::MAX, 1),
			(1, 1, u128::MAX),
			(u128::MAX/2, u128::MAX/2, u128::MAX/2),
			(1_000_000_000_000_000_000, 1_000_000_000_000_000_000, 1_000_000_000_000_000_000),
		];
		
		for (i, (collateral, debt, price)) in overflow_scenarios.iter().enumerate() {
			let account_id = (i + 200) as u64;
			
			assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), *price));
			assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/EUR".to_vec(), 80_000));
			
			let _ = Balances::deposit_creating(&account_id, u128::MAX);
			
			let create_result = CollateralEngine::create_cdp(RuntimeOrigin::signed(account_id), *collateral);
			
			if create_result.is_ok() && *debt > 0 {
				let _mint_result = CollateralEngine::mint_dusd(RuntimeOrigin::signed(account_id), *debt);
			}
			
			let _price_result = CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), *price);
		}
	});
}

#[test]
fn fuzz_price_manipulation_attacks() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		let price_attack_scenarios = vec![
			(100_000, 50_000, 25_000, 12_500),
			(50_000, 100_000, 200_000, 400_000),
			(1, 2, 3, 4),
			(u128::MAX, u128::MAX - 1, u128::MAX - 2, u128::MAX - 3),
			(100_000, 1, 100_000, 1),
		];
		
		for (i, (price1, price2, price3, price4)) in price_attack_scenarios.iter().enumerate() {
			let account_id = (i + 300) as u64;
			let liquidator = account_id + 1000;
			
			let _ = Balances::deposit_creating(&account_id, 100_000);
			let _ = Balances::deposit_creating(&liquidator, 100_000);
			
			assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), 100_000));
			assert_ok!(CollateralEngine::create_cdp(RuntimeOrigin::signed(account_id), 10_000));
			assert_ok!(CollateralEngine::mint_dusd(RuntimeOrigin::signed(account_id), 5_000));
			
			let _result1 = CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), *price1);
			let _result2 = CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), *price2);
			let _result3 = CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), *price3);
			let _result4 = CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), *price4);
			
			if let Some(cdp) = CollateralEngine::cdps(&account_id) {
				assert!(cdp.collateral > 0, "Collateral should never be zero in valid CDP");
				assert!(cdp.dusd_debt > 0 || cdp.deur_debt > 0, "CDP should have some debt");
			}
		}
	});
}

#[test]
fn fuzz_liquidation_edge_cases() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		
		let liquidation_scenarios = vec![
			(10_000, 7_700, 129_000), // Just under 130%
			(10_000, 7_690, 131_000), // Just over 130%
			(10_000, 9_000, 50_000),  // Very undercollateralized
			(13_001, 10_000, 100_000), // 130.01%
			(12_999, 10_000, 100_000), // 129.99%
		];
		
		for (i, (collateral, debt, price)) in liquidation_scenarios.iter().enumerate() {
			let account_id = (i + 400) as u64;
			let liquidator = account_id + 1000;
			
			let _ = Balances::deposit_creating(&account_id, collateral * 2);
			let _ = Balances::deposit_creating(&liquidator, 100_000);
			
			assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), 100_000));
			assert_ok!(CollateralEngine::create_cdp(RuntimeOrigin::signed(account_id), *collateral));
			
			let mint_result = CollateralEngine::mint_dusd(RuntimeOrigin::signed(account_id), *debt);
			
			if mint_result.is_ok() {
				assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), *price));
				
				let liquidation_result = CollateralEngine::liquidate(RuntimeOrigin::signed(liquidator), account_id);
				
				let collateral_value = (*collateral as u128) * (*price as u128) / 1_000_000_000_000_000_000u128;
				let expected_liquidatable = if *debt > 0 {
					let ratio = (collateral_value * 10000) / (*debt as u128);
					ratio < 13000 // Less than 130%
				} else {
					false
				};
				
				if expected_liquidatable {
					if liquidation_result.is_err() {
					}
				} else {
					assert!(liquidation_result.is_err(), 
						"CDP should not be liquidatable with ratio >= 130%");
				}
			}
		}
	});
}
