#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]

/// Types/traits for the choices provided to a voter
pub mod ballot;

/// Provides types/traits used to attach concrete instances of proof to back up a decision
pub mod evidence;

/// Types/traits for the subject matter of a given vote
pub mod proposal;
pub use proposal::Propose;

/// Types/traits for votes and vote casting
pub mod vote;
pub use vote::{CastVote, IterVoteCollector, Outcome, Vote};
