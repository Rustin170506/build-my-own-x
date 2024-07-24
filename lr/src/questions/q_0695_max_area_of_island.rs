use std::collections::HashSet;

pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut max_area = 0;
    let mut visited = HashSet::new();
    fn dfs(
        grid: &[Vec<i32>],
        rows: usize,
        cols: usize,
        i: isize,
        j: isize,
        visited: &mut HashSet<(isize, isize)>,
    ) -> i32 {
        if i as usize >= rows
            || i < 0
            || j as usize >= cols
            || j < 0
            || grid[i as usize][j as usize] == 0
            || visited.contains(&(i, j))
        {
            return 0;
        }
        visited.insert((i, j));
        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut count = 1;
        for (x, y) in directions {
            count += dfs(grid, rows, cols, i + x, j + y, visited);
        }
        count
    }
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 1 && !visited.contains(&(r as isize, c as isize)) {
                let count = dfs(&grid, rows, cols, r as isize, c as isize, &mut visited);
                max_area = i32::max(max_area, count)
            }
        }
    }

    max_area
}

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
