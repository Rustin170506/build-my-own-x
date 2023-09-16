use std::collections::HashSet;

pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let mut visited = HashSet::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                return dfs(&grid, &mut visited, i as i32, j as i32);
            }
        }
    }
    unreachable!()
}

fn dfs(grid: &[Vec<i32>], visited: &mut HashSet<(i32, i32)>, i: i32, j: i32) -> i32 {
    if (i < 0 || (i as usize) >= grid.len())
        || (j < 0 || j as usize >= grid[0].len())
        || grid[i as usize][j as usize] == 0
    {
        return 1;
    }
    if visited.contains(&(i, j)) {
        return 0;
    }

    visited.insert((i, j));

    let mut p = dfs(grid, visited, i + 1, j);
    p += dfs(grid, visited, i - 1, j);
    p += dfs(grid, visited, i, j + 1);
    p += dfs(grid, visited, i, j - 1);
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_island_perimeter() {
        assert_eq!(
            16,
            island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ])
        );
    }
}
