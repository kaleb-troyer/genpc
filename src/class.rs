
use crate::load::*;
use serde::{Deserialize, Serialize};

/// Class structure and collection
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Class {
    id: String,
    name: String,
    source: String,
    hit_die: u8,
    primary_ability: Vec<String>,
    saving_throws: Vec<String>,
    benefits: HashMap<u8, Benefits>,
    description: String,
}

/// Subclass structure and collection
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Subclass {
    id: String,
    name: String,
    source: String,
    saving_throws: Option<Vec<String>>,
    benefits: HashMap<u8, Benefits>,
    description: String,
}


// EOF
