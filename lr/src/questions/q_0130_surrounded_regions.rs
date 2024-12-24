#[allow(clippy::ptr_arg)]
pub fn solve(board: &mut Vec<Vec<char>>) {
    let (rows, cols) = (board.len() as i32, board[0].len() as i32);

    fn dfs(rows: i32, cols: i32, board: &mut [Vec<char>], row: i32, col: i32) {
        if row < 0
            || row >= rows
            || col < 0
            || col >= cols
            || board[row as usize][col as usize] != 'O'
        {
            return;
        }

        board[row as usize][col as usize] = 'F';
        dfs(rows, cols, board, row, col + 1);
        dfs(rows, cols, board, row, col - 1);
        dfs(rows, cols, board, row + 1, col);
        dfs(rows, cols, board, row - 1, col);
    }
    for row in 0..rows {
        for col in 0..cols {
            if board[row as usize][col as usize] == 'O'
                && (row == 0 || col == 0 || row == rows - 1 || col == cols - 1)
            {
                dfs(rows, cols, board, row, col)
            }
        }
    }

    for row in 0..rows {
        for col in 0..cols {
            if board[row as usize][col as usize] == 'O' {
                board[row as usize][col as usize] = 'X';
            }
        }
    }

    for row in 0..rows {
        for col in 0..cols {
            if board[row as usize][col as usize] == 'F' {
                board[row as usize][col as usize] = 'O';
            }
        }
    }
}

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
