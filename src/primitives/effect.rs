// <comment>
// 2026-08-16
// Kaleb Troyer

// ========================================
// Effects
// ========================================
// Effects are specially defined as a feature that modifies a class
// resource on an event, such as a long rest. Resources are just class resources
// like superiority dice, maneuvers, etc.

/// Effect object, which modifies character resources on an event
#[derive(Debug, Deserialize, Serialize)]
pub struct Effect {
    target: String,
    operation: String,
    event: String,
    value: DynMod,
}

impl Effect {

    // TODO
    //
    // Effects always apply to the character resources at event. I need
    // a general function that accepts a character object and performs
    // the operation on the target with the value. Then, all character
    // Effect objects can be looped through when a given event occurs,
    // and if the event trigger matches the event, execute the function.

    pub fn check(&self, event: &str) -> bool {
        self.event == event
    }

    // // !!! THIS DOESN"T WORK YET, DEVELOPMENT NOT DONE
    // pub fn apply(&self, char: &mut Character) {
    //
    //     // find target resource
    //     for res in char.resources.iter_mut() {
    //         if res.id == self.target {
    //             let mut attr = res.uses;
    //
    //             // modify resource
    //             match self.operation.as_str() {
    //                 "add" => attr = attr + self.value,
    //                 "sub" => attr = attr - self.value,
    //                 "set" => attr = self.value,
    //                 _ => None
    //             }
    //             break;
    //         }
    //     }
    // }
}

// EOF
