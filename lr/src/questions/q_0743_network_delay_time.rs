use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let mut edges: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    times.iter().for_each(|time| {
        let source = time[0];
        let target = time[1];
        let weight = time[2];
        edges
            .entry(source)
            .and_modify(|v| {
                v.push((weight, target));
            })
            .or_insert(vec![(weight, target)]);
    });

    let mut time = 0;
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse((0, k)));
    let mut visited = HashSet::new();
    while !min_heap.is_empty() {
        let (weight, node) = min_heap.pop().unwrap().0;
        if visited.contains(&node) {
            continue;
        }
        time = time.max(weight);
        visited.insert(node);

        if let Some(edges) = edges.get(&node) {
            for (neighbor_weight, neighbor) in edges {
                if visited.contains(neighbor) {
                    continue;
                }
                min_heap.push(Reverse((weight + neighbor_weight, *neighbor)));
            }
        }
    }

    if visited.len() == n as usize {
        time
    } else {
        -1
    }
}

#[test]
fn test_network_delay_time() {
    let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
    let n = 4;
    let k = 2;
    assert_eq!(network_delay_time(times, n, k), 2);
}
