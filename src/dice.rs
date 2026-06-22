// <comment>
// 2026-06-21
// Kaleb Troyer

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use crate::character::{Character};
use crate::load::{DynMod};

/// Represents a roll specification (number and sides of dice).
///
/// Example:
/// ```
/// let spec = DiceSpec{count: 1, sides: 20, bonus: 0}; // 1d20
/// ```
#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
pub struct DiceSpec {
    pub count: u8,
    pub sides: u8,
    pub bonus: i8,
}

/// Represents the parameters and results of a roll.
///
/// Example:
/// ```
/// let mut roll = Roll::new(
///     DiceSpec{ count: 4, sides: 6 }
/// ).adv();
/// roll.reroll(1)
/// let min = roll.min();
/// let max = roll.max();
/// let tot = roll.total();
/// ```
#[derive(Debug)]
pub struct Roll {
    spec: DiceSpec,
    results: Vec<u8>,
}

// Roll constructor
impl Roll {
    pub fn new(spec: DiceSpec) -> Self {
        let results = Self::roll(spec.count, spec.sides);
        Self { spec, results }
    }
}

// Roll methods
impl Roll {

    /// Rolls a number of dice of the specified size, returning a vector
    fn roll(count: u8, sides: u8) -> Vec<u8> {

        let mut rng = rand::rng();

        (0..count as usize)
            .map(|_| rng.random_range(1..=sides))
            .collect()
    }

    /// Returns a vector of the dice rolls
    pub fn results(&self) -> &[u8] {
        &self.results
    }

    /// Returns the sum total of dice rolled
    pub fn total(&self) -> i32 {
        let sum: i32 = self.results.iter().map(|&r| r as i32).sum();
        sum + self.spec.bonus as i32
    }

    /// Reroll one die that rolled the specified value
    pub fn reroll(&mut self, side: u8) {
        for val in &mut self.results {
            if *val == side {
                *val = Self::roll(1, self.spec.sides)[0];
                break;
            }
        }
    }

    /// Return the minimum value rolled
    pub fn min(&self) -> u8 {
        self.results.iter().copied().min().unwrap()
    }

    /// Return the maximum value rolled
    pub fn max(&self) -> u8 {
        self.results.iter().copied().max().unwrap()
    }

    /// Reroll the dice and take the higher total (roll with advantage)
    pub fn adv(self) -> Self {
        let alt = Roll::new(self.spec);
        if alt.total() > self.total() {
            alt
        } else {
            self
        }
    }

    /// Reroll the dice and take the lower total (roll with disadvantage)
    pub fn disadv(self) -> Self {
        let alt = Roll::new(self.spec);
        if alt.total() < self.total() {
            alt
        } else {
            self
        }
    }
}

/// A dice specification that may reference a character attribute for its bonus,
/// resolved into a [`DiceSpec`] at runtime.
///
/// Example:
/// ```
/// let dref = DiceRef { ... };
/// let roll = Roll::new(
///     dref.getspec(&MyCharacter)
/// ).adv();
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DiceRef {
    pub count: u8,
    pub sides: u8,
    pub bonus: DynMod, // See load.rs
}

// DiceRef methods
impl DiceRef {

    /// Unpacks the DiceRef and returns a fixed DiceSpec
    fn getspec(&self, char: &Character) -> DiceSpec {
        let bval: i8 = match self.bonus {
            DynMod::Flat(n) => n,
            DynMod::Reference(ref s) => 0 // resolve later
        };

        DiceSpec { count: self.count, sides: self.sides, bonus: bval }
    }
}



// EOF
