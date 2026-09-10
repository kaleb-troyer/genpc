// <comment>
// 2026-09-04
// Kaleb Troyer



use serde::{Deserialize, Serialize};

// ========================================
// 
// ========================================
// 

/// 
#[derive(Debug, Deserialize, Serialize)]
pub enum DamageType {
    Piercing,
    Slashing,
    Bludgeoning,
    Poison,
    Fire,
    Cold,
    Thunder,
    Lightening,
    Acid,
    Necrotic,
    Radiant,
    Psychic,
    Force,
    Healing,
}

/// 
#[derive(Debug, Deserialize, Serialize)]
pub enum CasterType {
    Full,
    Half,
    Third,
    Pact,
}

/// 
#[derive(Debug, Deserialize, Serialize)]
pub enum ArmorType {
    Light, Medium, Heavy, Shields
}

/// 
#[derive(Debug, Deserialize, Serialize)]
pub enum WeaponType {
    Simple, Martial
}

/// 
#[derive(Debug, Deserialize, Serialize)]
pub enum ActionType {
    A,  // Normal action
    M,  // Magic action
    B,  // Bonus action
    R,  // Reaction
    L,  // Legendary action
    F,  // Free action
    T(Duration), // Time, not action economy
}

/// 
#[derive(Debug, Deserialize, Serialize)]
pub enum School {
    Transmutation,
    Divination,
    Abjuration,
    Evocation,
    Conjuration,
    Enchantment,
    Illusion,
    Necromancy,
}




// EOF
