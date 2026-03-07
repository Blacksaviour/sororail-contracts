#![no_std]
#![doc = r#"
Shared primitives for the SoroRail payment contracts.

This crate is not a contract. It holds the pieces that must be identical
across `escrow`, `stream`, `vesting`, `recurring` and `batch_payout`:

- [`errors`] -- one `Error` enum with a stable, documented numeric mapping
- [`storage`] -- TTL constants and bump helpers, because Soroban state expires
- [`auth`] -- authorization guards that check membership before `require_auth`
- [`events`] -- a single topic scheme every contract emits under
- [`math`] -- checked money math that errors instead of saturating
"#]

#[cfg(test)]
extern crate std;

pub mod auth;
pub mod errors;
pub mod events;
pub mod math;
pub mod storage;

pub use errors::Error;

#[cfg(test)]
mod test;
