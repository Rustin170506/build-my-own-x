use std::collections::{hash_map::Entry, HashMap};

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    for s in strs {
        let mut s_chars: Vec<char> = s.chars().collect();
        s_chars.sort_by(|a, b| b.cmp(a));
        let key = s_chars.into_iter().collect();

        match result.entry(key) {
            Entry::Occupied(mut e) => e.get_mut().push(s),
            Entry::Vacant(e) => {
                e.insert(vec![s]);
            }
        }
    }

    result.iter().map(|(_, v)| v.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        assert_eq!(
            group_anagrams(vec!["".to_string()]),
            vec![vec!["".to_string()]]
        );
        assert_eq!(
            group_anagrams(vec!["a".to_string()]),
            vec![vec!["a".to_string()]]
        );
    }
}
