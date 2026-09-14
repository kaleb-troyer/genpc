// <comment>
// 2026-09-12
// Kaleb Troyer

use serde::{Deserialize, Serialize};

use crate::units::{Duration, Range, Summon};
use crate::typing::ActionType;
use crate::combat::{Attack, Damage};
use crate::abilities::Stat;


// ========================================
// Function Implementation
// ========================================
//

/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct Function {
    duration: Option<Duration>,
    action: Option<ActionType>,
    target: Option<u8>,
    ranges: Option<Range>,
    saving: Option<Stat>,
    summon: Option<Summon>,
    attack: Option<Attack>,
    damage: Option<Damage>,
}



// EOF
