pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0, nums.len() - 1);
    while l <= r {
        let m = l + (r - l) / 2;

        if (m as i32 - 1 < 0 || nums[m] != nums[m - 1])
            && (m + 1 > nums.len() - 1 || nums[m] != nums[m + 1])
        {
            return nums[m];
        }

        let left_size = if nums[m - 1] == nums[m] { m - 1 } else { m };

        if left_size % 2 == 0 {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    unreachable!()
}

#[test]
fn test_single_non_duplicate() {
    assert_eq!(single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
    assert_eq!(single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]), 10);
    assert_eq!(single_non_duplicate(vec![1, 1, 2, 2, 3]), 3);
}
