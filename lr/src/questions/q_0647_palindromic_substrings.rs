pub fn count_substrings(s: String) -> i32 {
    let mut res = 0;
    for i in 0..s.len() {
        let (mut l, mut r) = (i as i32, i as i32);

        while l >= 0
            && (r as usize) < s.len()
            && s.chars().nth(l as usize).unwrap() == s.chars().nth(r as usize).unwrap()
        {
            res += 1;
            l -= 1;
            r += 1;
        }

        let (mut l, mut r) = (i as i32, (i + 1) as i32);

        while l >= 0
            && (r as usize) < s.len()
            && s.chars().nth(l as usize).unwrap() == s.chars().nth(r as usize).unwrap()
        {
            res += 1;
            l -= 1;
            r += 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_substrings() {
        assert_eq!(count_substrings("s".to_string()), 1);
        assert_eq!(count_substrings("sss".to_string()), 6);
        assert_eq!(count_substrings("abc".to_string()), 3);
    }
}
