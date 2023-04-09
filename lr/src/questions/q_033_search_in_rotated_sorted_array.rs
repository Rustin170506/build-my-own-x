pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len() - 1);

    while left <= right {
        let mid = (left + right) / 2;
        if target == nums[mid] {
            return mid as i32;
        }

        // left sorted portion
        if nums[left] <= nums[mid] {
            if target > nums[mid] || target < nums[left] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        } else if target < nums[mid] || target > nums[right] {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(search(vec![1], 3), -1);
    }
}
