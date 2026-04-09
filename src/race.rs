
use crate::system::{Stat::*, AbilityScores};
use crate::load::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Species structure and collection
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Race {
    id: String,
    name: String,
    source: String,
    #[serde(rename = "type")]
    kind: String,
    size: Choice,
    speed: u8,
    benefits: HashMap<u8, Benefits>,
    description: String,
}




// EOF
