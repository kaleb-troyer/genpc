// <comment>
// 2026-06-21
// Kaleb Troyer

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::*;

/// Class structure and collection
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Class {
    id: String,                         // json file name
    name: String,                       // in-game class identifier
    source: String,
    benefits: HashMap<String, Benefits> // see load.rs
    description: String
}

// EOF
