
pub(crate) mod mandate_forbid;
pub use mandate_forbid::MandateForbidBallot;

use super::BallotResult;

use std::fmt::{Debug, Display};
use std::ops::AddAssign;

/// A ballot is 'unscored' when a candidate that receives a single vote is no more and no less
/// significant than if it were to receive multiple votes.
///
/// In other words, the significance lies in the ***presence*** of a vote for a candidate, rather
/// than the exact number of votes that candidate received.
///
/// ## Definition examples
///
/// - Rounds of an interview, where the interviewer for a round must approve to advance
///   - Here, each round's interviewer is the voter and the ballot is simply "yes"/"no"
/// - Verdict given by a court jury, where all 12 jurors must vote unanimously
///   - This one is a bit deceptive, the count of `12` would seem to imply a score
///   - However, 'unanimous' is the giveaway
///     - All jurors must have aligned votes - the precise number of jurors is irrelevant.
///   - An alternative perspective that fits the above definition
///     - Both innocent and guilty votes are present (not unanimous) -> mistrial
///     - Only innocent votes present (therefore unanimous) -> innocent
///     - Only guilty votes present (therefore unanimous) -> guilty
///
/// # Examples
///
/// ```ignore
/// todo!()
/// ```
pub trait UnscoredBallot
where
    Self: Copy
        + Debug + Display
        + PartialEq + Eq
        + Default
        + AddAssign<Self>,
{
    /// Whether or not the ballot must be arbitrated
    ///
    /// This occurs if the oucome variant result is uncertain - such as defer, abstain, skip, etc.
    ///
    /// **Note:** This should only be used once all votes are collected
    #[must_use]
    fn requires_arbitration(&self) -> bool {
        false
    }

    /// Handles settling of the resulting ballots when the outcome is uncertain
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

impl<B: UnscoredBallot> BallotResult for B {
    type Ballot = B;
    fn requires_arbitration(&self) -> bool {
        <Self as UnscoredBallot>::requires_arbitration(self)
    }
    fn arbitrate(self) -> Self {
        <Self as UnscoredBallot>::arbitrate(self)
    }
}
