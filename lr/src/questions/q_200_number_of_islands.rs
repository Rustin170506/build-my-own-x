use std::collections::{HashSet, VecDeque};

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() {
        return 0;
    }

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut visited = HashSet::new();
    let mut islands = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '1' && !visited.contains(&(r as isize, c as isize)) {
                bfs(
                    r as isize,
                    c as isize,
                    &grid,
                    &mut visited,
                    rows as isize,
                    cols as isize,
                );
                islands += 1;
            }
        }
    }

    islands
}

fn bfs(
    r: isize,
    c: isize,
    grid: &[Vec<char>],
    visited: &mut HashSet<(isize, isize)>,
    rows: isize,
    cols: isize,
) {
    let mut q = VecDeque::new();
    visited.insert((r, c));
    q.push_back((r, c));

    while !q.is_empty() {
        let (row, col) = q.pop_front().unwrap();
        let directions: Vec<(isize, isize)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dr, dc) in directions {
            let (r, c) = (row + dr, col + dc);
            if (r >= 0 && r < rows)
                && (c >= 0 && c < cols)
                && grid[r as usize][c as usize] == '1'
                && !visited.contains(&(r, c))
            {
                q.push_back((r, c));
                visited.insert((r, c));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
