use super::UnscoredBallot;

use std::cmp::max;
use std::fmt;
use std::ops::AddAssign;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Provides an interface for a proposal that represents inclusion of some provision/feature, etc.
///
/// - This is a good choice when the default policy would be to not include the proposed feature
// ///   - Accomplished by pairing with [`RestrictiveArbitrator`]
// /// - Can also be used when inclusion is tbe default, noting:
// ///   - Accomplished by pairing with [`PermissiveArbitrator`]
// ///   - `Mandate` and `Abstain` are functionally the same in this case, as unanimous `Abstain`
// ///     arbitrates to `Mandate`
// ///     - It is still good practice to follow the conventions listed by each variant,
// ///       as it allows for re-use between arbitrators
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum MandateForbidBallot {
    /// Voter formally requests the proposed action, if possible (not forbidden)
    Mandate = 1, // explicitly set discriminants for derived `PartialOrd`

    /// Voter is impartial; it is unconcerned with or does not have specific reason to be for or against the proposal
    #[default]
    Abstain = 0,

    /// Voter outright rejects the request; it is incompatible or unsound given current conditions
    Forbid = 2,
}

impl UnscoredBallot for MandateForbidBallot {
    fn arbitrate(self) -> Self {
         if matches!(&self, Self::Abstain) {
             Self::Forbid
         } else {
             self
         }
    }
    fn requires_arbitration(&self) -> bool {
        // occurs if there are no voters or all voters cast Abstain
        matches!(self, Self::Abstain)
    }
}

impl fmt::Display for MandateForbidBallot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Mandate => "Mandate",
            Self::Abstain => "Abstain",
            Self::Forbid => "Forbid",
        };
        write!(f, "{name}")
    }
}

impl AddAssign<Self> for MandateForbidBallot {
    fn add_assign(&mut self, rhs: Self) {
            *self = max(*self, rhs);
    }
}
