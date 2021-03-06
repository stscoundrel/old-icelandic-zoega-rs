use serde::{Deserialize, Serialize};

/// Individual dictionary entry.
/// Each entry contains word, and at least one definition for the word.
/// Many words have multiple definitions, or additional info in definitions.
///
#[derive(Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub word: String,
    pub definitions: Vec<String>
}