use std::collections::{HashSet, VecDeque};

pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let mut visited: HashSet<String> = deadends.into_iter().collect();
    if visited.contains("0000") {
        return -1;
    }

    let mut queue = VecDeque::new();
    queue.push_back(("0000".to_string(), 0));

    while !queue.is_empty() {
        let (lock, turns) = queue.pop_front().unwrap();
        if lock == target {
            return turns;
        }
        for child in children(&lock) {
            if !visited.contains(&child) {
                visited.insert(child.clone());
                queue.push_back((child, turns + 1));
            }
        }
    }

    -1
}

fn children(lock: &str) -> Vec<String> {
    let mut result = vec![];
    for (index, char) in lock.chars().enumerate() {
        let mut tmp = lock.chars().collect::<Vec<char>>();
        let num = char.to_digit(10).unwrap();
        tmp[index] = std::char::from_digit((num + 1) % 10, 10).unwrap();
        result.push(tmp.iter().collect());
        tmp[index] = std::char::from_digit((num + 9) % 10, 10).unwrap();
        result.push(tmp.iter().collect());
    }

    result
}

#[test]
fn test() {
    let deadends = vec![
        "0201".to_string(),
        "0101".to_string(),
        "0102".to_string(),
        "1212".to_string(),
        "2002".to_string(),
    ];
    let target = "0202".to_string();
    assert_eq!(open_lock(deadends, target), 6);
}
