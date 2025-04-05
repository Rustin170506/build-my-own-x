pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
    let sum: i32 = nums.iter().sum();

    if sum % k != 0 {
        return false;
    }

    let target = sum / k;
    let mut used = vec![false; nums.len()];

    fn backtrack(
        nums: &[i32],
        used: &mut Vec<bool>,
        target: i32,
        i: usize,
        k: i32,
        current_sum: i32,
    ) -> bool {
        if k == 0 {
            return true;
        }
        if current_sum == target {
            return backtrack(nums, used, target, 0, k - 1, 0);
        }

        for j in i..nums.len() {
            if used[j] || current_sum + nums[j] > target {
                continue;
            }
            used[j] = true;
            if backtrack(nums, used, target, j + 1, k, current_sum + nums[j]) {
                return true;
            }
            used[j] = false;
        }

        false
    }

    backtrack(&nums, &mut used, target, 0, k, 0)
}

#[test]
fn test() {
    assert!(can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4));
    assert!(!can_partition_k_subsets(vec![1, 2, 3, 4], 3));
    assert!(!can_partition_k_subsets(vec![1, 1, 2, 2, 2], 3));
}
