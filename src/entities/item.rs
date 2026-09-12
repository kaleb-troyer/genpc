// <comment>
// 2026-09-09
// Kaleb Troyer

use crate::value::{Rarity, Coin, Currency};
use crate::benefits::Benefits;
use crate::meta::Meta;
use crate::effect::Effect;
use crate::reference::Reference;
use crate::function::Function;
use crate::abilities::Preprequisites;


// ========================================
// Item Struct and Implementation
// ========================================
//

/// 
#[derive(Debug, Deserialize, Serialize, Default)]
struct Item {
    meta: Meta,
    weight: u16,
    rarity: Rarity,
    value: Option<Currency>,
    magic: bool,
    attunement: bool,
    preprequisites: Option<Preprequisites>,
    benefits: Option<Benefit>,
    effects: Option<Vec<Effect>>,
    properties: Option<Vec<Properties>,
    mastery: Option<Vec<Reference>>,
    usage: Option<Function>,
}

// ========================================
// Weapon Characteristics
// ========================================
//

/// 
#[derive(Debug, Deserialize, Serialize, Default)]
pub enum Properties {
    Versatile, Finese, Heavy, Thrown,
    Reach, Loading, TwoHanded, Ammunition,
}


// EOF
