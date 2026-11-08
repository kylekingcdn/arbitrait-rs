use crate::{
    arbitrate::RestrictiveArbitrator,
    vote::VetoBallot,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Provides support for using an arbitrary type as a proposal
pub trait Propose {
    /// Single sentance description of the decision to be evaluated
    fn description(&self) -> &str;
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde",derive(Deserialize, Serialize))]
/// Proposal description as well as the definition of the corresponding arbitrator and ballot type,
pub struct VetoProposal<B = VetoBallot, A = RestrictiveArbitrator> {
    ballot: B,
    arbitrator: A,

    description: String,
}

impl<B, A> Propose for VetoProposal<B, A> {
    fn description(&self) -> &str {
        &self.description
    }
}
