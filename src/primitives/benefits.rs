// Benefits are the abilities, attributes, and features conferred to a character
// via their class, race, and more. The benefits struct serves as a transferable
// container for these benefits, following the same pattern established by Meta.
// 2026-09-13
// Kaleb Troyer

use crate::attributes::{Proficiencies, Sense};
use crate::feature::Feature;


/// Benefits object and contents as imported from /data
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Benefits {
    proficiencies: Option<Proficiencies>,
    senses: Option<Vec<Sense>>,
    features: Option<Vec<Feature>>,
}

// EOF
