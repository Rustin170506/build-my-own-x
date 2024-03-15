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

#[derive(Debug, Clone)]
struct TrieNodeV2 {
    children: Vec<Option<TrieNodeV2>>,
    end_of_word: bool,
}

impl TrieNodeV2 {
    fn new() -> Self {
        TrieNodeV2 {
            children: vec![None; 26],
            end_of_word: false,
        }
    }
}

struct TrieV2 {
    root: TrieNodeV2,
}

impl TrieV2 {
    fn new() -> Self {
        TrieV2 {
            root: TrieNodeV2::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut cur = &mut self.root;
        for c in word.chars() {
            let index = c as usize - 'a' as usize;
            if cur.children[index].is_none() {
                cur.children[index] = Some(TrieNodeV2::new());
            }
            cur = cur.children[index].as_mut().unwrap();
        }

        cur.end_of_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut cur = &self.root;
        for c in word.chars() {
            let index = c as usize - 'a' as usize;
            if cur.children[index].is_none() {
                return false;
            }
            cur = cur.children[index].as_ref().unwrap();
        }

        cur.end_of_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = &self.root;
        for c in prefix.chars() {
            let index = c as usize - 'a' as usize;
            if cur.children[index].is_none() {
                return false;
            }
            cur = cur.children[index].as_ref().unwrap();
        }

        true
    }
}

#[test]
fn test_trie() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    assert!(trie.search("apple".to_string()));
    assert!(!trie.search("app".to_string()));
    assert!(trie.starts_with("app".to_string()));
    trie.insert("app".to_string());
    assert!(trie.search("app".to_string()));
    // V2
    let mut trie = TrieV2::new();
    trie.insert("apple".to_string());
    assert!(trie.search("apple".to_string()));
    assert!(!trie.search("app".to_string()));
    assert!(trie.starts_with("app".to_string()));
    trie.insert("app".to_string());
    assert!(trie.search("app".to_string()));
}
