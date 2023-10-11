pub fn remove_stars(s: String) -> String {
    let mut res = String::new();
    let mut star_count = 0;

    for c in s.chars().rev() {
        if c == '*' {
            star_count += 1;
        } else {
            if star_count > 0 {
                star_count -= 1;
            } else {
                res.push(c);
            }
        }
    }

    res.chars().rev().collect()
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
