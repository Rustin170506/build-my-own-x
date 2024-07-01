use std::collections::{HashMap, HashSet};

pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
    let words = HashSet::from_iter(dictionary);
    let mut dp = HashMap::new();
    dfs(&s, 0, &words, &mut dp)
}

fn dfs(s: &String, i: usize, words: &HashSet<String>, dp: &mut HashMap<usize, i32>) -> i32 {
    if i == s.len() {
        return 0;
    }

    if let Some(&result) = dp.get(&i) {
        return result;
    }

    let mut result = 1 + dfs(s, i + 1, words, dp);

    for j in i..s.len() {
        if words.get(&s[i..j + 1]).is_some() {
            result = i32::min(result, dfs(s, j + 1, words, dp));
        }
    }

    dp.insert(i, result);

    result
}

#[test]
fn test_min_extra_char() {
    let s = "abc";
    let dictionary = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    assert_eq!(min_extra_char(s.to_string(), dictionary), 0);

    let s = "abc";
    let dictionary = vec!["a".to_string(), "b".to_string()];
    assert_eq!(min_extra_char(s.to_string(), dictionary), 1);

    let s = "abc";
    let dictionary = vec!["a".to_string(), "b".to_string(), "d".to_string()];
    assert_eq!(min_extra_char(s.to_string(), dictionary), 1);

    let s = "abc";
    let dictionary = vec![
        "a".to_string(),
        "b".to_string(),
        "d".to_string(),
        "ab".to_string(),
    ];
    assert_eq!(min_extra_char(s.to_string(), dictionary), 1);
}
