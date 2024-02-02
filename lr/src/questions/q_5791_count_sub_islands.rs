use std::collections::HashSet;

pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (grid1.len(), grid1[0].len());
    let mut visited = HashSet::new();

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid2[r][c] == 1
                && !visited.contains(&(r as i32, c as i32))
                && dfs(&grid1, &grid2, rows, cols, &mut visited, r as i32, c as i32)
            {
                count += 1
            }
        }
    }

    count
}

fn dfs(
    grid1: &Vec<Vec<i32>>,
    grid2: &Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
    visited: &mut HashSet<(i32, i32)>,
    r: i32,
    c: i32,
) -> bool {
    if r < 0
        || c < 0
        || r == rows as i32
        || c == cols as i32
        || grid2[r as usize][c as usize] == 0
        || visited.contains(&(r, c))
    {
        return true;
    }
    visited.insert((r, c));
    let mut result = true;
    if grid1[r as usize][c as usize] == 0 {
        result = false;
    }

    result = dfs(grid1, grid2, rows, cols, visited, r + 1, c) && result;
    result = dfs(grid1, grid2, rows, cols, visited, r - 1, c) && result;
    result = dfs(grid1, grid2, rows, cols, visited, r, c + 1) && result;
    result = dfs(grid1, grid2, rows, cols, visited, r, c - 1) && result;

    result
}

#[test]
fn test_count_sub_islands() {
    let grid1 = vec![
        vec![1, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0],
        vec![1, 1, 0, 1, 1],
    ];
    let grid2 = vec![
        vec![1, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 1],
        vec![0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0],
        vec![1, 1, 0, 1, 1],
    ];
    assert_eq!(count_sub_islands(grid1, grid2), 3);
    let grid1 = vec![vec![1]];
    let grid2 = vec![vec![0]];
    assert_eq!(count_sub_islands(grid1, grid2), 0);
}
