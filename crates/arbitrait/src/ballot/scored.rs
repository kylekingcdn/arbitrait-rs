use super::BallotResult;

use std::fmt::{Debug, Display};

// !- Ballot

/// A 'scored' ballot is one where the number of votes received by a candidate has a direct influence on the outcome.
///
/// ## Definition examples
///
/// Each of the following is an example of a **scored** ballot:
///
/// - Simple elections (candidate with highest number of votes wins)
/// - Complex elections (tiers, districts, weightings, etc.)
/// - Voting on a new bill (majority rules)
/// - Vetoing some decision, where `n` number of vetos is required to bypass
///
/// # Examples
///
/// ```ignore
/// todo!()
/// ```
pub trait ScoredBallot
where
    Self: Copy
        + Debug + Display
        + PartialEq + Eq
        + Sized,
{
    /// The counted representation of this ballot
    type Tally: ScoredBallotTally<Ballot = Self>;
}

// !- Tally

/// Trait representing an active or finished [`ScoredBallot`] count
///
/// When the ballot is an enum, this type will usually be nearly identical to the [`ScoredBallot`] type, with the following changes:
///
/// - Any `Abstain` or *ignore* variants are removed
/// - All remaining variants receive an unnamed `usize` field representing a candidate's vote count
pub trait ScoredBallotTally
where
    Self: BallotResult + Sized,
    Self::Ballot: ScoredBallot,
{
}
