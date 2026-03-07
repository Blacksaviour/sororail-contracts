//! Recurring error surface.
//!
//! Errors are shared org-wide -- see `sororail_common::errors`, where recurring
//! owns the numeric range **80–99**. This module re-exports the enum and
//! documents which variants this contract can return:
//!
//! | Variant | Raised by |
//! |---|---|
//! | [`Error::AlreadyInitialized`] | `authorize` on an initialized instance |
//! | [`Error::NotInitialized`] | any entry point before `authorize` |
//! | [`Error::Unauthorized`] | `cancel` by someone who is neither party |
//! | [`Error::InvalidAmount`] | non-positive `amount_per_period`, or `max_periods == Some(0)` |
//! | [`Error::InvalidDuration`] | zero `period_seconds` |
//! | [`Error::RecurringCancelled`] | `charge` or `cancel` after cancellation |
//! | [`Error::RecurringPeriodNotElapsed`] | `charge` before `next_chargeable_at` |
//! | [`Error::RecurringExhausted`] | `charge` once `max_periods` is reached |

pub use sororail_common::Error;
