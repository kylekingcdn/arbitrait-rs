use crate::{
    ballot::BallotResult,
    proposal::{
        ProposalBallot, ProposalBallotResult,
        ProposalEvidence, Propose,
    },
    report::{AsReport, OutcomeReport, RenderElement},
};

use std::marker::PhantomData;

// !- Voter identity

/// Provides voter identity
///
/// This is separated from [`CastVote`] to allow for reuse
pub trait IdentifyVoter {
    /// Provides an arbitrary method of identifying the voter
    ///
    /// Used in reports to provide context behind decisions, and as such, should be unique
    #[must_use]
    fn voter_identity(&self) -> String;
}

// !- Vote casting

/// The primary vote-casting trait
///
/// Implement this for types that have instances which should have a vote
pub trait CastVote<P>
where
    Self: IdentifyVoter,
    P: Propose,
{
    /// Provides the voter's selection for the given proposal
    ///
    /// # Usage    ///
    #[must_use]
    fn cast_ballot(&self/*, proposal: P*/) -> Vote<P>;

    /// Helper fn used to build votes
    ///
    /// Should not need to be re-written by implementors
    #[must_use]
    fn build_vote(&self, ballot: ProposalBallot<P>) -> Vote<P> {
        Vote::new(ballot, self.voter_identity())
    }
}

// !- Vote

/// A composed vote that has been collected from a voter
// #[derive(Debug, Clone)]
// !- FIXME: impl Debug, Clone
pub struct Vote<P: Propose> {
    _proposal: PhantomData<P>,

    ballot: ProposalBallot<P>,
    evidence: ProposalEvidence<P>,

    voter_identity: String,
}
impl<P: Propose> Vote<P> {
    /// Crteates a new `Vote`
    #[must_use]
    pub fn new(ballot: ProposalBallot<P>, identity: String) -> Self {
        Self {
            _proposal: PhantomData,
            ballot,
            evidence: <P as Propose>::Evidence::default(),
            voter_identity: identity,
        }
    }
    /// Provides evidence of the voter's decision
    #[must_use]
    pub fn with_evidence(mut self, evidence: <P as Propose>::Evidence) -> Self {
        self.evidence = evidence;
        self
    }

    /// Returns the filled ballot
    pub fn ballot(&self) -> &ProposalBallot<P> {
        &self.ballot
    }
    /// Returns the evidence supporting the voter's decision (which may be empty)
    pub fn evidence(&self) -> &ProposalEvidence<P> {
        &self.evidence
    }
    /// Returns the self-provided identity of the voter
    pub fn voter_identity(&self) -> &str {
        self.voter_identity.as_str()
    }
}

// !- Vote outcome

/// Outcome of a vote once all `Vote`s have been collected
pub struct Outcome<P: Propose> {
    votes: Vec<Vote<P>>,
    result_unsettled: Option<ProposalBallotResult<P>>,
    result: ProposalBallotResult<P>,
}

impl<P: Propose> Outcome<P> {
    /// Returns the result of the vote
    ///
    /// This represents the post-arbitration result (if arbitration was required).
    ///
    /// For the collected result prior to arbitration, see
    /// [`arbitrated_from`](Self::arbitrated_from)
    pub fn result(&self) -> ProposalBallotResult<P> {
        self.result
    }

    /// Wherher or not the votes were arbitrated
    pub fn was_arbitrated(&self) -> bool {
        self.result_unsettled.is_some()
    }
    /// If the votes were arbitrated, returns the pre-arbitration ballot result
    ///
    /// Otherwise, if the votes **were not** arbitrated, returns `None`
    pub fn arbitrated_from(&self) -> Option<ProposalBallotResult<P>> {
        self.result_unsettled
    }

    /// The contained list of votes
    pub fn votes(&self) -> &[Vote<P>] {
        &self.votes
    }

    /// Builds a new `Outcome` from an iterator of [`Vote`]s
    ///
    /// Note: This struct implements [`FromIterator`], so if you already have an iterator
    /// chain containing `Vote`s going, you can do the following:
    ///
    /// ```ignore
    /// let outcome: Outcome<MyProposal> = x
    ///     .into_iter()
    /// //  ...
    ///    .collect();
    /// ```
    pub fn from_votes(votes: impl IntoIterator<Item = Vote<P>>) -> Self {
        let mut result = ProposalBallotResult::<P>::default();
        let mut votes_out= Vec::new();

        for vote in votes {
            result += vote.ballot;
            votes_out.push(vote);
        }

        let (result_unsettled, result) = if result.requires_arbitration() {
            (None, result)
        } else {
            (Some(result), result.arbitrate())
        };

        Self {
            votes: votes_out,
            result_unsettled,
            result,
        }
    }

    /// Converts [`Outcome`] into the contained [`Vec<Vote<P>>`].
    ///
    /// # Converting back to [`Outcome`]
    ///
    /// The votes can be turned back into an `Outcome` using [`Outcome::from_votes`].
    ///
    /// Note that doing so will re-count the votes. If the votes have been modified, this may
    /// result in a different outcome.
    ///
    /// Therefore, if you stored the outcome elsewhere, you should
    /// ensure it remains up-to-date as well.
    pub fn into_votes(self) -> Vec<Vote<P>> {
        self.votes
    }
}

// Add inline as_report() method
impl<P> Outcome<P>
where
    Self: AsReport<OutcomeReport>,
    P: Propose,
{
    /// Provides the skeleton used to create formatted reports
    pub fn as_report(&self) -> OutcomeReport {
        <Self as AsReport<_>>::as_report(self)
    }
    /// Provides the skeleton used to create formatted reports
    pub fn as_rendered_report<F>(&self, _format: F) -> String
    where
        F: RenderElement<OutcomeReport>
    {
        let context = <F as RenderElement<_>>::Context::default();
        F::render_element(self.as_report(), context)
    }
}
impl<P: Propose> FromIterator<Vote<P>> for Outcome<P> {
    fn from_iter<T: IntoIterator<Item = Vote<P>>>(iter: T) -> Self {
        Self::from_votes(iter)
    }
}
impl<P: Propose> From<Vec<Vote<P>>> for Outcome<P> {
    fn from(value: Vec<Vote<P>>) -> Self {
        value.into_iter().collect()
    }
}

// !- Vote collection

/// Utility trait for extracting ballots from Voters and returning an [`Outcome`]
///
/// See [`IterVoteCollector`] for an example impl.
pub trait CollectVotes<T, P: Propose> {
    /// Where `T` is some type containing 1 or more voters, this fn iterates through
    /// the voters, calling the [`CastVote::cast_ballot`] trait fn for each voter.
    ///
    /// Votes can be converted to an [`Outcome`] by either using [`Outcome::from_votes`]
    /// or by using `collect()` from the provided [`FromIterator`] impl.
    ///
    /// Implementors should prefer to not consume `T`, where possible.
    ///
    /// See [`IterVoteCollector`] for an example impl.
    fn collect_votes(voters: T) -> Outcome<P>;
}

/// Collects votes for an iterator of voters
#[derive(Debug, Copy, Clone)]
pub struct IterVoteCollector;
impl<'a, T, I, P> CollectVotes<T, P> for IterVoteCollector
where
    T: IntoIterator<Item = &'a I>,
    I: CastVote<P> + 'a,
    P: Propose,
{
    fn collect_votes(voters: T) -> Outcome<P> {
        voters.into_iter().map(I::cast_ballot).collect()
    }
}
