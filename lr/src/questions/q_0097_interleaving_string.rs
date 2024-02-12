pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }

    let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];
    dp[s1.len()][s2.len()] = true;

    for i in (0..=s1.len()).rev() {
        for j in (0..=s2.len()).rev() {
            if i < s1.len()
                && s1.chars().nth(i).unwrap() == s3.chars().nth(i + j).unwrap()
                && dp[i + 1][j]
            {
                dp[i][j] = true;
            }
            if j < s2.len()
                && s2.chars().nth(j).unwrap() == s3.chars().nth(i + j).unwrap()
                && dp[i][j + 1]
            {
                dp[i][j] = true;
            }
        }
    }

    dp[0][0]
}

#[test]
fn test_is_interleave() {
    assert!(is_interleave(
        "aabcc".to_string(),
        "dbbca".to_string(),
        "aadbbcbcac".to_string()
    ));
    assert!(!is_interleave(
        "aabcc".to_string(),
        "dbbca".to_string(),
        "aadbbbaccc".to_string()
    ));
    assert!(is_interleave(
        "".to_string(),
        "".to_string(),
        "".to_string()
    ));
}
