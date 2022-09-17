pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    assert!(!nums.is_empty());
    let mut result = vec![0; nums.len()];
    let mut prefix = vec![0; nums.len()];
    let mut postfix = vec![0; nums.len()];
    for i in 0..nums.len() {
        if i == 0 {
            prefix[i] = nums[i];
        } else {
            prefix[i] = prefix[i - 1] * nums[i];
        }
    }

    for i in (0..nums.len()).rev() {
        if i == nums.len() - 1 {
            postfix[i] = nums[i];
        } else {
            postfix[i] = postfix[i + 1] * nums[i];
        }
    }

    for i in 0..nums.len() {
        if i == 0 {
            result[i] = postfix[i + 1];
        } else if i == nums.len() - 1 {
            result[i] = prefix[nums.len() - 2];
        } else {
            result[i] = prefix[i - 1] * postfix[i + 1];
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(
            product_except_self(vec![4, 3, 2, 1, 2]),
            vec![12, 16, 24, 48, 24]
        );
        assert_eq!(product_except_self(vec![1, 2]), vec![2, 1]);
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
        assert_eq!(
            product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
