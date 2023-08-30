use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut count_map = HashMap::new();

    for c in s.chars() {
        let count = count_map.entry(c).or_insert(0);
        *count += 1;
    }

    for c in t.chars() {
        if let Some(count) = count_map.get_mut(&c) {
            *count -= 1;
        } else {
            return false;
        }
    }

    for count in count_map.values() {
        if *count != 0 {
            return false;
        }
    }

    true
}

#[test]
fn test_is_anagram() {
    assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
    assert!(!is_anagram("rat".to_string(), "car".to_string()));
    assert!(is_anagram("你我".to_string(), "我你".to_string()));
}
