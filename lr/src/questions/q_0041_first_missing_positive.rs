pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    for num in &mut nums {
        if *num < 0 {
            *num = 0;
        }
    }

    for i in 0..nums.len() {
        let val = nums[i].abs();
        if 1 <= val && val <= nums.len() as i32 {
            let val = val as usize;
            match nums[val - 1].cmp(&0) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => {
                    nums[val - 1] = -(nums.len() as i32 + 1);
                }
                std::cmp::Ordering::Greater => {
                    nums[val - 1] = -nums[val - 1];
                }
            }
        }
    }

    for i in 1..nums.len() + 1 {
        if nums[i - 1] >= 0 {
            return i as i32;
        }
    }

    nums.len() as i32 + 1
}

#[test]
fn test_first_missing_positive() {
    let nums = vec![1, 2, 0];
    assert_eq!(first_missing_positive(nums), 3);

    let nums = vec![3, 4, -1, 1];
    assert_eq!(first_missing_positive(nums), 2);

    let nums = vec![7, 8, 9, 11, 12];
    assert_eq!(first_missing_positive(nums), 1);
}
