pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; nums.len() * 2];
    for (i, n) in nums.iter().enumerate() {
        res[i] = *n;
        res[i + nums.len()] = *n;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_concatenation() {
        assert_eq!(vec![1, 2, 1, 1, 2, 1], get_concatenation(vec![1, 2, 1]));
        assert_eq!(vec![1, 2, 3, 1, 2, 3], get_concatenation(vec![1, 2, 3]));
    }
}
