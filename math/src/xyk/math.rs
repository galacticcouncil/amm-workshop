use crate::{ensure, round_up, to_balance, to_u256};
use core::convert::TryFrom;
use num_traits::Zero;
use primitive_types::U256;

#[cfg(test)]
mod invariants;
#[cfg(test)]
mod tests;

type Balance = u128;

const FIXED_ROUND_UP: Balance = 1;

/// Calculating spot price given reserve of selling asset and reserve of buying asset.
/// Formula : OUT_RESERVE * AMOUNT / IN_RESERVE
///
/// - `in_reserve` - reserve amount of selling ass
/// - `out_reserve` - reserve amount of buying asset
/// - `amount` - amount
///
/// Returns None in case of error
pub fn calculate_spot_price(
    in_reserve: Balance,
    out_reserve: Balance,
    amount: Balance,
) -> Option<Balance> {
    ensure!(in_reserve != 0);

    if amount == 0 || out_reserve == 0 {
        return to_balance!(0);
    }

    let (amount_hp, out_reserve_hp, in_reserve_hp) = to_u256!(amount, out_reserve, in_reserve);

    let spot_price_hp = out_reserve_hp
        .checked_mul(amount_hp)?
        .checked_div(in_reserve_hp)?;

    to_balance!(spot_price_hp)
}

/// Calculating amount to be received from the pool given the amount to be sent to the pool and both reserves.
/// Formula : OUT_RESERVE * AMOUNT_IN / (IN_RESERVE + AMOUNT_IN)
///
/// - `in_reserve` - reserve amount of selling asset
/// - `out_reserve` - reserve amount of buying asset
/// - `amount_in` - amount
///
/// Returns None in case of error
pub fn calculate_out_given_in(
    in_reserve: Balance,
    out_reserve: Balance,
    amount_in: Balance,
) -> Option<Balance> {
    if amount_in == 0 {
        return Some(0);
    };

    let (in_reserve_hp, out_reserve_hp, amount_in_hp) =
        to_u256!(in_reserve, out_reserve, amount_in);

    let denominator = in_reserve_hp.checked_add(amount_in_hp)?;
    ensure!(!denominator.is_zero());

    let numerator = out_reserve_hp.checked_mul(amount_in_hp)?;
    let sale_price_hp = numerator.checked_div(denominator)?;

    to_balance!(sale_price_hp)
}

/// Calculating amount to be sent to the pool given the amount to be received from the pool and both reserves.
/// Formula : IN_RESERVE * AMOUNT_OUT / (OUT_RESERVE - AMOUNT_OUT) + 1
///
/// - `in_reserve` - reserve amount of selling asset
/// - `out_reserve` - reserve amount of buying asset
/// - `amount_out` - buy amount
///
/// Returns None in case of error
pub fn calculate_in_given_out(
    out_reserve: Balance,
    in_reserve: Balance,
    amount_out: Balance,
) -> Option<Balance> {
    if amount_out == 0 {
        return Some(0);
    };
    ensure!(amount_out <= out_reserve);

    let (out_reserve_hp, in_reserve_hp, amount_out_hp) =
        to_u256!(out_reserve, in_reserve, amount_out);

    let numerator = in_reserve_hp.checked_mul(amount_out_hp)?;
    let denominator = out_reserve_hp.checked_sub(amount_out_hp)?;
    ensure!(!denominator.is_zero());
    let buy_price_hp = numerator.checked_div(denominator)?;

    let result = to_balance!(buy_price_hp)?;
    // We are rounding up to prevent value leaking from the pool
    round_up!(result)
}

/// Calculating required amount of asset b given asset a.
/// Formula : AMOUNT * ASSET_B_RESERVE / ASSET_A_RESERVE
///
/// - `asset_a_reserve` - reserve amount of asset a
/// - `asset_b_reserve` - reserve amount of asset b
/// - `amount` - liquidity amount
///
/// Returns None in case of error
pub fn calculate_liquidity_in(
    asset_a_reserve: Balance,
    asset_b_reserve: Balance,
    amount: Balance,
) -> Option<Balance> {
    ensure!(asset_a_reserve != 0);

    if amount.is_zero() || asset_b_reserve.is_zero() {
        return Some(Balance::zero());
    }

    let (a_reserve_hp, b_reserve_hp, amount_hp) =
        to_u256!(asset_a_reserve, asset_b_reserve, amount);

    let b_required_hp = amount_hp
        .checked_mul(b_reserve_hp)
        .and_then(|v| v.checked_div(a_reserve_hp))
        .and_then(|v| v.checked_add(U256::one()))?;

    to_balance!(b_required_hp)
}

/// Calculating amount of assets returned when removing liquidity.
/// Formula A: AMOUNT * ASSET_A_RESERVE / TOTAL_LIQUIDITY
/// Formula B: AMOUNT * ASSET_B_RESERVE / TOTAL_LIQUIDITY
///
/// - `asset_a_reserve` - reserve amount of asset a
/// - `asset_b_reserve` - reserve amount of asset b
/// - `amount` - liquidity amount
///
/// Returns None in case of error
pub fn calculate_liquidity_out(
    asset_a_reserve: Balance,
    asset_b_reserve: Balance,
    amount: Balance,
    total_liquidity: Balance,
) -> Option<(Balance, Balance)> {
    ensure!(total_liquidity != 0);

    let (a_reserve_hp, b_reserve_hp, amount_hp, liquidity_hp) =
        to_u256!(asset_a_reserve, asset_b_reserve, amount, total_liquidity);

    let remove_amount_a_hp = amount_hp
        .checked_mul(a_reserve_hp)?
        .checked_div(liquidity_hp)?;

    let remove_amount_a = to_balance!(remove_amount_a_hp)?;

    let remove_amount_b_hp = b_reserve_hp
        .checked_mul(amount_hp)?
        .checked_div(liquidity_hp)?;

    let remove_amount_b = to_balance!(remove_amount_b_hp)?;

    Some((remove_amount_a, remove_amount_b))
}

/// Calculating amount of shares given to LP for added liquidity
/// shares = issuance * amount / reserve
///
/// - `asset_reserve` - asset reserve
/// - `asset_b_reserve` - amount added by LP
/// - `share_issuance` - total issuance of share asset
///
pub fn calculate_shares(
    asset_reserve: Balance,
    asset_amount: Balance,
    share_issuance: Balance,
) -> Option<Balance> {
    if asset_reserve.is_zero() {
        return None;
    }

    let (reserve_hp, amount_hp, issuance_hp) =
        to_u256!(asset_reserve, asset_amount, share_issuance);

    let result = issuance_hp
        .checked_mul(amount_hp)
        .and_then(|v| v.checked_div(reserve_hp))?;

    to_balance!(result)
}
