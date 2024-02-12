use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();

    let mut adj = vec![vec![]; n];

    for i in 0..n {
        let (x1, y1) = (points[i][0], points[i][1]);
        for j in i + 1..n {
            let (x2, y2) = (points[j][0], points[j][1]);
            let dist = (x1 - x2).abs() + (y1 - y2).abs();
            adj[i].push((dist, j));
            adj[j].push((dist, i));
        }
    }

    let mut result = 0;

    let mut visited = HashSet::new();
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse((0, 0)));

    while visited.len() < n {
        let (cost, i) = min_heap.pop().unwrap().0;
        if visited.contains(&i) {
            continue;
        }

        result += cost;
        visited.insert(i);
        for (nei_cost, nei) in &adj[i] {
            if !visited.contains(nei) {
                min_heap.push(Reverse((*nei_cost, *nei)))
            }
        }
    }

    result
}

#[test]
fn test_min_cost_connect_points() {
    let points = vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]];
    assert_eq!(min_cost_connect_points(points), 20);
}
