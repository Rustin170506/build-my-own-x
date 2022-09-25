use std::collections::HashMap;

pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }
    assert!(!s1.is_empty());
    assert!(!s2.is_empty());
    let mut s1_map: HashMap<char, i32> = HashMap::new();
    for c in s1.chars() {
        s1_map.entry(c).and_modify(|count| *count += 1).or_insert(1);
    }
    let mut s2_map: HashMap<char, i32> = HashMap::new();

    for i in 0..s2.len() {
        let mut j = i;
        s2_map.clear();
        while j - i < s1.len() && j < s2.len() {
            s2_map
                .entry(s2.chars().nth(j).unwrap())
                .and_modify(|count| *count += 1)
                .or_insert(1);
            j += 1;
        }

        if s1_map == s2_map {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_replacement() {
        assert!(check_inclusion("a".to_string(), "a".to_string()));
        assert!(!check_inclusion("a".to_string(), "b".to_string()));
        assert!(check_inclusion("ab".to_string(), "eidbaooo".to_string()));
        assert!(!check_inclusion("ab".to_string(), "eidboaoo".to_string()));
        assert!(!check_inclusion(
            "hello".to_string(),
            "ooolleoooleh".to_string()
        ));
    }
}
