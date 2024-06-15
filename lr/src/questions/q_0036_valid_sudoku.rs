use std::collections::{hash_map::Entry, HashMap, HashSet};

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    assert!(board.len() == 9);
    assert!(board[0].len() == 9);

    let mut row_set = HashSet::new();
    let mut column_set = HashSet::new();
    let mut sub_grid_map: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

    fn get_index(index: usize) -> usize {
        index / 3
    }

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if row_set.contains(&board[i][j]) {
                return false;
            } else if board[i][j] != '.' {
                row_set.insert(board[i][j]);
            }
            if column_set.contains(&board[j][i]) {
                return false;
            } else if board[j][i] != '.' {
                column_set.insert(board[j][i]);
            }
            if board[i][j] != '.' {
                let key = (get_index(i), get_index(j));
                if let Entry::Occupied(mut e) = sub_grid_map.entry(key) {
                    if e.get().get(&board[i][j]).is_some() {
                        return false;
                    } else {
                        e.get_mut().insert(board[i][j]);
                    }
                } else {
                    sub_grid_map.insert(key, HashSet::from_iter(vec![board[i][j]]));
                }
            }
        }
        column_set.clear();
        row_set.clear();
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_sudoku() {
        assert!(is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]));
        assert!(!is_valid_sudoku(vec![
            vec!['.', '.', '4', '.', '.', '.', '6', '3', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['5', '.', '.', '.', '.', '.', '.', '9', '.'],
            vec!['.', '.', '.', '5', '6', '.', '.', '.', '.'],
            vec!['4', '.', '3', '.', '.', '.', '.', '.', '1'],
            vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '5', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.']
        ]));
    }
}
