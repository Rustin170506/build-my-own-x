use std::collections::VecDeque;

pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let mut time = 0;
    let mut queue = VecDeque::new();
    let mut fresh = 0;
    let (rows, cols) = (grid.len(), grid[0].len());
    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    for i in 0..rows {
        for j in 0..cols {
            match grid[i][j] {
                1 => fresh += 1,
                2 => queue.push_back((i, j)),
                _ => {}
            }
        }
    }

    while !queue.is_empty() && fresh != 0 {
        let q_len = queue.len();
        for _ in 0..q_len {
            let (i, j) = queue.pop_front().unwrap();
            for (x, y) in &directions {
                let i = i as isize + x;
                let j = j as isize + y;
                if i < 0 || i >= rows as isize || j < 0 || j >= cols as isize {
                    continue;
                }
                let i = i as usize;
                let j = j as usize;
                match grid[i][j] {
                    1 => {
                        grid[i][j] = 2;
                        fresh -= 1;
                        queue.push_back((i, j));
                    }
                    _ => {}
                }
            }
        }
        time += 1;
    }
    if fresh != 0 {
        return -1;
    }

    time
}

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
