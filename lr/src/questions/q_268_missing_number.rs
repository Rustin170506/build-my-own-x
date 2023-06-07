pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut res = (nums.len()) as i32;

    for (i, n) in nums.iter().enumerate() {
        res += (i as i32) - n;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_number() {
        assert_eq!(missing_number(vec![0, 1, 3]), 2);
        assert_eq!(missing_number(vec![0, 1, 2]), 3);
    }
}
