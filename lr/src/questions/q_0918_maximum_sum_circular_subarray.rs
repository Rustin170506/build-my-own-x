pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let (mut global_max, mut global_min) = (nums[0], nums[0]);
    let (mut current_max, mut current_min) = (0, 0);
    let mut total = 0;

    for n in nums {
        current_max = i32::max(n, current_max + n);
        current_min = i32::min(n, current_min + n);
        total += n;
        global_max = global_max.max(current_max);
        global_min = global_min.min(current_min);
    }

    if global_max > 0 {
        global_max.max(total - global_min)
    } else {
        global_max
    }
}

#[test]
fn test_max_subarray_sum_circular() {
    let test_cases = vec![
        (vec![1, -2, 3, -2], 3),
        (vec![5, -3, 5], 10),
        (vec![3, -1, 2, -1], 4),
        (vec![3, -2, 2, -3], 3),
        (vec![-2, -3, -1], -1),
    ];
    for (nums, expected) in test_cases {
        assert_eq!(max_subarray_sum_circular(nums), expected);
    }
}
