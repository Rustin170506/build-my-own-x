use std::collections::{HashMap, HashSet};

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut pre_map: HashMap<i32, Vec<i32>> = (0..num_courses).map(|i| (i, vec![])).collect();
    for pre in prerequisites {
        pre_map.entry(pre[0]).or_insert(vec![]).push(pre[1]);
    }

    let mut visited = HashSet::new();

    for c in 0..num_courses {
        if !dfs(c, &mut pre_map, &mut visited) {
            return false;
        }
    }

    true
}

fn dfs(c: i32, pre_map: &mut HashMap<i32, Vec<i32>>, visited: &mut HashSet<i32>) -> bool {
    if visited.contains(&c) {
        return false;
    }

    if pre_map.get(&c).unwrap().is_empty() {
        return true;
    }

    visited.insert(c);
    let pres = pre_map.get(&c).unwrap().clone();
    for pre in pres {
        if !dfs(pre, pre_map, visited) {
            return false;
        }
    }
    visited.remove(&c);
    *pre_map.get_mut(&c).unwrap() = vec![];

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_finish() {
        assert!(can_finish(2, vec![vec![1, 0]]));
        assert!(!can_finish(2, vec![vec![1, 0], vec![0, 1]]));
        assert!(can_finish(
            5,
            vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]]
        ));
    }
}
