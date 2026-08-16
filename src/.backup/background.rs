// <comment>
// 2026-06-21
// Kaleb Troyer

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::system::{Stat::*, AbilityScores, Coin::*, Currency};
use crate::common::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Background {
    pub id: String,
    pub name: String,
    pub source: String,
    pub benefits: HashMap<u8, Benefits>,
    pub equipment: Equipment,
    pub description: String,
}






// EOF
