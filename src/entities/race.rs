// <comment>
// 2026-06-21
// Kaleb Troyer

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::system::{Stat::*, AbilityScores};
use crate::common::*;

/// Species structure and collection
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Race {
    id: String,
    name: String,
    source: String,
    #[serde(rename = "type")]
    kind: String,           // System-specific, e.g. "humanoid", etc.
    size: SelectionPool,    // System-specific, e.g. "small", "medium", etc.
    speed: u8,
    benefits: HashMap<u8, Benefits>,
    description: String,
}




// EOF
