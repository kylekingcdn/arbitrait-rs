#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]

/// Tie-breaking (or overruling) logic for settling a vote
pub mod arbitrate;

/// Provides types/traits used to attach concrete instances of proof to back up a decision
pub mod evidence;

/// Types/traits for the subject matter pertaining to a given vote
pub mod proposal;

/// Post-vote breakdown types
pub mod record;

/// Primary traits for voting and
pub mod vote;
