pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let mut result = vec![0_i32; nums.len()];
    for (index, num) in nums.iter().enumerate() {
        let new_index = (index + k as usize) % nums.len();
        result[new_index] = *num;
    }
    let _ = std::mem::replace(nums, result);
}

pub fn rotate_with_reverse(nums: &mut Vec<i32>, k: i32) {
    let k = k as usize % nums.len();
    nums.reverse();
    nums[0..k].reverse();
    nums[k..].reverse();
}

#[test]
fn test_rotate() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    rotate(&mut nums, 3);
    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);

    let mut nums = vec![-1, -100, 3, 99];
    rotate(&mut nums, 2);
    assert_eq!(nums, vec![3, 99, -1, -100]);
}
