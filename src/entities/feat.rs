// <comment>
// 2026-06-21
// Kaleb Troyer

use std::sync::LazyLock;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::system::{Stat::*, AbilityScores};
use crate::common::*;

/// Feat structure and collection
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Feat {
    #[serde(rename = "type")]
    kind: String, // keyword 'type' is reserved; using kind
    source: String,
    id: String,
    name: String,
    prerequisites: Prerequisites,
    benefits: HashMap<u8, Benefits>,
    description: String,
}




// EOF
