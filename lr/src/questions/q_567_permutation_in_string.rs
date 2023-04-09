pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }
    assert!(!s1.is_empty());
    assert!(!s2.is_empty());
    let (mut s1_count, mut s2_count) = ([0; 26], [0; 26]);
    for i in 0..s1.len() {
        s1_count[s1.chars().nth(i).unwrap() as usize - 'a' as usize] += 1;
        s2_count[s2.chars().nth(i).unwrap() as usize - 'a' as usize] += 1;
    }
    let mut matches = 0;

    for i in 0..26 {
        matches += i32::from(s1_count[i] == s2_count[i]);
    }

    for (left, right) in (s1.len()..s2.len()).enumerate() {
        if matches == 26 {
            return true;
        }

        let index = s2.chars().nth(right).unwrap() as usize - 'a' as usize;
        s2_count[index] += 1;
        if s1_count[index] == s2_count[index] {
            matches += 1;
        } else if s1_count[index] + 1 == s2_count[index] {
            matches -= 1;
        }

        let index = s2.chars().nth(left).unwrap() as usize - 'a' as usize;
        s2_count[index] -= 1;
        if s1_count[index] == s2_count[index] {
            matches += 1;
        } else if s1_count[index] - 1 == s2_count[index] {
            matches -= 1;
        }
    }

    matches == 26
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
