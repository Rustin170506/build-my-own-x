use std::collections::HashMap;

pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    let mut order_index_map = HashMap::new();

    for (i, c) in order.chars().enumerate() {
        order_index_map.insert(c, i);
    }

    for i in 0..words.len() - 1 {
        let (word1, word2) = (&words[i], &words[i + 1]);
        for j in 0..word1.len() {
            if j == word2.len() {
                return false;
            }
            let c1 = &word1.chars().nth(j).unwrap();
            let c2 = &word2.chars().nth(j).unwrap();
            if c1 != c2 {
                let index1 = order_index_map.get(c1).unwrap();
                let index2 = order_index_map.get(c2).unwrap();
                if index2 < index1 {
                    return false;
                } else {
                    // If we find the first different character and they are sorted,
                    // then there's no need to check remaining characters
                    break;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_alien_sorted() {
        assert!(
            is_alien_sorted(
                vec!["hello".to_string(), "leetcode".to_string(),],
                "hlabcdefgijkmnopqrstuvwxyz".to_string()
            )
        );
        assert!(
            !is_alien_sorted(
                vec!["word".to_string(), "world".to_string(), "row".to_string(),],
                "worldabcefghijkmnpqstuvxyz".to_string()
            )
        );
        assert!(
            !is_alien_sorted(
                vec!["apple".to_string(), "app".to_string(),],
                "abcdefghijklmnopqrstuvwxyz".to_string()
            )
        );
    }
}
