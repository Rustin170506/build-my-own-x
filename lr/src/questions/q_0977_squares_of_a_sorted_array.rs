pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return nums;
    }

    (0..nums.len())
        .scan((0, nums.len() - 1), |(left, right), _| {
            if *left > *right {
                return None;
            }

            if nums[*left].abs() > nums[*right].abs() {
                let val = nums[*left] * nums[*left];
                *left += 1;
                Some(val)
            } else {
                let val = nums[*right] * nums[*right];
                *right -= 1;
                Some(val)
            }
        })
        .collect::<Vec<i32>>()
        .into_iter()
        .rev()
        .collect()
}

#[test]
fn test_sorted_squares() {
    let nums = vec![-4, -1, 0, 3, 10];
    assert_eq!(sorted_squares(nums), vec![0, 1, 9, 16, 100]);
    let nums = vec![-7, -3, 2, 3, 11];
    assert_eq!(sorted_squares(nums), vec![4, 9, 9, 49, 121]);
}
