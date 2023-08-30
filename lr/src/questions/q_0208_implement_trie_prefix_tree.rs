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
            if cur.children.get(&c).is_none() {
                cur.children.insert(c, TrieNode::new());
            }
            cur = cur.children.get_mut(&c).unwrap();
        }

        cur.end_of_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut cur = &self.root;
        for c in word.chars() {
            if cur.children.get(&c).is_none() {
                return false;
            }
            cur = cur.children.get(&c).unwrap();
        }

        cur.end_of_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = &self.root;
        for c in prefix.chars() {
            if cur.children.get(&c).is_none() {
                return false;
            }
            cur = cur.children.get(&c).unwrap();
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_208() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("app".to_string()));
        assert!(trie.starts_with("app".to_string()));
        trie.insert("app".to_string());
        assert!(trie.search("app".to_string()));
    }
}
