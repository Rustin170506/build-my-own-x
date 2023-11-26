pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let (mut i, mut j) = (0, nums.len() - 1);
    let (mut i_count, mut j_count) = (1, 1);
    while i < j {
        if j - 1 > 0 && nums[j - 1] == nums[j] {
            j_count += 1;
            j -= 1;
        }
        if i + 1 < (nums.len() - 1) && nums[i + 1] == nums[i] {
            i_count += 1;
            i += 1;
        }
        if j_count == 2 {
            j_count = 1;
            j -= 1;
        }
        if i_count == 2 {
            i_count = 1;
            i += 1;
        }
    }

    nums[i]
}

#[test]
fn test_single_non_duplicate() {
    assert_eq!(single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
    assert_eq!(single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]), 10);
}
