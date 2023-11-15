use std::collections::{hash_map::Entry, HashMap};

pub fn word_pattern(pattern: String, s: String) -> bool {
    let words: Vec<&str> = s.split(' ').collect();
    if pattern.len() != words.len() {
        return false;
    }

    let mut pattern_map = HashMap::new();
    let mut words_map = HashMap::new();

    for (c, word) in pattern.chars().zip(words.iter()) {
        match pattern_map.entry(c) {
            Entry::Occupied(entry) if entry.get() != word => return false,
            Entry::Vacant(entry) => {
                entry.insert(*word);
            }
            _ => {}
        }
        match words_map.entry(*word) {
            Entry::Occupied(entry) if entry.get() != &c => return false,
            Entry::Vacant(entry) => {
                entry.insert(c);
            }
            _ => {}
        }
    }

    true
}

#[test]
fn test_word_pattern() {
    assert!(word_pattern(
        "abba".to_owned(),
        "dog cat cat dog".to_owned()
    ));
    assert!(!word_pattern(
        "abba".to_owned(),
        "dog catt cat dog".to_owned()
    ));
    assert!(!word_pattern(
        "abba".to_owned(),
        "dog cat cat fish".to_owned()
    ));
    assert!(!word_pattern(
        "aaaa".to_owned(),
        "dog cat cat dog".to_owned()
    ));
    assert!(!word_pattern(
        "aaa".to_owned(),
        "cat cat cat dog".to_owned()
    ));
    assert!(!word_pattern("aaa".to_owned(), "cat cat".to_owned()));
    assert!(!word_pattern(
        "abba".to_owned(),
        "dog dog dog dog".to_owned()
    ));
}
