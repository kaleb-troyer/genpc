
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
    caster_type: String,                // indicates full, half, pact, etc.
    benefits: HashMap<u8, Benefits>,    // see load.rs
}

/// Subclass structure and collection
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Subclass {
    id: String,
    name: String,
    source: String,
    saving_throws: Option<Vec<String>>,
    caster_type: String,
    benefits: HashMap<u8, Benefits>,
    description: String,
}

// EOF
