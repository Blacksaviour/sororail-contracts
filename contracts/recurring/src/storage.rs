use soroban_sdk::{contracttype, Env};
use sororail_common::{storage as ttl, Error};

use crate::types::Authorization;

#[contracttype]
pub enum DataKey {
    /// The single authorization held by this instance.
    Auth,
}

/// Whether `authorize` has run.
pub fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Auth)
}

/// Loads the authorization, extending the instance TTL on the way through.
pub fn load(env: &Env) -> Result<Authorization, Error> {
    let auth = env
        .storage()
        .instance()
        .get(&DataKey::Auth)
        .ok_or(Error::NotInitialized)?;
    ttl::extend_instance(env);
    Ok(auth)
}

/// Persists the authorization and extends the instance TTL.
pub fn save(env: &Env, auth: &Authorization) {
    env.storage().instance().set(&DataKey::Auth, auth);
    ttl::extend_instance(env);
}
