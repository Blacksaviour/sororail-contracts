//! Vesting events.
//!
//! Follows the org convention in `sororail_common::events`.

use soroban_sdk::{contractevent, Address};

/// A grant was created and funded.
#[contractevent(topics = ["vesting", "created"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Created {
    #[topic]
    pub grantor: Address,
    pub beneficiary: Address,
    pub token: Address,
    pub total: i128,
    pub start: u64,
    pub cliff: u64,
    pub duration: u64,
    pub revocable: bool,
}

/// The beneficiary claimed vested tokens.
#[contractevent(topics = ["vesting", "claimed"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Claimed {
    #[topic]
    pub beneficiary: Address,
    pub amount: i128,
    /// Cumulative claimed after this call.
    pub total_claimed: i128,
}

/// The grantor revoked the grant, reclaiming the unvested portion. The vested
/// portion remains claimable by the beneficiary.
#[contractevent(topics = ["vesting", "revoked"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Revoked {
    #[topic]
    pub grantor: Address,
    pub returned_to_grantor: i128,
    pub still_claimable: i128,
    pub revoked_at: u64,
}
