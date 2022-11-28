use crate::mock::*;
use frame_support::traits::tokens::fungibles::*;

use crate::Balance;
use frame_support::assert_ok;

const ONE: Balance = 1_000_000_000_000;

#[test]
fn sell_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        let asset_in = HDX;
        let asset_out = DOT;
        assert_ok!(XYK::create_pool(
            Origin::signed(ALICE),
            asset_in,
            100_000_000_000_000,
            asset_out,
            100_000_000_000_000,
        ));

        assert_ok!(XYK::sell(
            Origin::signed(BOB),
            asset_in,
            asset_out,
            5 * ONE,
            0u128,
        ));

        assert_eq!(Tokens::balance(asset_in, &BOB), 5 * ONE);
        assert_eq!(Tokens::balance(asset_out, &BOB), 4_761_904_761_904);
    });
}

#[test]
fn buy_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        let asset_in = HDX;
        let asset_out = DOT;
        assert_ok!(XYK::create_pool(
            Origin::signed(ALICE),
            asset_in,
            100_000_000_000_000,
            asset_out,
            100_000_000_000_000,
        ));

        assert_ok!(XYK::buy(
            Origin::signed(BOB),
            asset_out,
            asset_in,
            4_761_904_761_904,
            u128::MAX,
        ));

        assert_eq!(Tokens::balance(asset_in, &BOB), 5_238_095_238_096);
        assert_eq!(Tokens::balance(asset_out, &BOB), 4_761_904_761_904);
    });
}
