use std::collections::HashMap;

pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (obstacle_grid.len() as i32, obstacle_grid[0].len() as i32);
    let mut cache: HashMap<(i32, i32), i32> = HashMap::new();
    fn dfs(
        obstacle_grid: &[Vec<i32>],
        r: i32,
        c: i32,
        rows: i32,
        cols: i32,
        cache: &mut HashMap<(i32, i32), i32>,
    ) -> i32 {
        if r >= rows || c >= cols || obstacle_grid[r as usize][c as usize] == 1 {
            return 0;
        }

        if r == rows - 1 && c == cols - 1 {
            return 1;
        }

        if let Some(v) = cache.get(&(r, c)) {
            return *v;
        }

        let result = dfs(obstacle_grid, r + 1, c, rows, cols, cache)
            + dfs(obstacle_grid, r, c + 1, rows, cols, cache);
        cache.insert((r, c), result);
        result
    }

    dfs(&obstacle_grid, 0, 0, rows, cols, &mut cache)
}

#[test]
fn test_unique_paths_with_obstacles() {
    let obstacle_grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    assert_eq!(unique_paths_with_obstacles(obstacle_grid), 2);
}
