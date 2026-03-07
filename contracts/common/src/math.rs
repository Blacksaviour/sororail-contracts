//! Checked money math.
//!
//! Every operation here returns [`Error`] rather than wrapping, saturating or
//! panicking. Saturation is the dangerous default for balances: it silently
//! converts an overflow into a wrong-but-plausible number, and a payment
//! contract that reports a wrong balance is worse than one that refuses to act.

use crate::Error;

/// Denominator for basis-point math. 10000 bps == 100%.
pub const BPS_DENOMINATOR: i128 = 10_000;

/// Maximum valid basis points.
pub const MAX_BPS: u32 = 10_000;

#[inline]
pub fn add(a: i128, b: i128) -> Result<i128, Error> {
    a.checked_add(b).ok_or(Error::Overflow)
}

#[inline]
pub fn sub(a: i128, b: i128) -> Result<i128, Error> {
    a.checked_sub(b).ok_or(Error::Underflow)
}

#[inline]
pub fn mul(a: i128, b: i128) -> Result<i128, Error> {
    a.checked_mul(b).ok_or(Error::Overflow)
}

#[inline]
pub fn div(a: i128, b: i128) -> Result<i128, Error> {
    if b == 0 {
        return Err(Error::DivisionByZero);
    }
    a.checked_div(b).ok_or(Error::Overflow)
}

/// `a * b / d`, rounded toward zero.
///
/// The intermediate product is computed in `i128` and errors on overflow
/// rather than widening. Callers doing proportional math over very large
/// totals (`total * elapsed / duration`) should be aware that an overflow is
/// reported as [`Error::Overflow`], not silently truncated.
pub fn mul_div(a: i128, b: i128, d: i128) -> Result<i128, Error> {
    div(mul(a, b)?, d)
}

/// `amount * bps / 10000`, rounded toward zero.
pub fn mul_bps(amount: i128, bps: u32) -> Result<i128, Error> {
    if bps > MAX_BPS {
        return Err(Error::InvalidBasisPoints);
    }
    mul_div(amount, bps as i128, BPS_DENOMINATOR)
}

/// Splits `amount` into `(first, second)` where `first` is `bps` of the total.
///
/// `second` is computed as the remainder rather than as `10000 - bps`, so the
/// two parts always sum to exactly `amount` with no rounding leakage. This is
/// the property escrow dispute resolution depends on.
pub fn split_bps(amount: i128, bps: u32) -> Result<(i128, i128), Error> {
    let first = mul_bps(amount, bps)?;
    let second = sub(amount, first)?;
    Ok((first, second))
}

/// Errors unless `amount` is strictly positive.
#[inline]
pub fn require_positive(amount: i128) -> Result<(), Error> {
    if amount <= 0 {
        return Err(Error::InvalidAmount);
    }
    Ok(())
}

/// Errors unless `amount` is zero or positive.
#[inline]
pub fn require_non_negative(amount: i128) -> Result<(), Error> {
    if amount < 0 {
        return Err(Error::InvalidAmount);
    }
    Ok(())
}

/// Returns the smaller of two values.
#[inline]
pub fn min(a: i128, b: i128) -> i128 {
    if a < b {
        a
    } else {
        b
    }
}
