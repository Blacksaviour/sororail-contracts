//! Batch payout events.
//!
//! Follows the org convention in `sororail_common::events`.

use soroban_sdk::{contractevent, Address};

/// A batch completed. Emitted once per batch, not once per recipient --
/// per-recipient detail is available from the token's own transfer events,
/// and duplicating it here would bloat the ledger for large payrolls.
#[contractevent(topics = ["batch_payout", "executed"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Executed {
    #[topic]
    pub funder: Address,
    pub token: Address,
    pub count: u32,
    pub total: i128,
}
