
use crate::system::{AbilityScores, LEVEL_EXP};
use crate::class::{Class};
use crate::race::{Race};

// Core Character Structure
#[derive(Debug)]
pub struct Character {
    name: String,
    race: &'static Race,
    class: &'static Class,
    pub ability_scores: AbilityScores,
    level: u8,
    experience: u32,
    prof_bonus: u8,
}

// Character Constructor
impl Character {

    // pub fn new(
    //     name: impl Into<String>,
    //     race: &'static Race,
    //     class: &'static Class,
    //     ability_scores: AbilityScores,
    // ) -> Self {
    //     let ascores = ability_scores + race.asi;
    //
    //     let level = 1;
    //     let experience = 0;
    //     let prof_bonus = 2;
    //
    //     Self {
    //         name: name.into(),
    //         ability_scores: ascores,
    //         race,
    //         class,
    //         level,
    //         experience,
    //         prof_bonus,
    //     }
    // }
}

// Character Reading and Advancement
impl Character {
    /// Adds the specified amount of experience to the character and updates their level
    pub fn add_exp(&mut self, exp: u32) {
        self.experience += exp;

        let level: u8 = LEVEL_EXP
            .iter()         // convert thresholds into an interable
            .enumerate()    // associate each threshold w/ its level via enumeration
            .rev()          // reverse the order
            .find(|&(_, &th)| self.experience >= th) // find the first threshold beneath current exp
            .map(|(i, _)| (i + 1) as u8) // adjust level for 0-indexing
            .unwrap_or(1);  // unpacks the Option object or defaults to level 1

        if self.level != level {
            self.level = level;
            self.advancements();
        }
    }

    /// Adds one level to the character and updates their experience, accordingly
    pub fn add_lvl(&mut self) {
        self.level = (self.level + 1).min(LEVEL_EXP.len() as u8);

        self.experience = LEVEL_EXP
            .iter()
            .enumerate()
            .find(|(i, _)| self.level == (i + 1) as u8)
            .map(|(_, &th)| th as u32)
            .unwrap_or(0);

        self.advancements();
    }

    /// Compute proficiency bonus from character level
    fn calc_prof_bonus(&self) -> u8 {
        2 + ((self.level - 1) / 4)
    }

    /// Initiate all character / race / class advancements on level-up
    fn advancements(&mut self) {
        self.prof_bonus = self.calc_prof_bonus();
        // more to come
    }
}

// EOF
