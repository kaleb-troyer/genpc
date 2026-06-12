
use crate::system::{Stat::*, AbilityScores, Coin::*, Currency};
use crate::load::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
