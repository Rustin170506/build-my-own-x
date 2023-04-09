use std::collections::{hash_map::Entry, HashMap};

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result: HashMap<Vec<i32>, Vec<String>> = HashMap::default();
    for s in strs {
        let mut key = vec![0; 26];
        for c in s.chars() {
            key[c as usize - 'a' as usize] += 1;
        }

        match result.entry(key) {
            Entry::Occupied(mut e) => e.get_mut().push(s),
            Entry::Vacant(e) => {
                e.insert(vec![s]);
            }
        }
    }

    result.into_values().collect()
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
