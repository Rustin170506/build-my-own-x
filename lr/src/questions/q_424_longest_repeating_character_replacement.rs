use std::collections::HashMap;

pub fn character_replacement(s: String, k: i32) -> i32 {
    assert!(!s.is_empty());
    if s.len() == 1 {
        return 1;
    }

    let mut result = 0;
    let mut left = 0;
    let mut count = HashMap::new();
    for i in 0..s.len() {
        count
            .entry(s.chars().nth(i))
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        while (i - left + 1) as i32 - count.values().max_by(|x, y| x.cmp(y)).unwrap() > k {
            count
                .entry(s.chars().nth(left))
                .and_modify(|counter| *counter -= 1);
            left += 1;
        }

        result = i32::max(result, (i - left + 1) as i32);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_replacement() {
        assert_eq!(character_replacement("A".to_string(), 1), 1);
        assert_eq!(character_replacement("ABAB".to_string(), 2), 4);
        assert_eq!(character_replacement("AABABBA".to_string(), 1), 4);
    }
}
