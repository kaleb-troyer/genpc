
use crate::load::*;
use serde::{Deserialize, Serialize};

/// Class structure and collection
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Class {
    id: String,
    name: String,
    source: String,
    prerequisites: Prerequisites,
    hit_die: u8,
    saving_throws: Vec<String>,
    benefits: Benefits,
    description: String,
}





// EOF
