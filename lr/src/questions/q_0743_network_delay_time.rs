use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let mut edges = HashMap::new();
    for time in times {
        let source = time[0];
        let target = time[1];
        let weight = time[2];
        edges
            .entry(source)
            .and_modify(|e: &mut Vec<(i32, i32)>| e.push((weight, target)))
            .or_insert(vec![(weight, target)]);
    }

    let mut min_heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
    min_heap.push(Reverse((0, k)));
    let mut visited = HashSet::new();
    let mut t = 0;
    while !min_heap.is_empty() {
        let (w1, n1) = min_heap.pop().unwrap().0;
        if visited.contains(&n1) {
            continue;
        }
        visited.insert(n1);
        t = t.max(w1);

        if let Some(edges) = edges.get(&n1) {
            for (w2, n2) in edges.iter() {
                if !visited.contains(n2) {
                    min_heap.push(Reverse((w1 + w2, *n2)));
                }
            }
        }
    }

    if visited.len() == n as usize {
        t
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
