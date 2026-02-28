
use std::ops::{Add, AddAssign, Index, IndexMut};

// ========================================
// Stat Data, Members, and Methods
// ========================================

/// Enum representing all system ability scores
#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum Stat {
    STR = 0,
    DEX = 1,
    CON = 2,
    INT = 3,
    WIS = 4,
    CHA = 5,
}

impl Stat {
    pub const COUNT: usize = 6;

    pub const ALL: [Stat; Self::COUNT] = [
        Stat::STR,
        Stat::DEX,
        Stat::CON,
        Stat::INT,
        Stat::WIS,
        Stat::CHA,
    ];

    pub fn index(self) -> usize {
        self as usize
    }
}

// ========================================
// Ability Score Data, Members, and Methods
// ========================================

/// Container for ability scores, stored as an array and indexed by Stat
#[derive(Debug, Clone, Copy)]
pub struct AbilityScores {
    values: [u8; Stat::COUNT],
}

impl AbilityScores {
    /// Default constructor with base as fill value
    pub fn new(base: u8) -> Self {
        Self { values: [base; Stat::COUNT] }
    }

    /// Constructor for building from a user-defined array
    pub fn from_array(values: [u8; Stat::COUNT]) -> Self {
        Self { values: values }
    }

    /// Constructor for cleanly defining only some stats via the diffs array
    pub fn from_diffs(diffs: &[(Stat, u8)]) -> Self {
        let mut ascore = AbilityScores::new(0);
        for &(stat, val) in diffs {
            ascore[stat] = val;
        }
        ascore
    }

    /// Return D&D ability score modifier for given stat
    pub fn modifier(&self, stat: Stat) -> i8 {
        (self[stat] as i8 - 10).div_euclid(2)
    }
}

impl Index<Stat> for AbilityScores {
    type Output = u8;

    fn index(&self, stat: Stat) -> &Self::Output {
        &self.values[stat as usize]
    }
}

impl IndexMut<Stat> for AbilityScores {
    fn index_mut(&mut self, stat: Stat) -> &mut Self::Output {
        &mut self.values[stat as usize]
    }
}

impl AddAssign for AbilityScores {
    fn add_assign(&mut self, rhs: AbilityScores) {
        for stat in Stat::ALL {
            self[stat] += rhs[stat];
        }
    }
}

impl Add for AbilityScores {
    type Output = AbilityScores;

    fn add(self, rhs: AbilityScores) -> AbilityScores {
        let mut result = self;
        result += rhs;
        result
    }
}

// ========================================
// D&D 5.5e Rules and Truisms
// ========================================

/// Experience table corresponding to levels 1-20
pub const LEVEL_EXP: [u32; 20] = [
    0,      // Level 1
    300,    // Level 2
    900,    // Level 3
    2700,   // Level 4
    6500,   // Level 5
    14000,  // Level 6
    23000,  // Level 7
    34000,  // Level 8
    48000,  // Level 9
    64000,  // Level 10
    85000,  // Level 11
    100000, // Level 12
    120000, // Level 13
    140000, // Level 14
    165000, // Level 15
    195000, // Level 16
    225000, // Level 17
    265000, // Level 18
    305000, // Level 19
    355000, // Level 20
];

// EOF
