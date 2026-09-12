// <comment>
// 2026-06-21
// Kaleb Troyer

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::meta::Meta;
use crate::inventory::Inventory;
use crate::benefits::Benefits;
use crate::abilities::Stat;
use crate::typing::CasterType;


/// Class structure and collection
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Class {
    meta: Meta,
    hit_die: u8,
    primary_ability: Option<SelectionPool<Stat>>,
    saving_throws: Option<Vec<Stat>>,
    caster_type: Option<CasterType>,
    benefits: HashMap<u8, Benefits>,
    equipment: Option<SelectionPool<Inventory>>,
}


// /// Class structure and collection
// #[derive(Debug, Deserialize, Serialize, Default)]
// pub struct Class {
//     id: String,                         // json file name
//     name: String,                       // in-game class identifier
//     source: String,
//     hit_die: u8,
//     primary_ability: Vec<String>,       // vector of STR, DEX, CON, INT, WIS, or CHA
//     saving_throws: Vec<String>,         // " "
//     caster_type: Option<String>,        // indicates full, half, pact, etc.
//     benefits: HashMap<String, Benefits>,// see load.rs
//     description: String,
// }
//
// /// Subclass structure and collection
// #[derive(Debug, Deserialize, Serialize, Default)]
// pub struct Subclass {
//     id: String,
//     name: String,
//     source: String,
//     parent: String,                     // defines the parent of the subclass; is for collection
//     saving_throws: Option<Vec<String>>,
//     caster_type: Option<String>,
//     benefits: HashMap<String, Benefits>,
//     description: String,
// }

// EOF
