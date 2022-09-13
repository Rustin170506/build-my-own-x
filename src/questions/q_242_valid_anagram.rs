use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.is_empty() || t.is_empty() || s.len() != t.len() {
        return false;
    }
    let mut s_map: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        let count = s_map.entry(c).or_insert(0);
        *count += 1
    }

    for c in t.chars() {
        if let Some(x) = s_map.get_mut(&c) {
            *x -= 1;
        } else {
            return false;
        }
    }

    for v in s_map.values() {
        if *v != 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert!(!is_anagram("".to_string(), "".to_string()));
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
        assert!(!is_anagram("rat".to_string(), "car".to_string()));
        assert!(!is_anagram("aacc".to_string(), "ccac".to_string()));
    }
}
