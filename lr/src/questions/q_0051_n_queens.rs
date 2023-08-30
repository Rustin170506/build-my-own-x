use std::collections::HashSet;

pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut result = vec![];
    let mut board = vec![vec!['.'; n as usize]; n as usize];
    let mut cols = HashSet::new();
    let mut pos_diags = HashSet::new();
    let mut neg_diags = HashSet::new();

    backtrack(
        &mut result,
        &mut board,
        &mut cols,
        &mut pos_diags,
        &mut neg_diags,
        0,
        n as isize,
    );

    result
}

fn backtrack(
    result: &mut Vec<Vec<String>>,
    board: &mut Vec<Vec<char>>,
    cols: &mut HashSet<isize>,
    pos_diags: &mut HashSet<isize>,
    neg_diags: &mut HashSet<isize>,
    r: isize,
    n: isize,
) {
    if r == n {
        result.push(board.iter().map(|row| row.iter().collect()).collect());
        return;
    }
    for c in 0..n {
        if cols.contains(&c) || pos_diags.contains(&(r + c)) || neg_diags.contains(&(r - c)) {
            continue;
        }

        cols.insert(c);
        pos_diags.insert(r + c);
        neg_diags.insert(r - c);
        board[r as usize][c as usize] = 'Q';

        backtrack(result, board, cols, pos_diags, neg_diags, r + 1, n);

        cols.remove(&c);
        pos_diags.remove(&(r + c));
        neg_diags.remove(&(r - c));
        board[r as usize][c as usize] = '.';
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_n_queens() {
        assert_eq!(
            vec![
                vec![
                    ".Q..".to_string(),
                    "...Q".to_string(),
                    "Q...".to_string(),
                    "..Q.".to_string()
                ],
                vec![
                    "..Q.".to_string(),
                    "Q...".to_string(),
                    "...Q".to_string(),
                    ".Q..".to_string()
                ]
            ],
            solve_n_queens(4)
        );
    }
}
