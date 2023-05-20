pub fn longest_palindrome(s: String) -> String {
    let n = s.len();
    if n == 0 {
        return "".to_string();
    }
    if n == 1 {
        return s;
    }

    let (mut minstart, mut maxlen) = (0, 0);

    let mut i = 0;
    while i < n {
        if n - i < maxlen / 2 {
            break;
        }

        let (mut l, mut r) = (i, i);

        // Find the center of the palindrome
        while r < n - 1 && s.chars().nth(r).unwrap() == s.chars().nth(r + 1).unwrap() {
            r += 1;
        }

        // Update the next starting point
        i = r + 1;

        // Expand around the center to find the longest palindrome
        while l > 0 && r < n - 1 && s.chars().nth(l - 1).unwrap() == s.chars().nth(r + 1).unwrap() {
            l -= 1;
            r += 1;
        }

        let newlen = r - l + 1;
        if newlen > maxlen {
            maxlen = newlen;
            minstart = l;
        }
    }

    s[minstart..minstart + maxlen].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(longest_palindrome("aba".to_string()), "aba".to_string());
        assert_eq!(longest_palindrome("abb".to_string()), "bb".to_string());
    }
}
