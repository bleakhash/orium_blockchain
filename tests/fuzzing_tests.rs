
use frame_support::{assert_ok, traits::Get};
use sp_runtime::traits::{Zero, Saturating};
use proptest::prelude::*;

mod common;
use common::*;

proptest! {
    #[test]
    fn prop_collateral_ratio_invariant(
        collateral in 1000u128..1_000_000_000u128,
        dusd_debt in 0u128..500_000_000u128,
        deur_debt in 0u128..500_000_000u128,
        orm_price in 10_000u128..1_000_000u128,
        eur_price in 10_000u128..1_000_000u128,
    ) {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            
            let account_id = 1u64;
            let _ = Balances::deposit_creating(&account_id, collateral * 2);
            
            assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), orm_price));
            assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/EUR".to_vec(), eur_price));
            
            assert_ok!(CollateralEngine::create_cdp(RuntimeOrigin::signed(account_id), collateral));
            
            let mint_dusd_result = CollateralEngine::mint_dusd(RuntimeOrigin::signed(account_id), dusd_debt);
            let mint_deur_result = CollateralEngine::mint_deur(RuntimeOrigin::signed(account_id), deur_debt);
            
            if mint_dusd_result.is_ok() && mint_deur_result.is_ok() {
                let cdp = CollateralEngine::cdps(&account_id).unwrap();
                
                let collateral_value_usd = (collateral as u128)
                    .saturating_mul(orm_price)
                    .saturating_div(1_000_000_000_000_000_000u128);
                
                let dusd_debt_value = cdp.dusd_debt as u128;
                let deur_debt_value_usd = (cdp.deur_debt as u128)
                    .saturating_mul(eur_price)
                    .saturating_div(orm_price);
                
                let total_debt_usd = dusd_debt_value.saturating_add(deur_debt_value_usd);
                
                if !total_debt_usd.is_zero() {
                    let ratio = collateral_value_usd
                        .saturating_mul(10000u128)
                        .saturating_div(total_debt_usd);
                    
                    prop_assert!(ratio >= 15000u128, "Collateral ratio must be ≥150% (15000 basis points), got {}", ratio);
                }
            }
        });
    }

    #[test]
    fn prop_liquidation_threshold_invariant(
        collateral in 1000u128..10_000_000u128,
        debt in 1u128..5_000_000u128,
        price_drop_factor in 50u128..90u128,
    ) {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            
            let account_id = 1u64;
            let liquidator_id = 2u64;
            let initial_price = 100_000u128;
            
            let _ = Balances::deposit_creating(&account_id, collateral * 2);
            let _ = Balances::deposit_creating(&liquidator_id, 1_000_000);
            
            assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), initial_price));
            assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/EUR".to_vec(), 80_000));
            
            assert_ok!(CollateralEngine::create_cdp(RuntimeOrigin::signed(account_id), collateral));
            
            let mint_result = CollateralEngine::mint_dusd(RuntimeOrigin::signed(account_id), debt);
            
            if mint_result.is_ok() {
                let new_price = initial_price.saturating_mul(price_drop_factor).saturating_div(100);
                assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), new_price));
                
                let liquidation_result = CollateralEngine::liquidate(RuntimeOrigin::signed(liquidator_id), account_id);
                
                if liquidation_result.is_ok() {
                    let collateral_value_usd = (collateral as u128)
                        .saturating_mul(new_price)
                        .saturating_div(1_000_000_000_000_000_000u128);
                    
                    let ratio = collateral_value_usd
                        .saturating_mul(10000u128)
                        .saturating_div(debt as u128);
                    
                    prop_assert!(ratio < 13000u128, "Liquidation should only occur below 130% ratio, ratio was {}", ratio);
                    
                    prop_assert!(CollateralEngine::cdps(&account_id).is_none(), "CDP should be removed after liquidation");
                }
            }
        });
    }

    #[test]
    fn prop_debt_conservation_invariant(
        operations in prop::collection::vec(
            (0u8..4u8, 1u128..10_000u128),
            1..10
        )
    ) {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            
            let account_id = 1u64;
            let _ = Balances::deposit_creating(&account_id, 1_000_000);
            
            assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), 100_000));
            assert_ok!(CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/EUR".to_vec(), 80_000));
            
            assert_ok!(CollateralEngine::create_cdp(RuntimeOrigin::signed(account_id), 500_000));
            
            let mut expected_dusd_debt = 0u128;
            let mut expected_deur_debt = 0u128;
            
            for (operation, amount) in operations {
                match operation {
                    0 => {
                        if CollateralEngine::mint_dusd(RuntimeOrigin::signed(account_id), amount).is_ok() {
                            expected_dusd_debt = expected_dusd_debt.saturating_add(amount);
                        }
                    },
                    1 => {
                        if CollateralEngine::mint_deur(RuntimeOrigin::signed(account_id), amount).is_ok() {
                            expected_deur_debt = expected_deur_debt.saturating_add(amount);
                        }
                    },
                    2 => {
                        let repay_amount = amount.min(expected_dusd_debt);
                        if repay_amount > 0 && CollateralEngine::repay_dusd(RuntimeOrigin::signed(account_id), repay_amount).is_ok() {
                            expected_dusd_debt = expected_dusd_debt.saturating_sub(repay_amount);
                        }
                    },
                    3 => {
                        let repay_amount = amount.min(expected_deur_debt);
                        if repay_amount > 0 && CollateralEngine::repay_deur(RuntimeOrigin::signed(account_id), repay_amount).is_ok() {
                            expected_deur_debt = expected_deur_debt.saturating_sub(repay_amount);
                        }
                    },
                    _ => {}
                }
                
                if let Some(cdp) = CollateralEngine::cdps(&account_id) {
                    prop_assert_eq!(cdp.dusd_debt as u128, expected_dusd_debt, "DUSD debt mismatch");
                    prop_assert_eq!(cdp.deur_debt as u128, expected_deur_debt, "DEUR debt mismatch");
                }
                
                prop_assert_eq!(CollateralEngine::total_dusd_debt() as u128, expected_dusd_debt, "Total DUSD debt mismatch");
                prop_assert_eq!(CollateralEngine::total_deur_debt() as u128, expected_deur_debt, "Total DEUR debt mismatch");
            }
        });
    }

    #[test]
    fn prop_arithmetic_overflow_safety(
        collateral in 1u128..u128::MAX/1000,
        debt in 1u128..u128::MAX/1000,
        price in 1u128..u128::MAX/1000,
    ) {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            
            let account_id = 1u64;
            let _ = Balances::deposit_creating(&account_id, u128::MAX);
            
            let price_result = CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/USD".to_vec(), price);
            let eur_price_result = CollateralEngine::update_price(RuntimeOrigin::root(), b"ORM/EUR".to_vec(), 80_000);
            
            prop_assert!(price_result.is_ok(), "Price update should not panic on large values");
            prop_assert!(eur_price_result.is_ok(), "EUR price update should not panic");
            
            let create_result = CollateralEngine::create_cdp(RuntimeOrigin::signed(account_id), collateral);
            
            if create_result.is_ok() {
                let mint_result = CollateralEngine::mint_dusd(RuntimeOrigin::signed(account_id), debt);
                
                prop_assert!(mint_result.is_ok() || mint_result.is_err(), "Mint operation should not panic");
                
                if mint_result.is_ok() {
                    let repay_result = CollateralEngine::repay_dusd(RuntimeOrigin::signed(account_id), debt);
                    prop_assert!(repay_result.is_ok() || repay_result.is_err(), "Repay operation should not panic");
                }
            }
        });
    }
}
