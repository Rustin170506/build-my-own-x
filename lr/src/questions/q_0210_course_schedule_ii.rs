use std::collections::{HashMap, HashSet};

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut pre_map: HashMap<i32, Vec<i32>> = (0..num_courses).map(|i| (i, vec![])).collect();
    for pre in prerequisites {
        pre_map.entry(pre[0]).or_default().push(pre[1]);
    }
    let mut res = vec![];

    let (mut visited, mut cycle) = (HashSet::new(), HashSet::new());

    for i in 0..num_courses {
        if !dfs(i, &mut res, &mut visited, &mut cycle, &pre_map) {
            return vec![];
        }
    }

    res
}

fn dfs(
    c: i32,
    res: &mut Vec<i32>,
    visited: &mut HashSet<i32>,
    cycle: &mut HashSet<i32>,
    pre_req: &HashMap<i32, Vec<i32>>,
) -> bool {
    if cycle.contains(&c) {
        return false;
    }

    if visited.contains(&c) {
        return true;
    }

    cycle.insert(c);
    for pre_c in pre_req.get(&c).unwrap() {
        if !dfs(*pre_c, res, visited, cycle, pre_req) {
            return false;
        }
    }

    cycle.remove(&c);
    visited.insert(c);
    res.push(c);
    true
}

#[test]
fn test_find_order() {
    assert_eq!(find_order(2, vec![vec![1, 0]]), vec![0, 1]);
    assert_eq!(
        find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
        vec![0, 1, 2, 3]
    );
    assert_eq!(find_order(1, vec![]), vec![0]);
}
