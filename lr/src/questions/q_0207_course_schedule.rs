use std::collections::{HashMap, HashSet};

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut pre_map = HashMap::new();
    for pre in prerequisites {
        pre_map
            .entry(pre[0])
            .or_insert_with(HashSet::new)
            .insert(pre[1]);
    }

    let mut visited = HashSet::new();
    fn dfs(
        course: i32,
        pre_map: &mut HashMap<i32, HashSet<i32>>,
        visited: &mut HashSet<i32>,
    ) -> bool {
        if visited.contains(&course) {
            return false;
        }
        if pre_map.get(&course).is_none() {
            return true;
        }
        visited.insert(course);
        if let Some(pres) = pre_map.get(&course).cloned() {
            for pre in pres {
                if !dfs(pre, pre_map, visited) {
                    return false;
                }
            }
        }
        visited.remove(&course);
        pre_map.get_mut(&course).unwrap().clear();
        true
    }

    for course in 0..num_courses {
        if !dfs(course, &mut pre_map, &mut visited) {
            return false;
        }
    }

    true
}

#[test]
fn test_can_finish() {
    assert!(can_finish(2, vec![vec![1, 0]]));
    assert!(!can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    assert!(can_finish(
        5,
        vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]]
    ));
}
