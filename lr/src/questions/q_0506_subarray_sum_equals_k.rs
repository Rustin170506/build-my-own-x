pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut result = 0;
    let mut sum = 0;
    let mut map = std::collections::HashMap::new();
    map.insert(0, 1);
    for num in &nums {
        sum += num;
        if let Some(&v) = map.get(&(sum - k)) {
            result += v;
        }
        *map.entry(sum).or_insert(0) += 1;
    }

    result
}

#[test]
fn test_subarray_sum() {
    let nums = vec![1, 1, 1];
    let k = 2;
    assert_eq!(subarray_sum(nums, k), 2);
}
