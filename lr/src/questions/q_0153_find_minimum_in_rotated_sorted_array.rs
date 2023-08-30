pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut result = nums[0];
    let (mut left, mut right) = (0, nums.len() - 1);
    while left <= right {
        if nums[left] < nums[right] {
            result = i32::min(result, nums[left]);
        }

        let mid = (left + right) / 2;
        result = i32::min(result, nums[mid]);
        if nums[mid] >= nums[left] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min() {
        assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(find_min(vec![11, 13, 15, 17]), 11);
        assert_eq!(find_min(vec![2, 1]), 1);
    }
}
