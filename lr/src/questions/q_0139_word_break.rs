//

use std::collections::{HashMap, HashSet};

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let set: HashSet<String> = word_dict.into_iter().collect();
    let s: Vec<char> = s.chars().collect();
    let mut mem: HashMap<usize, bool> = HashMap::new();
    helper(&s, &set, &mut mem, 0)
}

fn helper(s: &[char], set: &HashSet<String>, mem: &mut HashMap<usize, bool>, start: usize) -> bool {
    if start == s.len() {
        return true;
    }

    // If the result is in the cache, return it
    if let Some(&result) = mem.get(&start) {
        return result;
    }

    for end in start..s.len() {
        let temp = &s[start..=end];
        let word: String = temp.iter().collect();
        if set.contains(&word) && helper(s, set, mem, end + 1) {
            mem.insert(start, true); // Cache the result
            return true;
        }
    }

    mem.insert(start, false); // Cache the result
    false
}

pub fn word_break_dp(s: String, word_dict: Vec<String>) -> bool {
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    let set: HashSet<String> = word_dict.into_iter().collect();

    for i in 1..=n {
        for j in 0..i {
            if dp[j] && set.contains(&s[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }

    dp[n]
}

pub fn word_break_max_len(s: String, word_dict: Vec<String>) -> bool {
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    let max_len = word_dict.iter().map(|word| word.len()).max().unwrap_or(0);

    let word_dict: std::collections::HashSet<String> = word_dict.into_iter().collect();
    for i in 1..=n {
        for j in std::cmp::max(i as isize - max_len as isize - 1, 0) as usize..i {
            if dp[j] && word_dict.contains(&s[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }

    dp[n]
}

#[test]
fn test_word_break() {
    let s = "catstay".to_string();
    let word_dict = vec!["cats".to_string(), "cat".to_string(), "stay".to_string()];
    assert!(word_break(s, word_dict));
    let s = "aaaaaaa".to_string();
    let word_dict = vec!["aaaa".to_string(), "aaa".to_string()];
    assert!(word_break(s, word_dict));
    let s = "leetcode".to_string();
    let word_dict = vec!["leet".to_string(), "code".to_string()];
    assert!(word_break(s, word_dict));
    let s = "applepenapple".to_string();
    let word_dict = vec!["apple".to_string(), "pen".to_string()];
    assert!(word_break(s, word_dict));
    let s = "catsandog".to_string();
    let word_dict = vec![
        "cats".to_string(),
        "dog".to_string(),
        "sand".to_string(),
        "and".to_string(),
        "cat".to_string(),
    ];
    assert!(!word_break(s, word_dict));
}
