pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let mut board = board;
    let word = word.chars().collect::<Vec<_>>();
    fn dfs(board: &mut Vec<Vec<char>>, word: &Vec<char>, i: isize, j: isize, k: isize) -> bool {
        if k == word.len() as isize {
            return true;
        }
        if i >= board.len() as isize || j >= board[0].len() as isize || i < 0 || j < 0 {
            return false;
        }
        if board[i as usize][j as usize] != word[k as usize] {
            return false;
        }
        board[i as usize][j as usize] = ' ';
        let res = dfs(board, word, i + 1, j, k + 1)
            || dfs(board, word, i - 1, j, k + 1)
            || dfs(board, word, i, j + 1, k + 1)
            || dfs(board, word, i, j - 1, k + 1);
        board[i as usize][j as usize] = word[k as usize];
        res
    }
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == word[0] && dfs(&mut board, &word, i as isize, j as isize, 0) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCCED".to_string();
        assert!(exist(board, word));
    }
}
