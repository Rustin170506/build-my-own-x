use std::collections::HashSet;

pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (rows, cols) = (heights.len(), heights[0].len());
    let (mut pac, mut atl) = (HashSet::new(), HashSet::new());
    let mut result = vec![];
    for c in 0..cols {
        dfs(
            rows as isize,
            cols as isize,
            &heights,
            0,
            c as isize,
            &mut pac,
            heights[0][c],
        );
        dfs(
            rows as isize,
            cols as isize,
            &heights,
            (rows - 1) as isize,
            c as isize,
            &mut atl,
            heights[rows - 1][c],
        );
    }

    for r in 0..rows {
        dfs(
            rows as isize,
            cols as isize,
            &heights,
            r as isize,
            0,
            &mut pac,
            heights[r][0],
        );
        dfs(
            rows as isize,
            cols as isize,
            &heights,
            r as isize,
            (cols - 1) as isize,
            &mut atl,
            heights[r][cols - 1],
        );
    }

    for r in 0..rows {
        for c in 0..cols {
            if pac.contains(&(r as isize, c as isize)) && atl.contains(&(r as isize, c as isize)) {
                result.push(vec![r as i32, c as i32]);
            }
        }
    }

    result
}

fn dfs(
    rows: isize,
    cols: isize,
    heights: &[Vec<i32>],
    r: isize,
    c: isize,
    visited: &mut HashSet<(isize, isize)>,
    prev_height: i32,
) {
    if visited.contains(&(r, c))
        || r < 0
        || c < 0
        || r == rows
        || c == cols
        || heights[r as usize][c as usize] < prev_height
    {
        return;
    }

    visited.insert((r, c));
    dfs(
        rows,
        cols,
        heights,
        r + 1,
        c,
        visited,
        heights[r as usize][c as usize],
    );
    dfs(
        rows,
        cols,
        heights,
        r - 1,
        c,
        visited,
        heights[r as usize][c as usize],
    );
    dfs(
        rows,
        cols,
        heights,
        r,
        c + 1,
        visited,
        heights[r as usize][c as usize],
    );
    dfs(
        rows,
        cols,
        heights,
        r,
        c - 1,
        visited,
        heights[r as usize][c as usize],
    );
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
