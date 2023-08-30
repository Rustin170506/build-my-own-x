use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let mut result = 0;
    let mut char_set = HashSet::new();
    let mut left = 0;

    for c in s.chars() {
        while char_set.contains(&c) {
            char_set.remove(&s.chars().nth(left).unwrap());
            left += 1;
        }

        char_set.insert(c);
        result = i32::max(result, char_set.len() as i32)
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("".to_string()), 0);
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("aaaaaa".to_string()), 1);
    }
}
