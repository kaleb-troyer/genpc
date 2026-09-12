// <comment>
// 2026-09-12
// Kaleb Troyer

use crate::meta::Meta;
use crate::typing::School;
use crate::function::Function;
use crate::benefits::Benefits;

use serde::{Deserialize, Serialize};


// ========================================
// Spell Implementation
// ========================================
// 

/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct Spell {
    meta: Meta,
    level: u8,
    school: School,
    ritual: bool,
    components: Component,
    scaling: HashMap<u8, Function>,
    benefits: Option<Benefits>,
    effects: Option<Vec<Effect>>,
}


// ========================================
// Material and Components
// ========================================
// 

/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct Material {
    item: Option<Reference>,
    cost: Option<Currency>,
    flavor: Option<String>,
    consume: bool,
}

/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct Component {
    verbal: bool,
    somatic: bool,
    material: Option<Material>,
}




// EOF
