const DICTIONARY_PATH: &str = "src/dictionary/dataset/zoega.json";

pub enum DictionaryLocation {
    MarkupDictionary,
}

impl DictionaryLocation { 
    pub fn get_path(&self) -> String { 
        match *self { 
            DictionaryLocation::MarkupDictionary => String::from(DICTIONARY_PATH), 
        }
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_dictionary_paths() {
        let result1 = DictionaryLocation::MarkupDictionary.get_path();

        assert_eq!(result1, DICTIONARY_PATH);
    }
}