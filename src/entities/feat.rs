// <comment>
// 2026-06-21
// Kaleb Troyer

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::meta::Meta;
use crate::benefits::Benefits;
use crate::abilities::Prerequisites;

/// Feat structure and collection
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Feat {
    meta: Meta,
    prerequisites: Option<Prerequisites>,
    benefits: HashMap<u8, Benefits>,
}


// /// Feat structure and collection
// #[derive(Debug, Deserialize, Serialize, Default)]
// pub struct Feat {
//     #[serde(rename = "type")]
//     kind: String, // keyword 'type' is reserved; using kind
//     source: String,
//     id: String,
//     name: String,
//     prerequisites: Prerequisites,
//     benefits: HashMap<u8, Benefits>,
//     description: String,
// }

// EOF
