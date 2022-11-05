pub enum DictionaryLocation {
    MarkupDictionary,
    NoMarkupDictionary,
}

impl DictionaryLocation {
    pub fn get_dictionary_json(&self) -> &'static str {
        match *self {
            DictionaryLocation::MarkupDictionary => include_str!("./dataset/zoega.json"),
            DictionaryLocation::NoMarkupDictionary => include_str!("./dataset/zoega-no-markup.json")
        }
    }
}