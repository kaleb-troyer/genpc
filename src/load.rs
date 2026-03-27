
use serde::{Deserialize, Serialize};

// ========================================
// Primary Substructures from Data
// ========================================

#[derive(Debug, Deserialize, Serialize)]
pub struct Benefits {
    ability_scores: Option<ASmod>,
    feats: Option<SelectionPool>,
    skill_profs: Option<SelectionPool>,
    tool_profs: Option<SelectionPool>,
    features: Option<Vec<Feature>>,
    effects: Option<Vec<Effect>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Equipment {
    A: EquipmentOps,
    B: EquipmentOps,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EquipmentOps {
    items: Option<Vec<String>>,
    currency: Option<Currency>, // TODO import from system
    choose: Option<Choice>
}

// ========================================
// Ability Score Modifier Implementation
// ========================================

#[derive(Debug, Deserialize, Serialize)]
pub struct ASmod {
    value: u8,
    count: u8,
    maxper: u8,
    stats: Vec<String>,
}

// ========================================
// Source Features and Effects
// ========================================
// Features include everying that is brought in from a race, class, background,
// or feat. Features are implemented mechanically through choose blocks and
// effects. Effects are specially defined as a feature that modifies a class
// resource on an event, such as a long rest.

/// Simply contains name and description for a feature
#[derive(Debug, Deserialize, Serialize)]
pub struct Feature {
    name: String,
    description: String,
}

// Effect object and implementation
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
#[derive(Debug, Deserialize, Serialize, Clone)]
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
enum ChoiceError {
    OutOfBounds,
    NoChoicesLeft,
}

// EOF
