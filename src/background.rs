
use crate::system::{Stat::*, AbilityScores};
use crate::load::*;
use std::sync::LazyLock;
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
