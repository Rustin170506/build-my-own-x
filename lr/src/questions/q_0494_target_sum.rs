use std::collections::HashMap;

pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let mut dp = HashMap::new();

    dfs(&nums, target, &mut dp, 0, 0)
}

fn dfs(
    nums: &[i32],
    target: i32,
    dp: &mut HashMap<(usize, i32), i32>,
    index: usize,
    total: i32,
) -> i32 {
    if index == nums.len() {
        return if total == target { 1 } else { 0 };
    }

    if dp.get(&(index, total)).is_some() {
        return *dp.get(&(index, total)).unwrap();
    }

    let ways = dfs(nums, target, dp, index + 1, total + nums[index])
        + dfs(nums, target, dp, index + 1, total - nums[index]);
    dp.insert((index, total), ways);

    *dp.get(&(index, total)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_target_sum_ways() {
        assert_eq!(5, find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
    }
}
