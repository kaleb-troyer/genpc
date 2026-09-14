// Choices are the fundamental structures that define what and how many features
// are conferred to a character when new features are gained. Choices are
// presented via a selection pool, which, despite it's name, offers both fixed
// features automatically given the character and a set of options the player
// can choose from. The selection pool additionally stores what options were
// chosen, which is useful only after the struct has been cloned.
// 2026-09-13
// Kaleb Troyer

// ========================================
// Selection Pool and Choice Implementation
// ========================================
// 

/// Generic selection pool object
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct SelectionPool<T> {
    selection: Option<Vec<T>>,
    fixed: Option<Vec<T>>,
    choose: Choice<T>,
}

/// Handles the selection of generic features
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Choice<T> {
    count: u8,
    options: ChoiceTypes<T>,
}

impl Choice {

    /// Decrements the number of remaining choices and removes the
    /// selection from the pool of options.
    pub fn choose(&mut self, i: usize) -> Result<String, ChoiceError> {
        if i >= self.options.len() {
            return Err(ChoiceError::OutOfBounds);
        } else if self.count == 0 {
            return Err(ChoiceError::NoChoicesLeft);
        } else {
            self.count -= 1;
            Ok(self.options.remove(i))
        }
    }
}

/// 
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ChoiceTypes<T> {
    Options(Vec<T>),
    ListRef(String),
}

/// Possible errors incurred during feature selection
#[derive(Debug)]
pub enum ChoiceError {
    OutOfBounds,
    NoChoicesLeft,
}

// EOF
