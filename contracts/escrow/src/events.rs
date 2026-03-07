//! Escrow events.
//!
//! Typed `#[contractevent]` structs following the org convention documented in
//! `sororail_common::events`: fixed topics `[contract, action]`, then the
//! attributed address as the single `#[topic]` field, with amounts as data.

use soroban_sdk::{contractevent, Address};

/// `init` succeeded. The escrow is configured but not yet funded.
#[contractevent(topics = ["escrow", "created"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Created {
    #[topic]
    pub depositor: Address,
    pub beneficiary: Address,
    pub token: Address,
    pub amount: i128,
    pub deadline: u64,
}

/// Funds have been pulled from the depositor into the contract.
#[contractevent(topics = ["escrow", "funded"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Funded {
    #[topic]
    pub depositor: Address,
    pub amount: i128,
}

/// Paid out to the beneficiary.
#[contractevent(topics = ["escrow", "released"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Released {
    #[topic]
    pub beneficiary: Address,
    pub amount: i128,
    /// The depositor or the arbiter.
    pub released_by: Address,
}

/// Returned to the depositor.
#[contractevent(topics = ["escrow", "refunded"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Refunded {
    #[topic]
    pub depositor: Address,
    pub amount: i128,
    /// The depositor (after the deadline) or the arbiter (at any time).
    pub refunded_by: Address,
}

/// A party froze the escrow pending an arbiter decision.
#[contractevent(topics = ["escrow", "disputed"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Disputed {
    #[topic]
    pub raised_by: Address,
}

/// The arbiter split the funds between the parties.
#[contractevent(topics = ["escrow", "resolved"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Resolved {
    #[topic]
    pub arbiter: Address,
    /// Share paid to the beneficiary, in basis points.
    pub split_bps: u32,
    pub to_beneficiary: i128,
    pub to_depositor: i128,
}
