use std::collections::VecDeque;

pub fn is_valid(s: String) -> bool {
    if s.is_empty() {
        return true;
    }
    if s.len() % 2 != 0 {
        return false;
    }
    let mut chars_stack = VecDeque::new();
    for c in s.chars() {
        match c {
            '{' | '[' | '(' => chars_stack.push_front(c),
            '}' => {
                if chars_stack.is_empty() {
                    return false;
                }
                if chars_stack.pop_front().unwrap() != '{' {
                    return false;
                }
            }
            ']' => {
                if chars_stack.is_empty() {
                    return false;
                }
                if chars_stack.pop_front().unwrap() != '[' {
                    return false;
                }
            }
            ')' => {
                if chars_stack.is_empty() {
                    return false;
                }
                if chars_stack.pop_front().unwrap() != '(' {
                    return false;
                }
            }
            _ => return false,
        }
    }
    chars_stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test]
    fn test_is_valid() {
        assert!(is_valid("()".to_string()));
        assert!(is_valid("()[]{}".to_string()));
        assert!(is_valid("{[]}".to_string()));
        assert!(!is_valid("(".to_string()));
        assert!(!is_valid("(]".to_string()));
        assert!(!is_valid("((".to_string()));
    }
}
