use crate::{
    proposal::Propose,
    vote::VetoBallot,
};

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// !- Traits

/// Strategies for resolving voter-wide deferral
///
/// # With great power...
///
/// This trait does not have "guard-rails" in place to prevent an arbiter from settling results that don't call for deferal.
///
/// Tbis is by choice to allow for greater flexibility in decision making strategies
/// (e.g. promoting a "judge" or "final-say" mid-vote).
///
/// It's the arbiter's responsibility to ensure it's settling results within it's scope.
pub trait Arbitrate<P: Propose, B = VetoBallot>: fmt::Display {
    /// Takes in a ballot result and settles the verdict.
    fn arbitrate(&self, proposal: P, ballot: B) -> B;
}

// !- Implementations

// !- Permissive impl

/// Settles voter-wide 'defer' by always siding to [`VetoBallot::Mandate`]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde",derive(Deserialize, Serialize))]
pub struct PermissiveArbitrator;

impl fmt::Display for PermissiveArbitrator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Permissive")
    }
}

impl<P: Propose> Arbitrate<P, VetoBallot> for PermissiveArbitrator {
    fn arbitrate(&self, _proposal: P, ballot: VetoBallot) -> VetoBallot {
        match ballot {
            VetoBallot::Defer => VetoBallot::Mandate,
            other => other
        }
    }
}

// ! Restrictive impl

/// Settles voter-wide 'defer' by always siding to [`VetoBallot::Block`]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct RestrictiveArbitrator;

impl fmt::Display for RestrictiveArbitrator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Restrictive")
    }
}

impl<P: Propose> Arbitrate<P, VetoBallot> for RestrictiveArbitrator {
    fn arbitrate(&self, _p: P, ballot: VetoBallot) -> VetoBallot {
        match ballot {
            VetoBallot::Defer => VetoBallot::Block,
            other => other,
        }
    }
}
