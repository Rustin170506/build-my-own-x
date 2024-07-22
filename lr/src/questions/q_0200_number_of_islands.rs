use std::collections::{HashMap, VecDeque};

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[1].len());
    let mut result = 0;
    let mut vistied = HashMap::new();
    fn dfs(
        i: isize,
        j: isize,
        rows: usize,
        cols: usize,
        visited: &mut HashMap<(usize, usize), bool>,
        grid: &[Vec<char>],
    ) {
        if i as usize >= rows
            || i < 0
            || j as usize >= cols
            || j < 0
            || visited.contains_key(&(i as usize, j as usize))
            || grid[i as usize][j as usize] == '0'
        {
            return;
        }
        let (i, j) = (i as usize, j as usize);
        visited.insert((i, j), true);
        let directions: Vec<(isize, isize)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (x, y) in directions {
            dfs(i as isize + x, j as isize + y, rows, cols, visited, grid);
        }
    }
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '1' && !vistied.contains_key(&(i, j)) {
                dfs(i as isize, j as isize, rows, cols, &mut vistied, &grid);
                result += 1;
            }
        }
    }
    result
}
pub fn num_islands_with_bfs(grid: Vec<Vec<char>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut visited = HashMap::new();
    let mut result = 0;
    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '1' && !visited.contains_key(&(i, j)) {
                result += 1;
                let mut queue = VecDeque::new();
                queue.push_back((i, j));
                visited.insert((i, j), true);

                while let Some((x, y)) = queue.pop_front() {
                    for (dx, dy) in &directions {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
                            let nx = nx as usize;
                            let ny = ny as usize;
                            if grid[nx][ny] == '1' && !visited.contains_key(&(nx, ny)) {
                                queue.push_back((nx, ny));
                                visited.insert((nx, ny), true);
                            }
                        }
                    }
                }
            }
        }
    }

    result
}

#[test]
fn test_num_islands() {
    assert_eq!(
        num_islands(vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0']
        ]),
        1
    );
    assert_eq!(
        num_islands(vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1']
        ]),
        3
    );
}

#[test]
fn test_num_islands_with_bfs() {
    assert_eq!(
        num_islands_with_bfs(vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0']
        ]),
        1
    );
    assert_eq!(
        num_islands_with_bfs(vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1']
        ]),
        3
    );
}
