pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    dfs(&nums, 0, 0, &mut sum);
    sum
}

fn dfs(nums: &[i32], index: usize, pre_sum: i32, sum: &mut i32) {
    if index >= nums.len() {
        return;
    }

    for i in index..nums.len() {
        let current_sum = pre_sum ^ nums[i];
        *sum += current_sum;
        dfs(nums, i + 1, current_sum, sum);
    }
}

#[test]
fn test_subset_xor_sum() {
    assert_eq!(subset_xor_sum(vec![1, 3]), 6);
    assert_eq!(subset_xor_sum(vec![5, 1, 6]), 28);
    assert_eq!(subset_xor_sum(vec![3, 4, 5, 6, 7, 8]), 480);
}
