//! Authorization guards.
//!
//! Missing authorization checks are the most common Soroban vulnerability
//! class, so the intent is that a privileged entry point reads as one obvious
//! line rather than an ad-hoc comparison chain.
//!
//! Note the ordering rule these helpers enforce: **membership is checked
//! before `require_auth`**. Checking auth first would let an unrelated address
//! that happens to hold a valid signature reach the authorization prompt.

use soroban_sdk::{Address, Env, Vec};

use crate::Error;

/// Errors with `err` unless `cond` holds.
#[inline]
pub fn require(cond: bool, err: Error) -> Result<(), Error> {
    if cond {
        Ok(())
    } else {
        Err(err)
    }
}

/// Requires that `caller` is `expected`, then requires its authorization.
pub fn require_auth_as(caller: &Address, expected: &Address) -> Result<(), Error> {
    require(caller == expected, Error::Unauthorized)?;
    caller.require_auth();
    Ok(())
}

/// Requires that `caller` is one of `allowed`, then requires its authorization.
///
/// `allowed` is expected to be short -- two or three addresses -- so the linear
/// scan is deliberate.
pub fn require_auth_one_of(caller: &Address, allowed: &Vec<Address>) -> Result<(), Error> {
    require(allowed.contains(caller), Error::Unauthorized)?;
    caller.require_auth();
    Ok(())
}

/// Requires that `caller` is either `a` or `b`, then requires its authorization.
pub fn require_auth_either(caller: &Address, a: &Address, b: &Address) -> Result<(), Error> {
    require(caller == a || caller == b, Error::Unauthorized)?;
    caller.require_auth();
    Ok(())
}

/// Requires that `caller` is `a`, or `b` when `b` is present.
///
/// Models the recurring "depositor or the optional arbiter" shape.
pub fn require_auth_either_opt(
    caller: &Address,
    a: &Address,
    b: &Option<Address>,
) -> Result<(), Error> {
    let permitted = caller == a || b.as_ref().is_some_and(|x| caller == x);
    require(permitted, Error::Unauthorized)?;
    caller.require_auth();
    Ok(())
}

/// Convenience for building a short allow-list.
pub fn allow_list(env: &Env, addrs: &[&Address]) -> Vec<Address> {
    let mut v = Vec::new(env);
    for a in addrs {
        v.push_back((*a).clone());
    }
    v
}
