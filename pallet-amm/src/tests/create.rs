use crate::mock::*;

use frame_support::assert_ok;

#[test]
fn create_pool_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        let asset_a = HDX;
        let asset_b = DOT;
        assert_ok!(XYK::create_pool(
            Origin::signed(ALICE),
            asset_a,
            100_000_000_000_000,
            asset_b,
            100_000_000_000_000,
        ));
    });
}
