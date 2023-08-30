use std::collections::VecDeque;

pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let mut q = VecDeque::new();
    let (mut time, mut fresh) = (0, 0);
    let (rows, cols) = (grid.len(), grid[0].len());

    for (r, row) in grid.iter().enumerate() {
        for (c, e) in row.iter().enumerate() {
            if *e == 1 {
                fresh += 1;
            }

            if *e == 2 {
                q.push_back((r, c));
            }
        }
    }

    let directions: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    while !q.is_empty() && fresh > 0 {
        let q_len = q.len();
        for _ in 0..q_len {
            let (r, c) = q.pop_front().unwrap();
            for (dr, dc) in &directions {
                let (row, col) = (dr + r as isize, dc + c as isize);

                if (row < 0 || row == rows as isize)
                    || (col < 0 || col == cols as isize)
                    || grid[row as usize][col as usize] != 1
                {
                    continue;
                }

                grid[row as usize][col as usize] = 2;
                q.push_back((row as usize, col as usize));
                fresh -= 1;
            }
        }
        time += 1
    }

    if fresh == 0 {
        time
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oranges_rotting() {
        assert_eq!(
            oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1],]),
            4
        );
        assert_eq!(
            oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1],]),
            -1
        );
        assert_eq!(oranges_rotting(vec![vec![0, 2],]), 0);
    }
}
