use arbitrait::{
    ballot::unscored::MandateForbidBallot,
    evidence::BoxDynEvidence,
    proposal::Propose,
    vote::{CastVote, CollectVotes, IdentifyVoter, Outcome, Vote, IterVoteCollector},
};
use std::fmt::Debug;

// #[derive(Debug, Default)]
// struct FooStruct;
// #[derive(Debug, Default)]
// struct BarStruct;
//
// #[derive(Debug, Copy, Clone)]
// pub struct ImplementFrom<T: Debug>(PhantomData<T>);
//
// impl<T: Debug + Default + Copy> Propose for ImplementFrom<T> {
//     type CountedBallot = MandateForbidBallot;
//     type Evidence = BoxDynEvidence<String>;
//
//     fn description(&self) -> String {
//         format!("Implement From<{:#?}>", <T as Default>::default())
//     }
// }

#[derive(Debug, Copy, Clone)]
pub struct DeriveCopy;

impl Propose for DeriveCopy {
    type CountedBallot = MandateForbidBallot;
    type Evidence = BoxDynEvidence<String>;

    fn description(&self) -> String {
        "Derive `Copy`".to_string()
    }
}

struct Field {
    name: String,
    copy: bool,
}
impl IdentifyVoter for Field {
    fn voter_identity(&self) -> String {
        self.name.clone()
    }
}
impl CastVote<DeriveCopy> for Field {
    fn cast_ballot(&self) -> Vote<DeriveCopy> {
        let mut evidence = DeriveCopy::new_evidence();

        if self.copy {
            self.build_vote(MandateForbidBallot::Mandate)
        } else {
            evidence.add_piece("Type support".to_string(), Box::new("Missing copy"));
            self.build_vote(MandateForbidBallot::Forbid)
                .with_evidence(evidence)
        }
    }
}

fn main() {
    let fields = [
        Field { name: "field_1".to_string(), copy: true },
        Field { name: "field_2".to_string(), copy: false },
        Field { name: "field_3".to_string(), copy: true },
        Field { name: "field_4".to_string(), copy: true },
    ];

    let res: Outcome<DeriveCopy> = IterVoteCollector::collect_votes(&fields);
    println!("result: {:#?}", res.result());

    let res: Outcome<DeriveCopy> = IterVoteCollector::collect_votes(&fields);
    println!("result: {:#?}", res.result());
}
