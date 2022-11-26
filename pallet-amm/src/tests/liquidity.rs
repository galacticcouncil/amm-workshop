use crate::mock::*;
use frame_support::traits::tokens::fungibles::*;

use crate::Balance;
use frame_support::assert_ok;

const ONE: Balance = 1_000_000_000_000;

#[test]
fn add_liquidity_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        let asset_a = HDX;
        let asset_b = DOT;
        assert_ok!(XYK::create_pool(
            Origin::signed(ALICE),
            asset_a,
            20_000_000_000_000,
            asset_b,
            10_000_000_000_000,
        ));

        assert_eq!(Tokens::balance(asset_a, &CHARLIE), 10 * ONE);

        assert_ok!(XYK::add_liquidity(
            Origin::signed(CHARLIE),
            asset_a,
            asset_b,
            5 * ONE,
            u128::MAX,
        ));

        assert_eq!(Tokens::balance(asset_a, &CHARLIE), 5 * ONE);
        assert_eq!(Tokens::balance(asset_b, &CHARLIE), 7499_999_999_999);
        assert_eq!(
            Tokens::balance(POOL_SHARE_ASSET, &CHARLIE),
            5000_000_000_000
        );
    });
}

#[test]
fn add_liquidity_should_work_when_assets_are_inverted() {
    ExtBuilder::default().build().execute_with(|| {
        let asset_a = HDX;
        let asset_b = DOT;
        assert_ok!(XYK::create_pool(
            Origin::signed(ALICE),
            asset_a,
            20_000_000_000_000,
            asset_b,
            10_000_000_000_000,
        ));

        assert_eq!(Tokens::balance(asset_a, &CHARLIE), 10 * ONE);

        assert_ok!(XYK::add_liquidity(
            Origin::signed(CHARLIE),
            asset_b,
            asset_a,
            2500_000_000_001,
            u128::MAX,
        ));

        assert_eq!(Tokens::balance(asset_a, &CHARLIE), 4999_999_999_997);
        assert_eq!(Tokens::balance(asset_b, &CHARLIE), 7499_999_999_999);
        assert_eq!(
            Tokens::balance(POOL_SHARE_ASSET, &CHARLIE),
            5000_000_000_002
        );
    });
}
