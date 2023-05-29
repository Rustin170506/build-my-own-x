pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut result = vec![1; nums.len()];

    for i in (0..nums.len()).rev() {
        for j in i + 1..nums.len() {
            if nums[i] < nums[j] {
                result[i] = result[i].max(1 + result[j]);
            }
        }
    }

    *result.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lis() {
        assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(length_of_lis(vec![7, 7, 7, 7, 7, 7]), 1);
    }
}
