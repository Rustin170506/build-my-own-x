pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let sum = nums.iter().sum::<i32>();
    let mut left_sum = 0;
    for (i, n) in nums.iter().enumerate() {
        let right_sum = sum - left_sum - n;
        if left_sum == right_sum {
            return i as i32;
        }
        left_sum += n;
    }

    -1
}

#[cfg(test)]
mod tests {
    use crate::questions::q_0724_find_pivot_index::pivot_index;

    #[test]
    fn test_pivot_index() {
        assert_eq!(3, pivot_index(vec![1, 7, 3, 6, 5, 6]));
        assert_eq!(-1, pivot_index(vec![1, 2, 3]));
    }
}
