#![no_std]
#![doc = r#"
Escrow: funds held by the contract, released on a condition.

One agreement per deployed instance. The depositor funds it; the funds then
leave only by release to the beneficiary, refund to the depositor, or an
arbiter's split after a dispute.

```text
Created ──fund──▶ Funded ──release──▶ Released
                    │
                    ├────refund────▶ Refunded
                    │
                    └───dispute────▶ Disputed ──resolve──▶ Resolved
```

Illegal transitions return an error; they never panic.
"#]

#[cfg(test)]
extern crate std;

pub mod contract;
pub mod errors;
pub mod events;
pub mod storage;
pub mod types;

pub use contract::{EscrowContract, EscrowContractClient};
pub use errors::Error;
pub use types::{Escrow, State};

#[cfg(test)]
mod test;
