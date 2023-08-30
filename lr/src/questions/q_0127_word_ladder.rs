use std::collections::{HashMap, HashSet, VecDeque};

pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    if !word_list.iter().any(|w| *w.clone() == end_word) {
        return 0;
    }

    let mut word_list = word_list;
    word_list.push(begin_word.clone());

    let mut nei = HashMap::new();

    for word in word_list {
        for j in 0..word.len() {
            let pattern = format!("{}*{}", &word[..j], &word[j + 1..]);
            nei.entry(pattern).or_insert(vec![]).push(word.clone());
        }
    }

    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(begin_word);
    let mut res = 1;
    while !q.is_empty() {
        let q_len = q.len();
        for _ in 0..q_len {
            let word = q.pop_front().unwrap();
            if word == end_word {
                return res;
            }
            for j in 0..word.len() {
                let pattern = format!("{}*{}", &word[..j], &word[j + 1..]);
                for nei_word in nei.get(&pattern).unwrap() {
                    if !visited.contains(nei_word) {
                        visited.insert(nei_word.clone());
                        q.push_back(nei_word.clone());
                    }
                }
            }
        }
        res += 1
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ladder_length() {
        assert_eq!(
            ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                    "cog".to_string()
                ]
            ),
            5
        );
        assert_eq!(
            ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string()
                ]
            ),
            0
        );
    }
}
