use sp_runtime::{FixedPointNumber, FixedU128};

#[test]
fn rounding() {
    let l = 5285892814623836473811814630000000_u128;

    let r = 5285892814623878169850251600000000_u128;

    let result = FixedU128::checked_from_rational(l, r).unwrap();

    dbg!(result);
}
