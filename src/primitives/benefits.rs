// <comment>
// 2026-08-16
// Kaleb Troyer

// ========================================
// Primary Substructures from Data
// ========================================
// Describes the top-level substructures from classes, feats, backgrounds, etc.
// from .json data. The Option<> type indicates that the field doesn't need to
// be present for the structure to instantiate from the data.

/// Benefits object and contents as imported from /data
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Benefits {
    pub ability_scores: Option<ASI>,        // ability score modifier selection, see below
    pub feats: Option<SelectionPool>,       // feats selection, see below
    pub skill_profs: Option<SelectionPool>, // skill proficiency selection
    pub tool_profs: Option<SelectionPool>,  // tool proficiency selection
    pub sublcass: Option<Choice>,           // subclass selection
    pub features: Option<Vec<Feature>>,     // list of primary source features, see below
    pub resources: Option<Vec<Resource>>,   // list of character resources, see below
    pub effects: Option<Vec<Effect>>,       // list of primary source effects, see below
}

/// Collection of available equipment load-outs from backgrounds and classes
pub type Equipment = Vec<EquipmentOps>;

/// Equipment load-out contents
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct EquipmentOps {
    pub items: Option<Vec<String>>,
    pub currency: Option<Currency>,
    pub choose: Option<Choice>
}

// EOF
