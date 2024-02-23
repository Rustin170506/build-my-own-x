pub fn num_distinct(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for item in dp.iter_mut().take(s.len() + 1) {
        item[0] = 1;
    }
    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }
    dp[s.len()][t.len()]
}

#[test]
fn test() {
    let s = "rabbbit".to_string();
    let t = "rabbit".to_string();
    let res = 3;
    assert_eq!(num_distinct(s, t), res);
    let s = "babgbag".to_string();
    let t = "bag".to_string();
    let res = 5;
    assert_eq!(num_distinct(s, t), res);
}
