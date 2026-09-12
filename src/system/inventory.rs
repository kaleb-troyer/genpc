// <comment>
// 2026-08-16
// Kaleb Troyer

use crate::value::Currency;
use crate::reference::Reference;

use serde::{Deserialize, Serialize};


// ========================================
// Inventory and ItemStack
// ========================================
//

/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct Inventory {
    items: Option<Vec<ItemStack>>,
    purse: Option<Currency>,
}

/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemStack {
    id: Reference,
    qty: u32,
}




// EOF
