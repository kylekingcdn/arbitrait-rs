use crate::ballot::BallotResult;

// !- Proposal trait

/// Provides support for using an arbitrary type as a proposal
///
/// This should typically be used on an empty struct that derives, at a minimum:
///
/// - `Debug`
/// - `Clone`
/// - `Copy`
///
/// # Examples
///
/// ```ignore
/// todo!()
/// ```
pub trait Propose: Copy {
    /// The ballot type containing the counts for choices provided to a voter
    type CountedBallot: BallotResult;

    /// The type of storage used for evidence
    ///
    /// See the [`evidence`](crate::evidence) module for more information
    type Evidence: Default;

    /// Single sentence description of the decision to be evaluated
    fn description(&self) -> String;

    /// Builds new evidence storage
    #[must_use]
    fn new_evidence() -> Self::Evidence {
        Self::Evidence::default()
    }
}

// !- Proposal type aliases

/// Forwards to the [`Evidence`](crate::evidence::Evidence) type on the [`Propose`] trait
pub type ProposalEvidence<P> = <P as Propose>::Evidence;

/// Forwards to the [`BallotResult`] type on the [`Propose`] trait
pub type ProposalBallotResult<P> = <P as Propose>::CountedBallot;

/// Forwards to the [`Propose`] trait's [`Ballot`](crate::ballot::BallotResult::Ballot) type from the underlying [`BallotResult`] trait
pub type ProposalBallot<P> = <ProposalBallotResult<P> as BallotResult>::Ballot;
