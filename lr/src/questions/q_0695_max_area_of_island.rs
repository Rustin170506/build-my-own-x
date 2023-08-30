use std::collections::HashSet;

pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut max_area = 0;
    let mut visited = HashSet::new();

    for r in 0..rows {
        for c in 0..cols {
            max_area = i32::max(
                max_area,
                dfs(
                    r as i32,
                    c as i32,
                    rows as i32,
                    cols as i32,
                    &grid,
                    &mut visited,
                ),
            )
        }
    }

    max_area
}

fn dfs(
    r: i32,
    c: i32,
    rows: i32,
    cols: i32,
    grid: &[Vec<i32>],
    visited: &mut HashSet<(i32, i32)>,
) -> i32 {
    if r < 0
        || r == rows
        || c < 0
        || c == cols
        || grid[r as usize][c as usize] == 0
        || visited.contains(&(r, c))
    {
        return 0;
    }

    visited.insert((r, c));

    1 + dfs(r + 1, c, rows, cols, grid, visited)
        + dfs(r - 1, c, rows, cols, grid, visited)
        + dfs(r, c + 1, rows, cols, grid, visited)
        + dfs(r, c - 1, rows, cols, grid, visited)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area_of_island() {
        assert_eq!(
            max_area_of_island(vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 1],
                vec![0, 0, 0, 1, 1]
            ]),
            4
        );
    }
}
