// <comment>
// 2026-08-16
// Kaleb Troyer

use std::ops::{Add, AddAssign, Index, IndexMut};
use std::fmt;

use serde::{Deserialize, Serialize};

// ========================================
// Stat Data, Members, and Methods
// ========================================

/// Enum representing all system ability scores
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    /// Returns the enumerated value of the stat, e.g. Stat::WIS.val() = 4
    pub fn val(self) -> usize {
        self as usize
    }

    /// Returns a stat, given it's corresponding string
    pub fn from_str(stat: &str) -> Option<Stat> {
        match stat {
            "STR" => Some(Stat::STR),
            "DEX" => Some(Stat::DEX),
            "CON" => Some(Stat::CON),
            "INT" => Some(Stat::INT),
            "WIS" => Some(Stat::WIS),
            "CHA" => Some(Stat::CHA),
            _ => None,
        }
    }
}

impl fmt::Display for Stat {

    /// Returns the string value of the Stat object
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Stat::STR => "STR",
            Stat::DEX => "DEX",
            Stat::CON => "CON",
            Stat::INT => "INT",
            Stat::WIS => "WIS",
            Stat::CHA => "CHA",
        };
        write!(f, "{}", name)
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

// indexing and arithmetic operations for ability scores
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
// Abilities Implementation
// ========================================
// 

/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Skill {
    Athletics,
    Acrobatics,
    Arcana,
    SleightOfHand,
    Stealth,
    AnimalHandling,
    History,
    Investigation,
    Nature,
    Religion,
    Insight,
    Medicine,
    Deception,
    Perception,
    Survival,
    Intimidation,
    Performance,
    Persuasion,
}

impl Skill {

    /// Returns the Stat associated with the skill
    pub fn stat(self) -> Stat {
        use Skill::*;
        use Stat::*;

        match self {
            Athletics       => STR,
            Acrobatics      => DEX,
            Arcana          => INT,
            SleightOfHand   => DEX,
            Stealth         => DEX,
            AnimalHandling  => WIS,
            History         => INT,
            Investigation   => INT,
            Nature          => INT,
            Religion        => INT,
            Insight         => WIS,
            Medicine        => WIS,
            Deception       => CHA,
            Perception      => WIS,
            Survival        => WIS,
            Intimidation    => CHA,
            Performance     => CHA,
            Persuasion      => CHA,
        }
    }

}



// // ========================================
// // Ability Score Increase Implementation
// // ========================================
// // Describes the object and set of parameters that determine how ability score
// // modifiers are selected during character creation, level-up, or multiclassing.
// // Not to be confused with the Stat object, imported from system.
//
// /// Ability score increase from character creation or level-up
// #[derive(Debug, Deserialize, Serialize)]
// pub struct ASI {
//     value: u8,      // point value applied to ability score, not a modifier (usuall 1)
//     count: u8,      // number of points to allocate (usually 3)
//     maxper: u8,     // maximum number of points that may be allocated per stat
//     stats: Vec<String>, // ability scores to which points may be allocated
// }
//
// /// Ability score prerequisites for feature selection
// #[derive(Debug, Deserialize, Serialize, Default)]
// pub struct Prerequisites {
//     STR: Option<u8>,
//     DEX: Option<u8>,
//     CON: Option<u8>,
//     INT: Option<u8>,
//     WIS: Option<u8>,
//     CHA: Option<u8>,
// }
//
// impl Index<Stat> for Prerequisites {
//     type Output = Option<u8>;
//
//     fn index(&self, stat: Stat) -> &Self::Output {
//         match stat {
//             Stat::STR => &self.STR,
//             Stat::DEX => &self.DEX,
//             Stat::CON => &self.CON,
//             Stat::INT => &self.INT,
//             Stat::WIS => &self.WIS,
//             Stat::CHA => &self.CHA,
//         }
//     }
// }

// EOF
