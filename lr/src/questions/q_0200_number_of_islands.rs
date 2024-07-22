use std::collections::HashMap;

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
