pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    assert!(gas.len() == cost.len());
    if gas.iter().sum::<i32>() != cost.iter().sum() {
        return -1;
    }

    let mut result = 0;
    let mut total = 0;

    for i in 0..gas.len() {
        total += gas[i] - cost[i];

        if total < 0 {
            total = 0;
            result = (i + 1) as i32;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_134() {
        assert_eq!(
            can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
        assert_eq!(can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
    }
}
