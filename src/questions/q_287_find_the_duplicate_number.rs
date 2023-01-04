pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let (mut slow, mut fast) = (0_i32, 0_i32);
    loop {
        slow = nums[slow as usize];
        fast = nums[nums[fast as usize] as usize];
        if slow == fast {
            break;
        }
    }

    let mut slow2 = 0_i32;
    loop {
        slow = nums[slow as usize];
        slow2 = nums[slow2 as usize];
        if slow2 == slow {
            return slow;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate() {
        assert_eq!(find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }
}
