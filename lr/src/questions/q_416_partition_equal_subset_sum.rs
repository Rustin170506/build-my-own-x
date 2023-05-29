use std::collections::HashSet;

pub fn can_partition(nums: Vec<i32>) -> bool {
    if nums.iter().sum::<i32>() % 2 == 1 {
        return false;
    }

    let mut dp = HashSet::new();
    dp.insert(0);
    let target: i32 = nums.iter().sum::<i32>() / 2;

    for i in (0..nums.len()).rev() {
        let mut new_dp = HashSet::new();
        for t in dp {
            if t + nums[i] == target {
                return true;
            }
            new_dp.insert(t + nums[i]);
            new_dp.insert(t);
        }

        dp = new_dp;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_partition() {
        assert_eq!(can_partition(vec![1, 5, 11, 5]), true);
        assert_eq!(can_partition(vec![1, 2, 3, 5]), false);
    }
}
