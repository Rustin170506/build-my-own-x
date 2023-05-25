pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut dp = vec![false; s.len() + 1];
    dp[s.len()] = true;

    for i in (0..s.len()).rev() {
        for w in &word_dict {
            if i + w.len() <= s.len() && &s[i..i + w.len()].to_string() == w {
                dp[i] = dp[i + w.len()];
            }
            if dp[i] {
                break;
            }
        }
    }

    dp[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        assert!(word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()]
        ));
        assert!(word_break(
            "leetleetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()]
        ));
    }
}
