use super::{
    evidence::Evidence,
    proposal::Propose,
};

use std::fmt::Display;
use std::hash::Hash;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// !- Vote ballots / selection pools

/// Marker trait for ballots
///
/// Currently unused
pub trait Ballot {}

/// Decision made by voter on a topic
///
/// This is the default voting layout, but you can substitute your own types that impl [`VetoBallot`]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum VetoBallot {
    /// Voter formally requests the impl
    Mandate,
    /// Voter is impartial; doesn't have have specific reason to be for or against the proposal
    Defer,
    /// Voter outright rejects the request; it is incompatible or unsound given current conditions
    Block,
}

// ! Veto impl

impl Ballot for VetoBallot {}

// !- Vote context

/// Context for the reasooning of a vote
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "
    T: Deserialize<'de> + Hash + Eq,
    I: Deserialize<'de>
")))]
pub struct VoteContext<T = String, I = String> {
    /// Short summary of logic behind decision
    //:
    ///  e.g. "Conflicting implementations"
    summary: String,

    /// Evidence backing up the claimed reason
    ///
    /// The structure here is largely arbitrary
    /// - Support for assigning multiple offenses under categories should provide enough flexibility
    ///
    /// # Layout
    ///
    /// - Keys are arbitrary topics (missing required attrs, conflicting trait, etc)
    /// - Values are a list containing 1 or more (human-friendly) descriptions for a an instance of evidence relevant to the topic
    evidence: Evidence<T, I>,
}
impl<T, I> VoteContext<T, I> {
    /// Creates new vote context with the provided summary
    ///
    /// Summaries should be a single (human-readable) sentance describing the logic behind a voter's choice.
    ///
    /// E.g:
    /// - "Missing required authorization"
    /// - "Conflicts with current configuration"
    ///
    /// Evidence of reasoning can be attached with the `add_evidence*` or `with_evidence*` methods
    #[must_use]
    pub fn new(summary: String) -> Self {
        Self {
            summary,
            evidence: Evidence::default(),
        }
    }
    /// Returns the provided summary
    #[must_use]
    pub fn summary(&self) -> &str {
        self.summary.as_str()
    }
    /// Returns the contained evidence map
    #[must_use]
    pub fn evidence(&self) -> &Evidence<T, I> {
        &self.evidence
    }
}
impl<T, I> VoteContext<T, I>
where
    T: Eq + Hash,
    I: Display,
{
    /// Adds a new piece of evidence under the given topic
    pub fn add_evidence(&mut self, topic: T, evidence: I) {
        self.evidence.add_piece(topic, evidence);
    }
    /// Adds multiple pieces of evidence under the given topic, provided using any standard collection or iter for String
    pub fn add_evidence_bag(&mut self, topic: T, bag: impl IntoIterator<Item = I>) {
        self.evidence.add_bag(topic, bag);
    }
    /// Merges in pieces of evidence spanning multiple topics
    pub fn add_evidence_locker(&mut self, locker: impl IntoIterator<Item = (T, impl IntoIterator<Item = I>)>) {
        for (topic, bag) in locker {
            self.evidence.add_bag(topic, bag);
        }
    }

    /// Builder-style alternative for [`add_piece`]
    #[must_use]
    pub fn with_evidence(mut self, topic: T, evidence: I) -> Self {
        self.add_evidence(topic, evidence);
        self
    }
    /// Builder-style alternative for [`add_bag`]
    #[must_use]
    pub fn with_evidence_bag(mut self, topic: T, bag: impl IntoIterator<Item = I>) -> Self {
        self.add_evidence_bag(topic, bag);
        self
    }
    /// Builder-style alternative for [`add_locker`]
    #[must_use]
    pub fn with_evidence_locker(mut self, locker: impl IntoIterator<Item = (T, impl IntoIterator<Item = I>)>) -> Self {
        self.add_evidence_locker(locker);
        self
    }
}

// ! Ballot wrappers

/// A vote with context supporting the voter's decision
#[derive(Debug, Clone)]
pub struct InformedVote<B = VetoBallot, T = String, I = String> {
    choice: B,
    context: VoteContext<T, I>,
}

/// A vote choice, impartial on informed or uninformed
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde",derive(Deserialize, Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        deserialize = "
            B: Deserialize<'de>,
            T: Deserialize<'de> + Hash + Eq,
            I: Deserialize<'de>
        ")
    ))
]
pub struct AggregatedVote<B = VetoBallot, T = String, I = String> {
    choice: B,
    context: Option<VoteContext<T, I>>,
}
impl From<VetoBallot> for AggregatedVote<VetoBallot> {
    fn from(choice: VetoBallot) -> Self {
        Self {
            choice,
            context: None,
        }
    }
}
impl<B, T, I> From<InformedVote<B, T, I>> for AggregatedVote<B, T, I> {
    fn from(informed: InformedVote<B, T, I>) -> Self {
        Self {
            choice: informed.choice,
            context: Some(informed.context),
        }
    }
}

// !- Vote casting

/// The primary vote-casting trait
///
/// Implement this for types that have instances which should have a vote
pub trait CastVote<P: Propose, B: Ballot = VetoBallot> {
    /// Provides an arbitrary method of identifying the voter
    ///
    /// Used in reports to provide context behind decisions, and as such, should be uniqie
    fn identity(&self) -> String;

    /// Provides the voter's selection for the provided proposal
    ///
    /// # Usage
    ///
    /// **Note:** This is immautable and does not persist or handle tracking of votes.
    /// It simply provide the voter's choice for a given propopsal
    fn cast(&self, proposal: &P) -> B;
}

// !- Vote collector

/// Collects votes from voters
pub trait CollectVote<P: Propose, B: Ballot = VetoBallot> {
    /// Handles passing a ballot to one or more voters, colllecting and storing their response and identity
    fn collect_vote(&mut self, proposal: &P, voter: impl CastVote<P, B>);

    /// Supports collecting from multiple voters via the provider iterator
    fn collect_votes(&mut self, proposal: &P, voters: impl IntoIterator<Item = impl CastVote<P, B>>) {
        for voter in voters {
            self.collect_vote(proposal, voter);
        }
    }
}

/// Wrapper around vote, attaching the `identity` `String` given by the voter
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde",derive(Deserialize, Serialize))]
pub struct CollectedVote<B> {
    /// the voter's choice
    vote: B,

    /// Identity of voter
    identity: String
}

/// Baseline execution and storage handling for vote collection
///
/// Once all votes are collected, they can (optionally) be provided to an arbitrator for settling.
///
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde",derive(Deserialize, Serialize))]
pub struct StandardCollector<P, B = VetoBallot> {
    proposal: P,
    votes: Vec<CollectedVote<B>>,
}
impl<P, B> StandardCollector<P, B> {
    #[must_use]
    /// Creates a new `StandardCollector`
    pub fn new(proposal: P) -> Self {
        Self {
            proposal,
            votes: vec![],
        }
    }
    /// Returns the current pool of votes
    #[must_use]
    pub fn votes(&self) -> &[CollectedVote<B>] {
        &self.votes
    }
}
impl<P: Propose, B: Ballot> CollectVote<P, B> for StandardCollector<P, B> {
    fn collect_vote(&mut self, proposal: &P, voter: impl CastVote<P, B>) {
        let collected = CollectedVote {
            vote: voter.cast(proposal),
            identity: voter.identity(),
        };

        self.votes.push(collected);
    }
}

// !- TODO: fail-fast toggle
//    - off fully evaluating is great for seeing the full picture
//    - on is good for planning and acting
//
// !- TODO: `Negotiation` - an iterator of proposals - always opting
//    out after the first succeeds, or retuning None if all are blocked/non-accepted
