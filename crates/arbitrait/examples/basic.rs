use arbitrait::{
    ballot::unscored::MandateForbidBallot,
    evidence::BoxDynEvidence,
    proposal::Propose,
    report::{StandardFormat},
    vote::{CastVote, CollectVotes, IdentifyVoter, Outcome, Vote, IterVoteCollector},
};
use std::fmt::Debug;

struct Field {
    name: String,
    copy: bool,
}

// -- Arbitrait impl

impl IdentifyVoter for Field {
    fn voter_identity(&self) -> String {
        self.name.clone()
    }
}
/// Proposal on whether or not to derive the `Copy` trait
#[derive(Debug, Copy, Clone)]
pub struct DeriveCopy;

// Define proposal parameters
impl Propose for DeriveCopy {
    type CountedBallot = MandateForbidBallot;
    type Evidence = BoxDynEvidence<String>;

    fn description() -> String {
        "Derive `Copy`".to_string()
    }
}
// Add support for voting on our proposal type to our Field struct
impl CastVote<DeriveCopy> for Field {
    fn cast_ballot(&self) -> Vote<DeriveCopy> {
        if self.copy {
            self.build_vote(MandateForbidBallot::Mandate)
        } else {
             // Optionally add context behind our decision
            let evidence = DeriveCopy::new_evidence().with_piece(
                "Type support".to_string(), Box::new("Missing copy")
            );
            self.build_vote(MandateForbidBallot::Forbid)
                .with_evidence(evidence)
        }
    }
}

fn main() {
    // define our fields (which have support for voting on our proposal)
    let fields = [
        Field { name: "field_1".to_string(), copy: true },
        Field { name: "field_2".to_string(), copy: false },
        Field { name: "field_3".to_string(), copy: true },
    ];

    // execute the vote on our `DeriveCopy` proposal
    // - this handles collection of votes and resolves the outcome
    // - all intermediate types (ballot, evidence, etc) are automatically implied by our
    //   proposal definition
    let outcome: Outcome<DeriveCopy> = IterVoteCollector::collect_votes(&fields);
    println!("result: {:#?}", outcome.result());

    // renders the vote (outcome, individual votes, evidence) in a human readable format
    let report = outcome.as_rendered_report(StandardFormat);
    println!("\nStructured report:\n");
    println!("{report}");
    
}
