pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut result = Vec::new();
    for i in 0..nums.len() {
        let index = (nums[i].abs() - 1) as usize;
        nums[index] = -nums[index].abs();
    }
    for (i, item) in nums.iter().enumerate() {
        if *item > 0 {
            result.push(i as i32 + 1);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_disappeared_numbers() {
        assert_eq!(
            find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
    }
}
