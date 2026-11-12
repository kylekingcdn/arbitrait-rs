/// Scored ballot traits & provided impls
pub mod scored;
pub use scored::{ScoredBallot, ScoredBallotTally};

/// Unscored ballot traits & provided impls
pub mod unscored;
pub use unscored::UnscoredBallot;

use std::fmt::{Debug, Display};
use std::ops::AddAssign;

/// Represents counted ballots
///
/// For unscord ballots, this is automatically implemented on the ballot itself
pub trait BallotResult
where
    Self: Copy
        + Debug + Display
        + PartialEq + Eq
        + Default
        + AddAssign<Self::Ballot>
{
    /// The source ballot type
    type Ballot: Debug + Display + PartialEq + Eq + Copy;

    /// Whether or not the vote must be arbitrated
    ///
    /// This occurs if the oucome is uncertain - such as abstain, a tie, etc.
    ///
    /// **Note:** This should only be used once all votes are collected
    #[must_use]
    fn requires_arbitration(&self) -> bool {
        false
    }

    /// Handles settling of the resulting votes when the outcome is uncertain
    ///
    /// Ballot choices that return false from
    /// [`requires_arbitration()`](Self::requires_arbitration) should always return `self` here.
    ///
    /// **Note:** This should only be used once all votes are collected
    #[must_use]
    fn arbitrate(self) -> Self {
        self
    }
}
