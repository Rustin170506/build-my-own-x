use std::collections::{HashMap, HashSet};

struct TrieNode {
    pub(crate) children: HashMap<char, TrieNode>,
    pub(crate) end_of_word: bool,
}

impl TrieNode {
    pub(crate) fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            end_of_word: false,
        }
    }

    pub(crate) fn insert(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            if cur.children.get(&c).is_none() {
                cur.children.insert(c, TrieNode::new());
            }
            cur = cur.children.get_mut(&c).unwrap();
        }

        cur.end_of_word = true;
    }
}

pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let mut root = TrieNode::new();

    for word in words {
        root.insert(word);
    }

    let (rows, cols) = (board.len(), board[0].len());
    let mut visted: HashSet<(i32, i32)> = HashSet::new();
    let mut res = HashSet::new();

    #[allow(clippy::too_many_arguments)]
    fn dfs(
        board: &Vec<Vec<char>>,
        row: i32,
        col: i32,
        rows: i32,
        cols: i32,
        res: &mut HashSet<String>,
        visted: &mut HashSet<(i32, i32)>,
        node: &TrieNode,
        word: String,
    ) {
        if row < 0
            || col < 0
            || row == rows
            || col == cols
            || node
                .children
                .get(&board[row as usize][col as usize])
                .is_none()
            || visted.get(&(row, col)).is_some()
        {
            return;
        }

        visted.insert((row, col));

        let c = board[row as usize][col as usize];
        let next_node = node.children.get(&c).unwrap();
        let mut word = word;
        word.push(c);
        if next_node.end_of_word {
            res.insert(word.clone());
        }
        dfs(
            board,
            row + 1,
            col,
            rows,
            cols,
            res,
            visted,
            next_node,
            word.clone(),
        );
        dfs(
            board,
            row - 1,
            col,
            rows,
            cols,
            res,
            visted,
            next_node,
            word.clone(),
        );
        dfs(
            board,
            row,
            col + 1,
            rows,
            cols,
            res,
            visted,
            next_node,
            word.clone(),
        );
        dfs(
            board,
            row,
            col - 1,
            rows,
            cols,
            res,
            visted,
            next_node,
            word.clone(),
        );
        visted.remove(&(row, col));
    }

    for r in 0..rows {
        for c in 0..cols {
            dfs(
                &board,
                r as i32,
                c as i32,
                rows as i32,
                cols as i32,
                &mut res,
                &mut visted,
                &root,
                "".to_string(),
            )
        }
    }

    let mut sorted_res: Vec<String> = res.into_iter().collect();
    sorted_res.sort();

    sorted_res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_words() {
        assert_eq!(
            find_words(
                vec![
                    vec!['o', 'a', 'a', 'n'],
                    vec!['e', 't', 'a', 'e'],
                    vec!['i', 'h', 'k', 'r'],
                    vec!['i', 'f', 'l', 'v']
                ],
                vec![
                    "oath".to_string(),
                    "pea".to_string(),
                    "eat".to_string(),
                    "rain".to_string()
                ]
            ),
            vec!["eat".to_string(), "oath".to_string()]
        );
        assert_eq!(
            find_words(
                vec![
                    vec!['o', 'a', 'a', 'n'],
                    vec!['e', 't', 'a', 'e'],
                    vec!['i', 'h', 'k', 'r'],
                    vec!['i', 'f', 'l', 'v']
                ],
                vec![
                    "oath".to_string(),
                    "pea".to_string(),
                    "eat".to_string(),
                    "rain".to_string(),
                    "hklf".to_string(),
                    "hf".to_string(),
                ]
            ),
            vec![
                "eat".to_string(),
                "hf".to_string(),
                "hklf".to_string(),
                "oath".to_string()
            ]
        );
    }
}
