// <comment>
// 2026-08-16
// Kaleb Troyer

// ========================================
// Selection Pool and Choice Implementation
// ========================================
// asdf

/// Generic selection pool object
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct SelectionPool {
    fixed: Option<Vec<String>>,
    choose: Choice,
}

/// Handles the selection of generic features
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Choice {
    count: u8,
    options: ChoiceTypes,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ChoiceTypes {
    Single(String),
    Multiple(Vec<String>)
}

impl ChoiceTypes {

    pub fn len(&self) -> usize {
        match self {
            ChoiceTypes::Single(_) => 1,
            ChoiceTypes::Multiple(v) => v.len(),
        }
    }

    pub fn remove(&mut self, i: usize) -> String {
        match self {
            ChoiceTypes::Single(s) => s.clone(),
            ChoiceTypes::Multiple(v) => v.remove(i),
        }
    }
}

impl Default for ChoiceTypes {
    fn default() -> Self {
        ChoiceTypes::Multiple(Vec::new())
    }
}

/// Possible errors incurred during feature selection
#[derive(Debug)]
pub enum ChoiceError {
    OutOfBounds,
    NoChoicesLeft,
}

// EOF
