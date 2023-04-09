pub fn is_palindrome(s: String) -> bool {
    if !s.is_ascii() {
        return false;
    }

    let s = s.to_lowercase();
    let low_case_s = s.trim();
    let mut temp = String::new();

    for c in low_case_s.chars() {
        if c.is_ascii_lowercase() || c.is_ascii_digit() {
            temp.push(c);
        }
    }

    if temp.is_empty() {
        return true;
    }

    let rev_temp = temp.chars().rev().collect::<String>();
    temp == rev_temp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(!is_palindrome("0P".to_string()));
        assert!(is_palindrome(" ".to_string()));
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
        assert!(!is_palindrome("race a car".to_string()));
    }
}
