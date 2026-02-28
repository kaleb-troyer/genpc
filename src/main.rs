
mod system;
mod class;
mod race;
mod char;
mod dice;

use system::{Stat::*, AbilityScores};
use class::{Class, CLASSES};
use race::{Race, RACES};
use char::{Character};
use dice::{Roll, DiceSpec};
use std::collections::HashMap;

fn main() {

    let classes: HashMap<&str, &Class> = CLASSES.iter().map(
        |c| (c.name, c)
    ).collect();

    let races: HashMap<&str, &Race> = RACES.iter().map(
        |r| (r.name, r)
    ).collect();

    // this is a stat roller per 4d6 & drop the lowest rules
    let statspec = DiceSpec { count: 4, sides: 6 };
    let rolls: [u8; 6] = loop {
        let rolls: [u8; 6] = std::array::from_fn(|_| {
            let r = Roll::new(statspec);
            (r.total() - r.min() as u32) as u8
        });

        let score: u32 = rolls
            .iter().map(|&r| r as u32).sum();

        if score >= 72 {
            break rolls;
        }
    };

    let statrolls = AbilityScores::from_array(rolls);
    println!("{:?}", statrolls);

    let mut char1 = Character::new(
        "Perry Paladin",
        races.get("Halfling").expect("Race not found."),
        classes.get("Paladin").expect("Class not found."),
        statrolls,
    );

    char1.add_lvl();

    println!("{:#?}", char1);
    println!("DEX mod = {}", char1.ability_scores.modifier(DEX));

}

// EOF
