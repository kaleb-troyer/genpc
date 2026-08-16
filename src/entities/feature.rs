// <comment>
// 2026-08-16
// Kaleb Troyer

// ========================================
// Source Features and Resources
// ========================================
// Features include everying that is brought in from a race, class, background,
// or feat. Features are implemented mechanically through choose blocks and
// effects. Effects are specially defined as a feature that modifies a class
// resource on an event, such as a long rest. Resources are just class resources
// like superiority dice, maneuvers, etc.

/// Contains the name and description for a feature; no other functionality
#[derive(Debug, Deserialize, Serialize)]
pub struct Feature {
    name: String,
    description: String,
}

/// Contains the resource name, associated die, resource usage, etc.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Resource {
    id: String,
    name: String,
    roll: Option<DiceRef>,
    uses: Option<u8>,
    items: Option<SelectionPool>,
}

// EOF
