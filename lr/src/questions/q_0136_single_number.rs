pub fn single_number(nums: Vec<i32>) -> i32 {
    assert!(!nums.is_empty());
    if nums.len() == 1 {
        return nums[0];
    }
    let mut result = 0;
    for n in nums {
        result ^= n;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(single_number(vec![1]), 1);
        assert_eq!(single_number(vec![2, 2, 1]), 1);
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    }
}
