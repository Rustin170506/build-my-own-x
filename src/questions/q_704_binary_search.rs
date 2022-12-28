pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut result = -1;
    let mut left = 0_i32;
    let mut right = (nums.len() - 1) as i32;
    while left <= right {
        let mid = (left + right) / 2;
        match nums[mid as usize].cmp(&target) {
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Equal => {
                result = mid;
                break;
            }
            std::cmp::Ordering::Greater => right = mid - 1,
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
        assert_eq!(search(vec![5], 5), 0);
        assert_eq!(search(vec![5], -5), -1);
        assert_eq!(search(vec![2, 5], 5), 1);
    }
}
