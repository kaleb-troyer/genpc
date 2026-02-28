
use rand::prelude::*;

/// Represents a roll specification (number and sides of dice).
///
/// Example:
/// ```
/// let spec = DiceSpec{count: 1, sides: 20};
/// ```
#[derive(Clone, Copy)]
pub struct DiceSpec {
    pub count: u8,
    pub sides: u8,
}

/// Represents the parameters and results of a roll.
///
/// Example:
/// ```
/// let roll = Roll::new(
///     DiceSpec{count: 4, sides: 6}
/// );
/// ```
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
    pub fn total(&self) -> u32 {
        self.results.iter().map(|&r| r as u32).sum()
    }

    /// Reroll a die that rolled the specified value
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
        let reroll = Roll::new(self.spec);
        if reroll.total() > self.total() {
            reroll
        } else {
            self
        }
    }

    /// Reroll the dice and take the lower total (roll with disadvantage)
    pub fn disadv(self) -> Self {
        let reroll = Roll::new(self.spec);
        if reroll.total() < self.total() {
            reroll
        } else {
            self
        }
    }
}

// EOF
