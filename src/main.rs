
// internal modules
mod background;
mod system;
mod class;
mod race;
mod char;
mod feat;
mod dice;
mod load;

// internal crates
use system::{Stat, Stat::*, AbilityScores};
use class::{Class, Subclass};
use race::{Race};
use char::{Character};
use dice::{Roll, DiceSpec};

// external crates
use std::collections::HashMap;
use dialoguer::{Select, console::Term};


use std::io;
use std::fs;
use std::path::Path;
use serde_json;
use background::{Background};
use feat::{Feat};

use load::{Database};


// just read a background object
fn read_background(path: &str) -> Result<Background, Box<dyn std::error::Error>> {
    let background: Background = serde_json::from_str(
        &fs::read_to_string(path)?
    )?;
    Ok(background)
}

// trying loading each object one by one
fn read_class(path: &str) -> Result<Class, Box<dyn std::error::Error>> {
    let class: Class = serde_json::from_str(
        &fs::read_to_string(path)?
    )?;
    Ok(class)
}

fn read_sc(path: &str) -> Result<Subclass, Box<dyn std::error::Error>> {
    let sc: Subclass = serde_json::from_str(
        &fs::read_to_string(path)?
    )?;
    Ok(sc)
}

fn read_feat(path: &str) -> Result<Feat, Box<dyn std::error::Error>> {
    let feat: Feat = serde_json::from_str(
        &fs::read_to_string(path)?
    )?;
    Ok(feat)
}

fn read_race(path: &str) -> Result<Race, Box<dyn std::error::Error>> {
    let race: Race = serde_json::from_str(
        &fs::read_to_string(path)?
    )?;
    Ok(race)
}

fn get_data_test(dir: &Path, db: &mut Database) -> io::Result<()> {

    if dir.is_dir() {

        for entry in fs::read_dir(dir)? {

            let entry = entry?;

            if Path::new(&entry.path()).is_dir() {

                get_data_test(Path::new(&entry.path()), db)?;

            } else {

                let entry = fs::read_to_string(&Path::new(&entry.path()))?;
                let etype: serde_json::Value = serde_json::from_str(
                    &entry
                )?;

                match &etype["category"].as_str() {
                    Some("background") => {
                        println!("{:#?} -> {:#?}", &etype["name"], &etype["category"]);
                        db.backgrounds.push(serde_json::from_str(&entry)?);
                    }
                    Some("feature") => {
                        println!("{:#?} -> {:#?}", &etype["name"], &etype["category"]);

                    }
                    Some("class") => {
                        println!("{:#?} -> {:#?}", &etype["name"], &etype["category"]);
                        db.classes.push(serde_json::from_str(&entry)?);
                    }
                    Some("feat") => {
                        println!("{:#?} -> {:#?}", &etype["name"], &etype["category"]);
                        db.feats.push(serde_json::from_str(&entry)?);
                    }
                    Some("race") => {
                        println!("{:#?} -> {:#?}", &etype["name"], &etype["category"]);
                        db.races.push(serde_json::from_str(&entry)?);
                    }
                    _ => println!("unknown :("),
                };

            }

        }

    }

    Ok(())

}





fn main() -> io::Result<()> {

    // this is a stat roller per 4d6 & drop the lowest rules
    let statspec = DiceSpec { count: 4, sides: 6, bonus: 0 };
    let rolls: [u8; 6] = loop {
        let rolls: [u8; 6] = std::array::from_fn(|_| {
            let r = Roll::new(statspec);
            (r.total() - r.min() as i32) as u8
        });

        let score: u32 = rolls
            .iter().map(|&r| r as u32).sum();

        if score >= 72 {
            break rolls;
        }
    };

    let statrolls = AbilityScores::from_array(rolls);
    println!("\n{:?}\n", rolls);

    let bg = read_background("data/backgrounds/farmer.json")
        .expect("Failed to load.");

    let class = read_class("data/classes/fighter/fighter.json")
        .expect("Failed to load.");

    let feat = read_feat("data/feats/alert.json")
        .expect("Failed to load.");

    let sc = read_sc("data/classes/fighter/battle_master.json")
        .expect("Failed to load.");

    let race = read_race("data/races/human.json")
        .expect("Failed to load.");

    // println!("{:#?}", bg);
    // println!("{:#?}", class);
    // println!("{:#?}", sc);
    // println!("{:#?}", feat);
    // println!("{:#?}", race);


    // try to load and store all json items

    let mut db = Database::new();
    get_data_test(Path::new("./data"), &mut db);

    println!("{:#?}", db);

    std::process::exit(0);

    // TUI begins here:

    let term = Term::stdout();
    term.clear_screen().unwrap();

    let mut u_menu;
    let mut u_stat;
    let mut s_menu;
    let mut s_stat;
    let mut s_tmsg = "Select an option";
    let mut v_menu;

    loop {

        term.clear_screen().unwrap();

        v_menu = vec!["New", "Load", "Exit"];

        u_menu = Select::new()
            .with_prompt(format!("Welcome to genpc!\n{}", s_tmsg))
            .items(&v_menu)
            .default(0)
            .interact()
            .unwrap();
        s_menu = v_menu[u_menu];

        if s_menu == "New" {

            loop {

                term.clear_screen().unwrap();

                v_menu = vec!["Race", "Background", "Class", "Ability Scores", "Finish", "Cancel"];

                let mut c_stat = AbilityScores::new(10);




                u_menu = Select::new()
                    .with_prompt(format!("Welcome to genpc!\n{}", s_tmsg))
                    .items(&v_menu)
                    .default(0)
                    .interact()
                    .unwrap();
                s_menu = v_menu[u_menu];

                if s_menu == "Ability Scores" {

                    let mut v_stat = rolls.to_vec();

                    for stat in Stat::ALL {

                        term.clear_screen().unwrap();

                        println!("Welcome to genpc!\n{:?}\n", c_stat);

                        u_stat = Select::new()
                            .with_prompt(format!("Choose your {} stat", stat))
                            .items(&v_stat)
                            .default(0)
                            .interact()
                            .unwrap();
                        s_stat = v_stat.remove(u_stat);

                        c_stat[stat] = s_stat;

                    }

                } else if s_menu == "Finish" || s_menu == "Cancel" {
                    break;
                }
            }
        } else if s_menu == "Load" {
            s_tmsg = "Load not yet implemented";
        } else if s_menu == "Exit" {
            break;
        }
    }

    term.clear_screen().unwrap();

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
