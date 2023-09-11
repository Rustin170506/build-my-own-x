pub fn valid_palindrome(s: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let (mut l, mut r) = (0, s.len() - 1);

    while l < r {
        if s[l] != s[r] {
            return is_palindrome(&s[l + 1..r + 1]) || is_palindrome(&s[l..r]);
        }
        l += 1;
        r -= 1;
    }

    true
}

fn is_palindrome(s: &[char]) -> bool {
    let (mut left, mut right) = (0, s.len() - 1);

    while left < right {
        if s[left] != s[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_palindrome() {
        assert_eq!(valid_palindrome("aba".to_string()), true);
        assert_eq!(valid_palindrome("abca".to_string()), true);
        assert_eq!(valid_palindrome("abc".to_string()), false);
        assert_eq!(valid_palindrome("ab".to_string()), true);
        assert_eq!(valid_palindrome("a".to_string()), true);
    }
}
