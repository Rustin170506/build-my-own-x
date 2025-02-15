pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let (mut l, mut total) = (0, 0);
    let mut result = i32::MAX;
    for r in 0..nums.len() {
        total += nums[r];
        while total >= target {
            result = result.min((r - l) as i32 + 1);
            total -= nums[l];
            l += 1;
        }
    }

    if result == i32::MAX {
        0
    } else {
        result
    }
}

#[test]
fn test_min_sub_array_len() {
    let nums = vec![2, 3, 1, 2, 4, 3];
    let target = 7;
    assert_eq!(min_sub_array_len(target, nums), 2);
    let nums = vec![1, 4, 4];
    let target = 4;
    assert_eq!(min_sub_array_len(target, nums), 1);
    let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
    let target = 11;
    assert_eq!(min_sub_array_len(target, nums), 0);
}
