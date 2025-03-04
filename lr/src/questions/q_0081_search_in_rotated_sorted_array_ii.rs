pub fn search(nums: Vec<i32>, target: i32) -> bool {
    if nums.is_empty() {
        return false;
    }

    let (mut left, mut right) = (0, nums.len() - 1);
    while left <= right {
        let middle = left + (right - left) / 2;

        if nums[middle] == target {
            return true;
        }

        // If we can't determine which half is sorted due to duplicates
        if nums[left] == nums[middle] && nums[middle] == nums[right] {
            left += 1;
            right = right.saturating_sub(1);
            continue;
        }

        // If left half is sorted
        if nums[left] <= nums[middle] {
            if nums[left] <= target && target < nums[middle] {
                right = middle.saturating_sub(1);
            } else {
                left = middle + 1;
            }
        }
        // If right half is sorted
        else if nums[middle] < target && target <= nums[right] {
            left = middle + 1;
        } else {
            right = middle.saturating_sub(1);
        }
    }

    false
}

#[test]
fn test_search() {
    assert!(search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    assert!(!search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    assert!(search(vec![1, 0, 1, 1, 1], 0));
}
