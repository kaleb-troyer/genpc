
use crate::system::{Stat::*, AbilityScores, Coin::*, Currency};
use crate::load::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct Background {
    id: String,
    name: String,
    source: String,
    benefits: HashMap<u8, Benefits>,
    equipment: Equipment,
    description: String,
}






// EOF
