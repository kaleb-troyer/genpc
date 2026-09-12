// <comment>
// 2026-06-21
// Kaleb Troyer

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::meta::Meta;
use crate::benefits::Benefits;
use crate::inventory::Inventory;
use crate::typing::Size;

/// Species structure and collection
#[derive(Debug, Deserialize, Serialize)]
pub struct Race {
    meta: Meta,
    size: SelectionPool<Size>,
    benefits: HashMap<u8, Benefits>,
    equipment: Option<Vec<Inventory>>,
}


// /// Species structure and collection
// #[derive(Debug, Deserialize, Serialize, Default)]
// pub struct Race {
//     id: String,
//     name: String,
//     source: String,
//     #[serde(rename = "type")]
//     kind: String,           // System-specific, e.g. "humanoid", etc.
//     size: SelectionPool,    // System-specific, e.g. "small", "medium", etc.
//     speed: u8,
//     benefits: HashMap<u8, Benefits>,
//     description: String,
// }

// EOF
