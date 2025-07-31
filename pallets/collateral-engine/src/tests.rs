use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, traits::Currency};
use sp_runtime::traits::BadOrigin;

use crate::mock::{CollateralEngine, RuntimeOrigin, System, Test};

#[test]
fn create_cdp_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/USD".to_vec(),
            100_000
        )); // $1.00
        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/EUR".to_vec(),
            80_000
        )); // €0.80

        let _ = <Test as crate::Config>::Currency::deposit_creating(&1, 10_000);

        assert_ok!(CollateralEngine::create_cdp(
            RuntimeOrigin::signed(1),
            5_000
        ));

        assert!(CollateralEngine::cdps(&1).is_some());
        let cdp = CollateralEngine::cdps(&1).unwrap();
        assert_eq!(cdp.collateral, 5_000);
        assert_eq!(cdp.dusd_debt, 0);
        assert_eq!(cdp.deur_debt, 0);

        System::assert_last_event(
            Event::CdpCreated {
                owner: 1,
                collateral: 5_000,
            }
            .into(),
        );
    });
}

#[test]
fn create_cdp_fails_already_exists() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let _ = <Test as crate::Config>::Currency::deposit_creating(&1, 10_000);
        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/USD".to_vec(),
            100_000
        ));
        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/EUR".to_vec(),
            80_000
        ));

        assert_ok!(CollateralEngine::create_cdp(
            RuntimeOrigin::signed(1),
            5_000
        ));

        assert_noop!(
            CollateralEngine::create_cdp(RuntimeOrigin::signed(1), 3_000),
            Error::<Test>::CdpAlreadyExists
        );
    });
}

#[test]
fn deposit_collateral_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/USD".to_vec(),
            100_000
        ));
        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/EUR".to_vec(),
            80_000
        ));
        let _ = <Test as crate::Config>::Currency::deposit_creating(&1, 10_000);
        assert_ok!(CollateralEngine::create_cdp(
            RuntimeOrigin::signed(1),
            5_000
        ));

        assert_ok!(CollateralEngine::deposit_collateral(
            RuntimeOrigin::signed(1),
            2_000
        ));

        let cdp = CollateralEngine::cdps(&1).unwrap();
        assert_eq!(cdp.collateral, 7_000);

        assert_eq!(CollateralEngine::total_collateral(), 7_000);

        System::assert_last_event(
            Event::CollateralDeposited {
                owner: 1,
                amount: 2_000,
            }
            .into(),
        );
    });
}

#[test]
fn deposit_collateral_fails_no_cdp() {
    new_test_ext().execute_with(|| {
        let _ = <Test as crate::Config>::Currency::deposit_creating(&1, 10_000);

        assert_noop!(
            CollateralEngine::deposit_collateral(RuntimeOrigin::signed(1), 2_000),
            Error::<Test>::CdpNotFound
        );
    });
}

#[test]
fn mint_dusd_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/USD".to_vec(),
            100_000
        ));
        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/EUR".to_vec(),
            80_000
        ));
        let _ = <Test as crate::Config>::Currency::deposit_creating(&1, 10_000);
        assert_ok!(CollateralEngine::create_cdp(
            RuntimeOrigin::signed(1),
            5_000
        ));

        assert_ok!(CollateralEngine::mint_dusd(RuntimeOrigin::signed(1), 3_000));

        let cdp = CollateralEngine::cdps(&1).unwrap();
        assert_eq!(cdp.dusd_debt, 3_000);

        assert_eq!(CollateralEngine::total_dusd_debt(), 3_000);

        System::assert_last_event(
            Event::DusdMinted {
                owner: 1,
                amount: 3_000,
            }
            .into(),
        );
    });
}

#[test]
fn mint_dusd_fails_insufficient_collateral() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/USD".to_vec(),
            100_000
        ));
        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/EUR".to_vec(),
            80_000
        ));
        let _ = <Test as crate::Config>::Currency::deposit_creating(&1, 2_000);
        assert_ok!(CollateralEngine::create_cdp(
            RuntimeOrigin::signed(1),
            1_000
        ));

        assert_noop!(
            CollateralEngine::mint_dusd(RuntimeOrigin::signed(1), 1_000),
            Error::<Test>::CollateralRatioTooLow
        );
    });
}

#[test]
fn withdraw_collateral_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/USD".to_vec(),
            100_000
        ));
        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/EUR".to_vec(),
            80_000
        ));
        let _ = <Test as crate::Config>::Currency::deposit_creating(&1, 10_000);
        assert_ok!(CollateralEngine::create_cdp(
            RuntimeOrigin::signed(1),
            5_000
        ));

        assert_ok!(CollateralEngine::withdraw_collateral(
            RuntimeOrigin::signed(1),
            2_000
        ));

        let cdp = CollateralEngine::cdps(&1).unwrap();
        assert_eq!(cdp.collateral, 3_000);

        System::assert_last_event(
            Event::CollateralWithdrawn {
                owner: 1,
                amount: 2_000,
            }
            .into(),
        );
    });
}

#[test]
fn withdraw_collateral_fails_with_debt() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/USD".to_vec(),
            100_000
        ));
        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/EUR".to_vec(),
            80_000
        ));
        let _ = <Test as crate::Config>::Currency::deposit_creating(&1, 10_000);
        assert_ok!(CollateralEngine::create_cdp(
            RuntimeOrigin::signed(1),
            5_000
        ));
        assert_ok!(CollateralEngine::mint_dusd(RuntimeOrigin::signed(1), 3_000));

        assert_noop!(
            CollateralEngine::withdraw_collateral(RuntimeOrigin::signed(1), 3_000),
            Error::<Test>::CollateralRatioTooLow
        );
    });
}

#[test]
fn price_update_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/USD".to_vec(),
            120_000
        ));
        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/EUR".to_vec(),
            90_000
        ));

        assert_eq!(CollateralEngine::orm_usd_price(), 120_000);
        assert_eq!(CollateralEngine::orm_eur_price(), 90_000);

        System::assert_has_event(
            Event::PriceUpdated {
                asset: b"ORM/USD".to_vec(),
                price: 120_000,
            }
            .into(),
        );
        System::assert_has_event(
            Event::PriceUpdated {
                asset: b"ORM/EUR".to_vec(),
                price: 90_000,
            }
            .into(),
        );
    });
}

#[test]
fn price_update_fails_for_non_root() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            CollateralEngine::update_price(RuntimeOrigin::signed(1), b"ORM/USD".to_vec(), 120_000),
            BadOrigin
        );
    });
}

#[test]
fn collateral_ratio_calculation_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/USD".to_vec(),
            100_000
        )); // $1.00
        assert_ok!(CollateralEngine::update_price(
            RuntimeOrigin::root(),
            b"ORM/EUR".to_vec(),
            80_000
        )); // €0.80

        let _ = <Test as crate::Config>::Currency::deposit_creating(&1, 10_000);
        assert_ok!(CollateralEngine::create_cdp(
            RuntimeOrigin::signed(1),
            5_000
        ));

        assert_ok!(CollateralEngine::mint_dusd(RuntimeOrigin::signed(1), 3_000));

        assert_noop!(
            CollateralEngine::mint_dusd(RuntimeOrigin::signed(1), 500),
            Error::<Test>::CollateralRatioTooLow
        );
    });
}
