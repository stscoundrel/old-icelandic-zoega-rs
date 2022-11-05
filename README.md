# Old Icelandic Zoega

Old Icelandic dictionary for Rust. From "A Concise Dictionary of Old Icelandic" by Geir Zoëga.

The dictionary consists of 29 000+ Old Icelandic words with English translations. Also available for [Node.js](https://github.com/stscoundrel/old-icelandic-zoega)


### Install

`cargo add old_icelandic_zoega`

Or add this to your `Cargo.toml`:

```toml
[dependencies]
old_icelandic_zoega = "1.1.0"
```

### Usage

Dictionary comes in two variants: one with html markup, and one without.

```rust
// Ships two variants, plus DictionaryEntry.
use old_icelandic_zoega::{get_dictionary, get_no_markup_dictionary, DictionaryEntry};

// Standard dictionary. Contains "strong" and "i" tags to match the printed book.
let dictionary = get_dictionary();

// No-markup version. Contains no tags or additional markup.
let no_markup_dictionary = get_no_markup_dictionary();

// Both methods return Result, which should always be safe to unwrap.
// Up to you if you wish to just unwrap, or use other error handling method.
let dictionary_content: Vec<DictionaryEntry> = dictionary.unwrap();
let no_markup_dictionary_content: Vec<DictionaryEntry> = no_markup_dictionary.unwrap();

println!("A word from dictionary: {}. First definition for it is: {}", &dictionary_content[0].word, &dictionary_content[0].definitions[0])
```


### About "A Concise Dictionary of Old Icelandic"

"A Concise Dictionary of Old Icelandic" dictionary was published in 1910 by Geir Zoëga, which leads to there being many public domain versions of the book available. Zoëgas attempt was to made easier-to-approach version of the more full Cleasby - Vigfusson dictionary, specifically for beginners and those interested in Old Icelandic prose writing.
