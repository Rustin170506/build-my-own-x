pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut goal_pos = (nums.len() - 1) as i32;

    for i in (0..nums.len() - 1).rev() {
        if i as i32 + nums[i] >= goal_pos {
            goal_pos = i as i32;
        }
    }

    goal_pos == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_055() {
        assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false);
    }
}
