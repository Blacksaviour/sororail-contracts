use soroban_sdk::{contracttype, Env};
use sororail_common::{storage as ttl, Error};

use crate::types::Escrow;

#[contracttype]
pub enum DataKey {
    /// The single escrow agreement held by this instance.
    Escrow,
}

/// Whether `init` has run.
pub fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Escrow)
}

/// Loads the agreement, extending the instance TTL on the way through.
pub fn load(env: &Env) -> Result<Escrow, Error> {
    let escrow = env
        .storage()
        .instance()
        .get(&DataKey::Escrow)
        .ok_or(Error::NotInitialized)?;
    ttl::extend_instance(env);
    Ok(escrow)
}

/// Persists the agreement and extends the instance TTL.
pub fn save(env: &Env, escrow: &Escrow) {
    env.storage().instance().set(&DataKey::Escrow, escrow);
    ttl::extend_instance(env);
}
