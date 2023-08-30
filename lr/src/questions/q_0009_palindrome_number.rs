pub fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let mut x = x;
    let mut reverted_num = 0;

    while x > reverted_num {
        reverted_num = reverted_num * 10 + x % 10;
        x /= 10;
    }

    reverted_num == x || reverted_num / 10 == x
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(121));
        assert!(!is_palindrome(-121));
        assert!(!is_palindrome(10));
        assert!(is_palindrome(1221));
    }
}
