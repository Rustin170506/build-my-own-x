pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    assert!(!weights.is_empty());
    assert!(days > 0);
    let mut left = *weights.iter().max().unwrap();
    let mut right = weights.iter().sum();
    let mut result = i32::MAX;
    while left <= right {
        let middle = (left + right) / 2;
        let mut cap = middle;
        let mut remain_days = days - 1;
        for weight in &weights {
            if cap >= *weight {
                cap -= weight;
            } else {
                remain_days -= 1;
                cap = middle - weight;
            }
        }
        if remain_days >= 0 {
            result = result.min(middle);
            right = middle - 1;
        } else {
            left = middle + 1;
        }
    }

    result
}

#[test]
fn test_ship_within_days() {
    let weights = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let days = 5;
    let result = ship_within_days(weights, days);
    assert_eq!(result, 15);
}
