
use crate::system::{Stat::*, AbilityScores};
use crate::load::*;
use std::sync::LazyLock;
use serde::{Deserialize, Serialize};

/// Species structure and collection
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Feat {
    category: String,
    type: String,
    source: String,
    id: String,
    name: String,
    prerequisites: Prerequisites,
    benefits: HashMap<u8, Benefits>,
    description: String,
}




// EOF
