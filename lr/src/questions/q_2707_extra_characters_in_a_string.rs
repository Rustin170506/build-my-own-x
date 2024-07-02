use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            end_of_word: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut cur = &mut self.root;
        for c in word.chars() {
            cur.children.entry(c).or_insert_with(TrieNode::new);
            cur = cur.children.get_mut(&c).unwrap();
        }

        cur.end_of_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut cur = &self.root;
        for c in word.chars() {
            if !cur.children.contains_key(&c) {
                return false;
            }
            cur = cur.children.get(&c).unwrap();
        }

        cur.end_of_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = &self.root;
        for c in prefix.chars() {
            if !cur.children.contains_key(&c) {
                return false;
            }
            cur = cur.children.get(&c).unwrap();
        }

        true
    }
}

pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
    let mut trie = Trie::new();
    for w in dictionary {
        trie.insert(w);
    }
    let mut dp = HashMap::new();
    dfs(&s, 0, &trie, &mut dp)
}

fn dfs(s: &String, i: usize, trie: &Trie, dp: &mut HashMap<usize, i32>) -> i32 {
    if i == s.len() {
        return 0;
    }

    if let Some(&result) = dp.get(&i) {
        return result;
    }

    let mut result = 1 + dfs(s, i + 1, trie, dp);
    let mut current = &trie.root;
    for j in i..s.len() {
        let c = &s.chars().nth(j).unwrap();
        if !current.children.contains_key(c) {
            break;
        }
        current = current.children.get(c).unwrap();
        if current.end_of_word {
            result = i32::min(result, dfs(s, j + 1, trie, dp));
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
