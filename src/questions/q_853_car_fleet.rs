use std::collections::VecDeque;

pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    assert!(position.len() == speed.len());
    let mut stack = VecDeque::new();
    let mut pair = Vec::new();
    for i in 0..position.len() {
        pair.push((position[i], speed[i]));
    }
    pair.sort();

    for p in pair.iter().rev() {
        let cost: f64 = (target - p.0) as f64 / p.1 as f64;
        stack.push_back(cost);
        if stack.len() >= 2 && stack.back().unwrap() <= stack.get(stack.len() - 2).unwrap() {
            stack.pop_back();
        }
    }

    stack.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_fleet() {
        assert_eq!(car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3],), 3);
        assert_eq!(car_fleet(10, vec![3], vec![3],), 1);
    }
}
