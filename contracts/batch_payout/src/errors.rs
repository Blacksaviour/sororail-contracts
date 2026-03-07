//! Batch payout error surface.
//!
//! Errors are shared org-wide -- see `sororail_common::errors`, where
//! batch_payout owns the numeric range **100–119**. This module re-exports the
//! enum and documents which variants this contract can return:
//!
//! | Variant | Raised by |
//! |---|---|
//! | [`Error::BatchEmpty`] | a batch with no recipients |
//! | [`Error::BatchTooLarge`] | more recipients than [`crate::types::MAX_RECIPIENTS`] |
//! | [`Error::InvalidAmount`] | any non-positive amount in the batch |
//! | [`Error::Overflow`] | the batch total exceeding `i128` |
//!
//! Number 102 was `BatchDuplicateRecipient`, removed before release: detecting
//! duplicates on-chain costs a quadratic scan, and paying one address twice in
//! a batch is legitimate. That check belongs in the client's CSV import.
//!
//! There is no `NotInitialized`: this contract is stateless.

pub use sororail_common::Error;
