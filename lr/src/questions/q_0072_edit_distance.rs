pub fn min_distance(word1: String, word2: String) -> i32 {
    let (m, n) = (word1.len(), word2.len());
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for (i, item) in dp.iter_mut().enumerate().take(m + 1) {
        item[0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            if word1.chars().nth(i - 1) == word2.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j].min(dp[i][j - 1]));
            }
        }
    }
    dp[m][n] as i32
}

#[test]
fn test_0072() {
    assert_eq!(3, min_distance("horse".to_string(), "ros".to_string()));
    assert_eq!(
        5,
        min_distance("intention".to_string(), "execution".to_string())
    );
}
