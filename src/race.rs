
use crate::system::{Stat::*, AbilityScores};
use std::sync::LazyLock;

// Race Structure and Collection
#[derive(Debug)]
pub struct Race {
    pub name: &'static str,
    pub asi: AbilityScores,
}

pub static RACES: LazyLock<Vec<Race>> = LazyLock::new(|| vec![
    Race {
        name: "Human",
        asi: AbilityScores::new(1),
    },
    Race {
        name: "Halfling",
        asi: AbilityScores::from_diffs(&[(DEX, 2), (CON, 1)]),
    },
]);

// EOF
