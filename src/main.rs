
// internal modules
mod background;
mod system;
mod class;
mod race;
mod char;
mod dice;
mod load;

// internal crates
use system::{Stat, Stat::*, AbilityScores};
use class::{Class};
use race::{Race};
use char::{Character};
use dice::{Roll, DiceSpec};

// external crates
use std::collections::HashMap;
use dialoguer::Select;

fn main() {

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
    println!("\n{:?}\n", rolls);

    let mut u_menu;
    let mut s_menu;
    let mut u_stat;
    let mut s_stat;
    let v_menu = vec!["New", "Load", "Exit"];
    loop {

        u_menu = Select::new()
            .with_prompt("Welcome to genpc!\n")
            .items(&v_menu)
            .default(0)
            .interact()
            .unwrap();
        s_menu = v_menu[u_menu];

        if s_menu == "New" {

            for stat in Stat::ALL {

                u_stat = Select::new()
                    .with_prompt(format!("Choose your {} stat:", stat))
                    .items(&rolls)
                    .default(0)
                    .interact()
                    .unwrap();
                s_stat = rolls[u_stat];

                println!("You chose: {}", s_stat);

            }
        }

        if (s_menu == "Exit") {
            break;
        }
    }
}


// use dialoguer::Select;
//
// let options = vec!["Fighter", "Wizard", "Rogue"];
//
// let selection = Select::new()
//     .with_prompt("Choose a class")
//     .items(&options)
//     .interact()
//     .unwrap();
//
// println!("You chose: {}", options[selection]);


// EOF
