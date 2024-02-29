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

pub fn length_of_longest_substring_brute_force(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let s: Vec<char> = s.chars().collect();

    let mut result = 0;
    for i in 0..s.len() {
        let mut seen: Vec<bool> = vec![false; 128];
        let mut j = i;
        while j < s.len() && !seen[s[j] as usize] {
            seen[s[j] as usize] = true;
            j += 1;
        }
        result = result.max(j - i);
    }

    result as i32
}

#[test]
fn test_length_of_longest_substring() {
    assert_eq!(length_of_longest_substring("".to_string()), 0);
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(length_of_longest_substring("aaaaaa".to_string()), 1);
    assert_eq!(length_of_longest_substring_brute_force("".to_string()), 0);
    assert_eq!(
        length_of_longest_substring_brute_force("abcabcbb".to_string()),
        3
    );
    assert_eq!(
        length_of_longest_substring_brute_force("aaaaaa".to_string()),
        1
    );
}
