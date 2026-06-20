
use crate::load::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Class structure and collection
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Class {
    id: String,                         // json file name
    name: String,                       // in-game class identifier
    source: String,
    hit_die: u8,
    primary_ability: Vec<String>,       // vector of STR, DEX, CON, INT, WIS, or CHA
    saving_throws: Vec<String>,         // " "
    caster_type: Option<String>,        // indicates full, half, pact, etc.
    benefits: HashMap<String, Benefits> // see load.rs
}

/// Subclass structure and collection
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Subclass {
    id: String,
    name: String,
    source: String,
    parent: String,                     // defines the parent of the subclass; is for collection
    saving_throws: Option<Vec<String>>,
    caster_type: Option<String>,
    benefits: HashMap<String, Benefits>,
    description: String,
}

// EOF
