use crate::mock::*;
use frame_support::assert_ok;

use crate::mock::{Dusd, RuntimeOrigin, System};

#[test]
fn transfer_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(Dusd::mint_to(&1, 1000));
        assert_ok!(Dusd::transfer(RuntimeOrigin::signed(1), 2, 500));

        assert_eq!(Dusd::balance_of(&1), 500);
        assert_eq!(Dusd::balance_of(&2), 500);
    });
}

#[test]
fn mint_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(Dusd::mint_to(&1, 1000));

        assert_eq!(Dusd::balance_of(&1), 1000);
        assert_eq!(Dusd::total_supply(), 1000);
    });
}
