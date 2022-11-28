use crate::xyk::*;
use std::vec;

#[test]
fn spot_price_should_work() {
    let cases = vec![
        (1000, 2000, 500, Some(1000), "Easy case"),
        (1, 1, 1, Some(1), "Easy case"),
        (1, 0, 1, Some(0), "Zero buy_reserve"),
        (1, 1, 0, Some(0), "Zero amount"),
        (u128::MAX, u128::MAX - 1, 1, Some(0), "Truncated result"),
    ];

    for case in cases {
        assert_eq!(
            calculate_spot_price(case.0, case.1, case.2),
            case.3,
            "{}",
            case.4
        );
    }
}

#[test]
fn out_given_in_should_work() {
    let cases = vec![
        (1000, 2000, 500, Some(666), "Easy case"),
        (1000, 1000, 0, Some(0), "Zero amount in"),
        (
            0,
            u128::MAX,
            u128::MAX,
            Some(u128::MAX),
            "Zero sell reserve",
        ),
        (0, 0, 0, Some(0), "Zero reserves and weights"),
        (0, 1, 0, Some(0), "Zero sell reserve and amount"),
        (1, 0, 0, Some(0), "Zero buy reserve and amount"),
        (
            0,
            0,
            u128::MAX,
            Some(0),
            "Zero buy reserve and sell reserve",
        ),
    ];

    for case in cases {
        assert_eq!(
            calculate_out_given_in(case.0, case.1, case.2),
            case.3,
            "{}",
            case.4
        );
    }
}

#[test]
fn in_given_out_should_work() {
    let cases = vec![
        (2000, 1000, 500, Some(334), "Easy case"),
        (1000, 1000, 0, Some(0), "Zero amount out"),
        (0, 0, 0, Some(0), "Zero reserves and weights"),
        (0, 1, 0, Some(0), "Zero buy reserve and amount"),
        (1000, 1000, 1000, None, "Zero reserves and weights"),
        (0, 10, 1000, None, "amount cannot be > buy reserve"),
        (0, u128::MAX, u128::MAX, None, "div by zero"),
        (
            u128::MAX,
            u128::MAX,
            u128::MAX - 1,
            None,
            "Overflow weights",
        ),
    ];

    for case in cases {
        assert_eq!(
            calculate_in_given_out(case.0, case.1, case.2),
            case.3,
            "{}",
            case.4
        );
    }
}

#[test]
fn add_liquidity_should_work() {
    let cases = vec![
        (1000, 2000, 500, Some(1001), "Easy case"),
        (100, 100, 0, Some(0), "amount is zero"),
        (110, 0, 100, Some(0), "asset b is zero"),
        (1, u128::MAX, u128::MAX, None, "asset b and amount are zero"),
    ];

    for case in cases {
        assert_eq!(
            calculate_liquidity_in(case.0, case.1, case.2),
            case.3,
            "{}",
            case.4
        );
    }
}

#[test]
fn remove_liquidity_should_work() {
    let cases = vec![
        (1000, 2000, 500, 2500, Some((200, 400)), "Easy case"),
        (100, 100, 100, 0, None, "total liquidity is zero"),
        (0, 0, 0, 100, Some((0, 0)), "amount is zero"),
        (0, 110, 100, 100, Some((0, 110)), "remove amount a is zero"),
        (110, 0, 100, 100, Some((110, 0)), "remove amount b is zero"),
        (u128::MAX, 0, u128::MAX, 1, None, "Formula a overflow"),
        (0, u128::MAX, u128::MAX, 1, None, "Formula b overflow"),
    ];

    for case in cases {
        assert_eq!(
            calculate_liquidity_out(case.0, case.1, case.2, case.3),
            case.4,
            "{}",
            case.5
        );
    }
}

#[test]
fn calculate_shares_should_work() {
    let one: Balance = 1_000_000_000_000;

    let cases = vec![
        (
            100 * one,
            one,
            10000 * one,
            Some(100000000000000),
            "Easy case",
        ),
        (
            100 * one,
            15 * one,
            143 * one,
            Some(21450000000000),
            "Easy case",
        ),
        (
            0u128,
            one,
            10000 * one,
            Some(1000000000000),
            "initial shares",
        ),
    ];

    for case in cases {
        assert_eq!(
            calculate_shares(case.0, case.1, case.2),
            case.3,
            "{}",
            case.4
        );
    }
}
