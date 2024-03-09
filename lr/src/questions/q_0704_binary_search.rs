pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut res = -1;
    let (mut left, mut right) = (0, nums.len() - 1);
    while left <= right {
        let m = (left + right) / 2;
        match nums[m].cmp(&target) {
            std::cmp::Ordering::Less => {
                left = m + 1;
            }
            std::cmp::Ordering::Equal => {
                res = m as i32;
                break;
            }
            std::cmp::Ordering::Greater => {
                right = m - 1;
            }
        }
    }

    res
}

pub fn search_recursive(nums: Vec<i32>, target: i32) -> i32 {
    fn helper(nums: &[i32], target: i32, left: usize, right: usize) -> i32 {
        if left > right {
            return -1;
        }
        let m = left + (right - left) / 2;
        match nums[m].cmp(&target) {
            std::cmp::Ordering::Less => helper(nums, target, m + 1, right),
            std::cmp::Ordering::Equal => m as i32,
            std::cmp::Ordering::Greater => helper(nums, target, left, m - 1),
        }
    }

    helper(&nums, target, 0, nums.len() - 1)
}

#[test]
fn test_search() {
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    assert_eq!(search_recursive(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(search_recursive(vec![-1, 0, 3, 5, 9, 12], 2), -1);
}
