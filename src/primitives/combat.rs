// <comment>
// 2026-09-04
// Kaleb Troyer


use crate::dynmod::{DynVec}
use crate::abilities::{Stat}
use crate::units::{Duration}
use crate::typing::{DamageType}


use serde::{Deserialize, Serialize};


// ========================================
// 
// ========================================
// 

/// 
#[derive(Debug, Deserialize, Serialize)]
pub enum AttackAttr {
    Spellcasting,
    Ability(Vec<Stat>)
}

///
#[derive(Debug, Deserialize, Serialize)]
pub struct Attack {
    attribute: AttackAttr,
    bonus: DynVec,
}


// ========================================
// 
// ========================================
// 

/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct Damage {
    roll: DiceRef,
    kind: DamageType,
}




// EOF
