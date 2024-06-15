pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut red_count = 0;
    let mut white_count = 0;
    let mut blue_count = 0;

    for num in nums.iter() {
        match num {
            0 => red_count += 1,
            1 => white_count += 1,
            2 => blue_count += 1,
            _ => unreachable!(),
        }
    }

    for index in 0..red_count {
        nums[index] = 0
    }
    let new_index = red_count + white_count;
    for index in red_count..new_index {
        nums[index] = 1
    }
    for index in new_index..new_index + blue_count {
        nums[index] = 2
    }
}

#[test]
fn test_sort_colors() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut nums);
    assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
}
