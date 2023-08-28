pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];

    for i in (0..=text1.len() - 1).rev() {
        for j in (0..=text2.len() - 1).rev() {
            if text1.chars().nth(i).unwrap() == text2.chars().nth(j).unwrap() {
                dp[i][j] = 1 + dp[i + 1][j + 1];
            } else {
                dp[i][j] = i32::max(dp[i][j + 1], dp[i + 1][j]);
            }
        }
    }

    dp[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_subsequence() {
        assert_eq!(
            longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        );
        assert_eq!(
            longest_common_subsequence("abc".to_string(), "abc".to_string()),
            3
        );
        assert_eq!(
            longest_common_subsequence("abc".to_string(), "def".to_string()),
            0
        );
    }
}
