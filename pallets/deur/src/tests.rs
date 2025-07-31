use crate::mock::*;
use frame_support::assert_ok;

use crate::mock::{Deur, RuntimeOrigin, System};

#[test]
fn transfer_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(Deur::mint_to(&1, 1000));
        assert_ok!(Deur::transfer(RuntimeOrigin::signed(1), 2, 500));

        assert_eq!(Deur::balance_of(&1), 500);
        assert_eq!(Deur::balance_of(&2), 500);
    });
}

#[test]
fn mint_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(Deur::mint_to(&1, 1000));

        assert_eq!(Deur::balance_of(&1), 1000);
        assert_eq!(Deur::total_supply(), 1000);
    });
}
