pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
    if k == 1 || nums.len() <= 1 {
        return 0;
    }

    let mut nums = nums;
    nums.sort_unstable();
    let k = k as usize;

    (0..=nums.len() - k)
        .map(|i| nums[i + k - 1] - nums[i])
        .min()
        .unwrap_or(0)
}

#[test]
fn test_minimum_difference() {
    let test_cases = vec![
        (vec![90], 1, 0),
        (vec![9, 4, 1, 7], 2, 2),
    ];
    for (nums, k, expected) in test_cases {
        assert_eq!(minimum_difference(nums, k), expected);
    }
}
