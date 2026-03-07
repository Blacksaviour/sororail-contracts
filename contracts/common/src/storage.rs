//! TTL constants and bump helpers.
//!
//! Soroban state expires. An entry that is not extended is archived, and a
//! contract whose config has been archived cannot run. Every entry point that
//! touches storage must therefore extend the TTL of what it touched, so that
//! logic lives here and is written once rather than forgotten in one contract
//! out of six.

use soroban_sdk::{Env, IntoVal, Val};

/// Ledgers closed in a day, at the ~5s close time Soroban targets.
pub const DAY_IN_LEDGERS: u32 = 17_280;

/// Instance entries (contract config) are extended to 30 days...
pub const INSTANCE_BUMP: u32 = 30 * DAY_IN_LEDGERS;
/// ...whenever they fall below 29 days remaining.
pub const INSTANCE_THRESHOLD: u32 = INSTANCE_BUMP - DAY_IN_LEDGERS;

/// Persistent entries (per-position state) are extended to 90 days...
pub const PERSISTENT_BUMP: u32 = 90 * DAY_IN_LEDGERS;
/// ...whenever they fall below 83 days remaining.
pub const PERSISTENT_THRESHOLD: u32 = PERSISTENT_BUMP - 7 * DAY_IN_LEDGERS;

/// Extends the TTL of the contract instance and its code.
pub fn extend_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_THRESHOLD, INSTANCE_BUMP);
}

/// Extends the TTL of one persistent entry.
pub fn extend_persistent<K>(env: &Env, key: &K)
where
    K: IntoVal<Env, Val>,
{
    env.storage()
        .persistent()
        .extend_ttl(key, PERSISTENT_THRESHOLD, PERSISTENT_BUMP);
}
