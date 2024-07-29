use std::collections::HashSet;

pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if heights.is_empty() {
        return vec![];
    }
    let (rows, cols) = (heights.len(), heights[0].len());
    let mut result = vec![];
    let mut pacific = HashSet::new();
    let mut atlantic = HashSet::new();

    fn dfs(
        rows: usize,
        cols: usize,
        r: isize,
        c: isize,
        heights: &[Vec<i32>],
        visited: &mut HashSet<(usize, usize)>,
        prev_height: i32,
    ) {
        if r < 0
            || r >= rows as isize
            || c < 0
            || c >= cols as isize
            || visited.contains(&(r as usize, c as usize))
            || heights[r as usize][c as usize] < prev_height
        {
            return;
        }
        let (r, c) = (r as usize, c as usize);
        visited.insert((r, c));
        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (x, y) in directions {
            let (i, j) = (r as isize + x, c as isize + y);
            dfs(rows, cols, i, j, heights, visited, heights[r][c]);
        }
    }

    for c in 0..cols {
        dfs(
            rows,
            cols,
            0,
            c as isize,
            &heights,
            &mut pacific,
            heights[0][c],
        );
        dfs(
            rows,
            cols,
            rows as isize - 1,
            c as isize,
            &heights,
            &mut atlantic,
            heights[rows - 1][c],
        );
    }

    for r in 0..rows {
        dfs(
            rows,
            cols,
            r as isize,
            0,
            &heights,
            &mut pacific,
            heights[r][0],
        );
        dfs(
            rows,
            cols,
            r as isize,
            cols as isize - 1,
            &heights,
            &mut atlantic,
            heights[r][cols - 1],
        );
    }

    for i in 0..rows {
        for j in 0..cols {
            let index = &(i, j);
            if pacific.contains(index) && atlantic.contains(index) {
                result.push(vec![i as i32, j as i32])
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
}
