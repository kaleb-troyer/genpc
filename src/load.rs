
use std::ops::{Add, AddAssign, Index, IndexMut};
use serde::{Deserialize, Serialize};
use crate::system::{Currency, Stat};

// ========================================
// Primary Substructures from Data
// ========================================
// Describes the top-level substructures from classes, feats, backgrounds, etc.
// from .json data. The Option<> type indicates that the field doesn't need to
// be present for the structure to instantiate from the data.

/// Benefits object and contents as imported from /data
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Benefits {
    ability_scores: Option<ASI>,        // ability score modifier selection, see below
    feats: Option<SelectionPool>,       // feats selection, see below
    skill_profs: Option<SelectionPool>, // skill proficiency selection
    tool_profs: Option<SelectionPool>,  // tool proficiency selection
    sublcass: Option<SelectionPool>,    // subclass selection
    features: Option<Vec<Feature>>,     // list of primary source features, see below
    effects: Option<Vec<Effect>>,       // list of primary source effects, see below
}

/// Collection of available equipment load-outs from backgrounds
#[derive(Debug, Deserialize, Serialize)]
pub struct Equipment {
    A: EquipmentOps,
    B: EquipmentOps,
}

/// Equipment load-out contents
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct EquipmentOps {
    items: Option<Vec<String>>,
    currency: Option<Currency>, // TODO import from system
    choose: Option<Choice>
}

// ========================================
// Ability Score Increase Implementation
// ========================================
// Describes the object and set of parameters that determine how ability score
// modifiers are selected during character creation, level-up, or multiclassing.

#[derive(Debug, Deserialize, Serialize)]
pub struct ASI {
    value: u8,      // point value applied to ability score, not a modifier (usuall 1)
    count: u8,      // number of points to allocate (usually 3)
    maxper: u8,     // maximum number of points that may be allocated per stat
    stats: Vec<String>, // ability scores to which points may be allocated
}

/// Prerequisites for multiclassing
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Prerequisites {
    STR: Option<u8>,
    DEX: Option<u8>,
    CON: Option<u8>,
    INT: Option<u8>,
    WIS: Option<u8>,
    CHA: Option<u8>,
}

impl Index<Stat> for Prerequisites {
    type Output = Option<u8>;

    fn index(&self, stat: Stat) -> &Self::Output {
        match stat {
            Stat::STR => &self.STR,
            Stat::DEX => &self.DEX,
            Stat::CON => &self.CON,
            Stat::INT => &self.INT,
            Stat::WIS => &self.WIS,
            Stat::CHA => &self.CHA,
        }
    }
}

// ========================================
// Source Features and Effects
// ========================================
// Features include everying that is brought in from a race, class, background,
// or feat. Features are implemented mechanically through choose blocks and
// effects. Effects are specially defined as a feature that modifies a class
// resource on an event, such as a long rest.

/// Contains the name and description for a feature; no other functionality
#[derive(Debug, Deserialize, Serialize)]
pub struct Feature {
    name: String,
    description: String,
}

/// Effect object, which modifies character resources on an event
#[derive(Debug, Deserialize, Serialize)]
pub struct Effect {
    target: String,
    operation: String,
    event: String,
    value: u32,
}

impl Effect {

    // TODO
    //
    // Effects always apply to the character resources at event. I need
    // a general function that accepts a character object and performs
    // the operation on the target with the value. Then, all character
    // Effect objects can be looped through when a given event occurs,
    // and if the event trigger matches the event, execute the function.

}

// ========================================
// Selection Pool and Choice Implementation
// ========================================

/// Generic selection pool object
#[derive(Debug, Deserialize, Serialize)]
pub struct SelectionPool {
    fixed: Vec<String>,
    choose: Choice,
}

/// Handles the selection of generic features
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Choice {
    count: u8,
    options: Vec<String>,
}

impl Choice {

    pub fn choose(&mut self, i: usize) -> Result<String, ChoiceError> {
        if i >= self.options.len() {
            return Err(ChoiceError::OutOfBounds);
        } else if self.count == 0 {
            return Err(ChoiceError::NoChoicesLeft);
        } else {
            self.count -= 1;
            Ok(self.options.remove(i))
        }
    }
}

/// Possible errors incurred during feature selection
#[derive(Debug)]
pub enum ChoiceError {
    OutOfBounds,
    NoChoicesLeft,
}

// EOF
