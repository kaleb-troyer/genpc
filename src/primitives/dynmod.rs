// <comment>
// 2026-08-16
// Kaleb Troyer

use crate::dice::{DiceRef}

// ========================================
// 
// ========================================
// asdf

/// 
#[derive(Debug, Deserialize, Serialize)]
pub enum DynMod {
    Flat(i32),
    Roll(Box<DiceRef>),
    Reference(String),
}

/// 
#[derive(Debug, Deserialize, Serialize)]
pub enum DynVec {
    One(DynMod),
    Many(Vec<DynMod>),
}



// EOF
