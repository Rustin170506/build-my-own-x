use std::collections::HashMap;

pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (matrix.len(), matrix[0].len());
    let mut dp: HashMap<(isize, isize), i32> = HashMap::new();

    for r in 0..rows {
        for c in 0..cols {
            dfs(r as isize, c as isize, rows, cols, -1, &matrix, &mut dp);
        }
    }

    *dp.values().max().unwrap()
}

fn dfs(
    r: isize,
    c: isize,
    rows: usize,
    cols: usize,
    pre: i32,
    matrix: &[Vec<i32>],
    dp: &mut HashMap<(isize, isize), i32>,
) -> i32 {
    if r < 0
        || r as usize == rows
        || c < 0
        || c as usize == cols
        || matrix[r as usize][c as usize] <= pre
    {
        return 0;
    }

    if dp.contains_key(&(r, c)) {
        return *dp.get(&(r, c)).unwrap();
    }

    let mut res = 1;
    let cur_value = matrix[r as usize][c as usize];
    res = res.max(1 + dfs(r + 1, c, rows, cols, cur_value, matrix, dp));
    res = res.max(1 + dfs(r - 1, c, rows, cols, cur_value, matrix, dp));
    res = res.max(1 + dfs(r, c + 1, rows, cols, cur_value, matrix, dp));
    res = res.max(1 + dfs(r, c - 1, rows, cols, cur_value, matrix, dp));
    dp.insert((r, c), res);

    res
}

#[test]
fn test_longest_increasing_path() {
    let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
    assert_eq!(longest_increasing_path(matrix), 4);
    let matrix = vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]];
    assert_eq!(longest_increasing_path(matrix), 4);
    let matrix = vec![vec![1]];
    assert_eq!(longest_increasing_path(matrix), 1);
    let matrix = vec![vec![1, 2]];
    assert_eq!(longest_increasing_path(matrix), 2);
    let matrix = vec![vec![1], vec![2]];
    assert_eq!(longest_increasing_path(matrix), 2);
    let matrix = vec![vec![1, 2], vec![3, 4]];
    assert_eq!(longest_increasing_path(matrix), 3);
    let matrix = vec![
        vec![1, 2, 3, 4, 5],
        vec![16, 17, 24, 23, 6],
        vec![15, 18, 25, 22, 7],
        vec![14, 19, 20, 21, 8],
        vec![13, 12, 11, 10, 9],
    ];
    assert_eq!(longest_increasing_path(matrix), 25);
}
