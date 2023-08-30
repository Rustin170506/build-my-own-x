pub fn num_decodings(s: String) -> i32 {
    let mut dp = vec![1; s.len() + 1];

    for i in (0..=s.len() - 1).rev() {
        if s.chars().nth(i).unwrap() == '0' {
            dp[i] = 0;
        } else {
            dp[i] = dp[i + 1];
        }

        if i + 1 < s.len()
            && (s.chars().nth(i).unwrap() == '1'
                || (s.chars().nth(i).unwrap() == '2' && s[i..=i + 1].parse::<i32>().unwrap() <= 26))
        {
            dp[i] += dp[i + 2];
        }
    }

    dp[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_decodings() {
        assert_eq!(num_decodings("12".to_string()), 2);
        assert_eq!(num_decodings("226".to_string()), 3);
        assert_eq!(num_decodings("0".to_string()), 0);
        assert_eq!(num_decodings("06".to_string()), 0);
    }
}
