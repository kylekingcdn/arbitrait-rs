use crate::vote::VetoBallot;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// !- Record

/// A set of votes
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Record<B = VetoBallot> {
    votes: Vec<B>,
}

// !- Report

/// Contains the completed vote information
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Report<P, B = VetoBallot> {
    proposal: P,
    record: Record<B>,
    arbitrator_id: String,
}
