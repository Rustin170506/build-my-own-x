pub fn solve(board: &mut Vec<Vec<char>>) {
    let (rows, cols) = (board.len(), board[0].len());

    for r in 0..rows {
        for c in 0..cols {
            if board[r][c] == 'O' && ((r == 0 || r == rows - 1) || (c == 0 || c == cols - 1)) {
                capture(rows as isize, cols as isize, board, r as isize, c as isize)
            }
        }
    }

    for rows in board.iter_mut() {
        for c in rows.iter_mut() {
            if *c == 'O' {
                *c = 'X';
            }
        }
    }

    for rows in board.iter_mut() {
        for c in rows.iter_mut() {
            if *c == 'F' {
                *c = 'O';
            }
        }
    }
}

fn capture(rows: isize, cols: isize, board: &mut [Vec<char>], r: isize, c: isize) {
    if r < 0 || r == rows || c < 0 || c == cols || board[r as usize][c as usize] != 'O' {
        return;
    }

    board[r as usize][c as usize] = 'F';
    capture(rows, cols, board, r + 1, c);
    capture(rows, cols, board, r - 1, c);
    capture(rows, cols, board, r, c + 1);
    capture(rows, cols, board, r, c - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        solve(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        );
    }
}
