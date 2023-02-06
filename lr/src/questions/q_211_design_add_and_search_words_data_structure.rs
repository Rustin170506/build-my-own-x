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

struct WordDictionary {
    root: TrieNode,
}

impl WordDictionary {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn add_word(&mut self, word: String) {
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
        fn dfs(j: usize, root: &TrieNode, word: &String) -> bool {
            let mut cur = root;
            for i in j..word.len() {
                let c = word.chars().nth(i).unwrap();
                if c == '.' {
                    for child in cur.children.values() {
                        if dfs(i + 1, child, word) {
                            return true;
                        }
                    }
                    return false;
                } else {
                    if cur.children.get(&c).is_none() {
                        return false;
                    }
                    cur = cur.children.get(&c).unwrap();
                }
            }

            cur.end_of_word
        }

        dfs(0, &self.root, &word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_211() {
        let mut wd = WordDictionary::new();
        wd.add_word("bad".to_string());
        wd.add_word("dad".to_string());
        wd.add_word("mad".to_string());
        assert!(!wd.search("pad".to_string()));
        assert!(wd.search("bad".to_string()));
        assert!(wd.search(".ad".to_string()));
        assert!(wd.search("b..".to_string()));
    }
}
