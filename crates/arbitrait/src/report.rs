use crate::{
    evidence::{Evidence, EvidenceItem, EvidenceTopic},
    proposal::Propose,
    vote::{Outcome, Vote},
};

use std::rc::Rc;

/// Top-level report output for a proposal that's been voted on
#[derive(Debug, Clone)]
pub struct OutcomeReport {
    /// Provided by [`Propose::description`]
    pub proposal: String,
    
    // TODO: arbitration

    /// Provided by [`BallotResult`](crate::ballot::BallotResult)'s
    /// [`Display`](std::fmt::Display) impl
    pub outcome: String,
    
    /// Report output of each [`Vote`]
    pub votes: Vec<VoteReport>,
}

/// Report output corresponding to a [`Vote`]
#[derive(Debug, Clone)]
pub struct VoteReport {
    /// Provided by [`Propose::description`]
    pub voter: String,

    /// Provided by the [`Display`](std::fmt::Display) impl of the ballot implementor
    ///
    /// E.g. by:
    /// - [`UnscoredBallot`](crate::ballot::UnscoredBallot) implementor
    /// - [`ScoredBallot`](crate::ballot::ScoredBallot) implementor
    pub choice: String,

    /// Report output of the evidence tree
    pub evidence: Vec<Rc<EvidenceReportNode>>,
}

/// Provides support for tree-like structures which may be supported by evidence types
#[derive(Debug, Clone)]
pub enum EvidenceReportNode {
    /// Represents an evidence group/category - containing [`EvidenceReportNode::Item`]'s nested one
    /// or more levels within
    ///
    /// For the crate-provided [`Evidence`] types, this will
    /// be the evidence topics
    Group {
        /// Group name, typically evidence topic name
        name: String,
        /// Group children, typically evidence items, but also supports 
        /// structures with deeper heirarchies
        children: Vec<Rc<EvidenceReportNode>>,
    },
    /// Represents an evidence leaf, with no children
    ///
    /// For the crate-provided [`Evidence`] types, this will
    /// be the evidence items
    Item {
        /// Leaf name, typically evidence item text
        name: String,
    },
}

/// Handles conversion of a voting object into its report counterpart
pub trait AsReport<R> {
    /// Creates the corresponding report type, `R`, for self
    fn as_report(&self) -> R;
}
impl<P> AsReport<OutcomeReport> for Outcome<P>
where
    P: Propose,
    Vote<P>: AsReport<VoteReport>,
{
    fn as_report(&self) -> OutcomeReport {
        OutcomeReport {
            proposal: P::description(),
            outcome: self.result().to_string(),
            votes: self.votes().iter().map(Vote::as_report).collect(),
        }
    }
}
impl<P> AsReport<VoteReport> for Vote<P>
where
    P: Propose,
    P::Evidence: AsReport<Vec<Rc<EvidenceReportNode>>>,
{
    fn as_report(&self) -> VoteReport {
        VoteReport {
            voter: self.voter_identity().to_string(),
            choice: self.ballot().to_string(),
            evidence: self.evidence().as_report(),
        }
    }
}
impl<T, I> AsReport<Vec<Rc<EvidenceReportNode>>> for Evidence<T, I>
where
    T: EvidenceTopic,
    I: EvidenceItem,
{
    fn as_report(&self) -> Vec<Rc<EvidenceReportNode>> {
        let mut out = Vec::new();
        for (topic, items) in self.inner() {
            out.push(Rc::new(EvidenceReportNode::Group {
                name: topic.to_string(),
                children: items.iter().map(|child| 
                    Rc::new(EvidenceReportNode::Item {
                        name: child.to_string()
                    })
                ).collect(),
            }));
        }
        out
    }
}

/// Handles rendering of a `*Report` for the implementing format
pub trait RenderElement<E> {
    /// Additional context that may be required for proper formatting, especially in nested
    /// structures.
    ///
    /// E.g. This could provide the current indent level
    type Context: Default; // ! TODO: merge into self
    
    /// Build the provided report element into a desired output string
    ///
    /// If the element potentially has children (`OutcomeReport`, `VoteReport`),
    /// rendering should be dispatched to the nested types respective `RenderElement` impl.
    fn render_element(element: E, context: Self::Context) -> String;
}

// !- Implementations

/// Rendering context for [`StandardFormat`]
#[derive(Debug, Copy, Clone, Default)]
pub struct StandardFormatCtx {
    /// Indentation level for the current element
    pub indent: usize,
}
impl StandardFormatCtx {
    #[must_use]
    /// Returns a new context with indentation increased by `indent`
    pub fn inc(&self, indent: usize) -> Self {
        Self {
            indent: self.indent + indent
        }
    }
}

/// Provides a simple output format for reports
#[derive(Debug, Copy, Clone)]
pub struct StandardFormat;

impl RenderElement<OutcomeReport> for StandardFormat {
    type Context = StandardFormatCtx;

    fn render_element(outcome: OutcomeReport, ctx: Self::Context) -> String {
        let mut out = Vec::new();
        let ind = " ".repeat(ctx.indent);
        out.push(format!("{ind}{}:", outcome.proposal));
        out.push(format!("{ind}  Outcome: {}", outcome.outcome));

        if outcome.votes.is_empty() {
            out.push(format!("{ind}  Votes: NONE"));
        } else {
            out.push(format!("{ind}  Votes:"));
            let ctx = ctx.inc(4);
            for vote in outcome.votes {
                out.push(Self::render_element(vote, ctx));
            }
        }
        out.join("\n")
    } 
}
impl RenderElement<VoteReport> for StandardFormat {
    type Context = StandardFormatCtx;

    fn render_element(vote: VoteReport, ctx: Self::Context) -> String {
        let mut out = Vec::new();
        let ind = " ".repeat(ctx.indent);
        out.push(format!("{ind}- {}: {}", vote.voter, vote.choice));

        if !vote.evidence.is_empty() {
            out.push(format!("{ind}  - Evidence:"));
            let ctx = ctx.inc(6);
            for evidence in vote.evidence {
                out.push(Self::render_element(evidence, ctx));
            }
        }
        out.join("\n")
    } 
}
impl RenderElement<Rc<EvidenceReportNode>> for StandardFormat {
    type Context = StandardFormatCtx;

    fn render_element(evidence: Rc<EvidenceReportNode>, ctx: Self::Context) -> String {
        let mut out = Vec::new();
        let ind = " ".repeat(ctx.indent);
        match (*evidence).clone() {
            EvidenceReportNode::Group { name, children } => {
                out.push(format!("{ind}{name}"));
                let nested_ctx = ctx.inc(2);
                for child in children {
                    out.push(Self::render_element(child, nested_ctx));
                }
            },
            EvidenceReportNode::Item { name } => {
                out.push(format!("{ind}- {name}"));
            },
        }
        out.join("\n")
    } 
}