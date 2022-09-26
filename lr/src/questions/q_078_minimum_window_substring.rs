use std::collections::HashMap;

pub fn min_window(s: String, t: String) -> String {
    if t.is_empty() || s.len() < t.len() {
        return "".to_string();
    }

    let mut result = (-1_i32, -1_i32);

    let mut count_t = HashMap::new();
    for c in t.chars() {
        count_t
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut windows = HashMap::new();
    let (mut have, need) = (0, count_t.len());

    let mut left = 0;
    for right in 0..s.len() {
        let c = s.chars().nth(right).unwrap();
        if count_t.contains_key(&c) {
            windows
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            if windows.get(&c) == count_t.get(&c) {
                have += 1;
            }
        }
        while have == need {
            if ((right - left + 1) as i32) < (result.1 - result.0 + 1)
                || (result.1 == -1 && result.0 == -1)
            {
                result = (left as i32, right as i32);
            }
            let c = s.chars().nth(left).unwrap();
            if count_t.contains_key(&c) {
                windows.entry(c).and_modify(|count| *count -= 1);
                if windows.get(&c) < count_t.get(&c) {
                    have -= 1;
                }
            }
            left += 1;
        }
    }

    if result.1 == -1 && result.0 == -1 {
        return "".to_string();
    }

    let mut result_string = String::new();
    for i in result.0..result.1 + 1 {
        result_string.push(s.chars().nth(i as usize).unwrap());
    }

    result_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window() {
        assert_eq!(min_window("aa".to_string(), "aa".to_string()), "aa");
        assert_eq!(min_window("a".to_string(), "b".to_string()), "");
        assert_eq!(min_window("A".to_string(), "ABC".to_string()), "");
        assert_eq!(
            min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC"
        );
        assert_eq!(min_window("a".to_string(), "a".to_string()), "a");
    }
}
