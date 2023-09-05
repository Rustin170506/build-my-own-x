pub fn majority_element(nums: Vec<i32>) -> i32 {
    let (mut res, mut count) = (0, 0);

    for n in nums {
        if count == 0 {
            res = n;
        }

        count += if n == res { 1 } else { -1 }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_majority_element() {
        assert_eq!(2, majority_element(vec![2, 2, 2]));
        assert_eq!(2, majority_element(vec![2, 2, 1]));
        assert_eq!(1, majority_element(vec![1, 2, 1]));
    }
}
