use std::collections::HashSet;

pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if heights.is_empty() {
        return vec![];
    }

    let (rows, cols) = (heights.len() as i32, heights[0].len() as i32);
    let mut pacific = HashSet::new();
    let mut atlantic = HashSet::new();

    fn dsf(
        heights: &Vec<Vec<i32>>,
        row: i32,
        col: i32,
        rows: i32,
        cols: i32,
        visited: &mut HashSet<(i32, i32)>,
        previous_height: i32,
    ) {
        if row < 0
            || row >= rows
            || col < 0
            || col >= cols
            || visited.contains(&(row, col))
            || heights[row as usize][col as usize] < previous_height
        {
            return;
        }

        visited.insert((row, col));
        let directions = vec![vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]];
        for direction in directions {
            let (r, c) = (row + direction[0], col + direction[1]);
            dsf(
                heights,
                r,
                c,
                rows,
                cols,
                visited,
                heights[row as usize][col as usize],
            );
        }
    }

    for row in 0..rows {
        dsf(
            &heights,
            row,
            0,
            rows,
            cols,
            &mut pacific,
            heights[row as usize][0],
        );

        dsf(
            &heights,
            row,
            cols - 1,
            rows,
            cols,
            &mut atlantic,
            heights[row as usize][(cols - 1) as usize],
        );
    }

    for col in 0..cols {
        dsf(
            &heights,
            0,
            col,
            rows,
            cols,
            &mut pacific,
            heights[0][col as usize],
        );

        dsf(
            &heights,
            rows - 1,
            col,
            rows,
            cols,
            &mut atlantic,
            heights[(rows - 1) as usize][col as usize],
        );
    }

    let mut result = vec![];
    for i in 0..rows {
        for j in 0..cols {
            if pacific.contains(&(i, j)) && atlantic.contains(&(i, j)) {
                result.push(vec![i, j]);
            }
        }
    }

    result
}

#[test]
fn test_pacific_atlantic() {
    assert_eq!(
        vec![
            vec![0, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 2],
            vec![3, 0],
            vec![3, 1],
            vec![4, 0]
        ],
        pacific_atlantic(vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4]
        ])
    );

    assert_eq!(
        vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 1],
            vec![2, 0],
            vec![2, 1],
            vec![3, 0],
            vec![3, 1],
        ],
        pacific_atlantic(vec![vec![1, 1], vec![1, 1], vec![1, 1], vec![1, 1],])
    );
}
