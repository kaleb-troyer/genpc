
use crate::system::{Stat::*, AbilityScores, Coin::*, Currency};
use crate::load::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Background {
    id: String,
    name: String,
    source: String,
    benefits: Benefits,
    equipment: Equipment,
    description: String,
}






// EOF
