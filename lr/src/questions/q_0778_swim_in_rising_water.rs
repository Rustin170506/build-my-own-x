use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len() as i32;
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse((grid[0][0], 0, 0)));
    let directions: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    while !min_heap.is_empty() {
        let (t, r, c) = min_heap.pop().unwrap().0;
        if r == n - 1 && c == n - 1 {
            return t;
        }

        for (dr, dc) in &directions {
            let (nei_r, nei_c) = (r + dr, c + dc);
            if nei_r < 0
                || nei_c < 0
                || nei_r == n
                || nei_c == n
                || visited.contains(&(nei_r, nei_c))
            {
                continue;
            }
            visited.insert((nei_r, nei_c));
            min_heap.push(Reverse((
                t.max(grid[nei_r as usize][nei_c as usize]),
                nei_r,
                nei_c,
            )));
        }
    }

    unreachable!()
}

#[test]
fn test_swim_in_water() {
    let grid = vec![vec![0, 2], vec![1, 3]];
    assert_eq!(swim_in_water(grid), 3);
}
