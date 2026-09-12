// <comment>
// 2026-09-12
// Kaleb Troyer

use crate::reference::Reference;
use crate::dice::DiceRef;
use crate::choice::SelectionPool;
use crate::dynmod::DynMod;
use crate::units::Distance;

use serde::{Deserialize, Serialize};


// ========================================
// Feature Implementation
// ========================================
// 

/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct Feature {
    id: Reference,
    name: String,
    roll: Option<DiceRef>,
    usage: Option<Function>,
    items: Option<SelectionPool<Reference>>,
    charges: Option<Charges>,
    effects: Vec<Effect>,
    description: String,
}

/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct Charges {
    current: Option<u8>,
    max: DynMod,
}


// ========================================
// Proficiency Implementation
// ========================================
// 

/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct Feature {
    skills: Option<SelectionPool<Skill>>,
    tools: Option<SelectionPool<Reference>>,
    langs: Option<SelectionPool<Reference>>,
    armor: Option<SelectionPool<ArmorType>>,
    weaps: Option<SelectionPool<WeaponType>>,
}


// ========================================
// Sense Implementation
// ========================================
// 

/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct Sense {
    kind: String,
    tint: Option<String>,
    range: Distance,
}


// EOF
