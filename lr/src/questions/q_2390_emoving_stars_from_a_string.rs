use std::collections::VecDeque;

pub fn remove_stars(s: String) -> String {
    let mut stack = VecDeque::new();
    for c in s.chars() {
        if c == '*' {
            if !stack.is_empty() {
                stack.pop_back();
            }
        } else {
            stack.push_back(c);
        }
    }

    stack.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2390() {
        assert_eq!(remove_stars("a*b*c".to_string()), "c".to_string());
        assert_eq!(remove_stars("a*bc".to_string()), "bc".to_string());
        assert_eq!(remove_stars("leet**cod*e".to_string()), "lecoe".to_string());
    }
}
