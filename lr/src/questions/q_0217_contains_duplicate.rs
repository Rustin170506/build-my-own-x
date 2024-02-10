use std::collections::{hash_map::Entry, BTreeSet, HashMap};

/// Use hashmap to detect duplication.
/// Time Complicity: O(n)
/// Memory Complicity: O(n)
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    assert!(!nums.is_empty() && nums.len() <= usize::pow(10, 5));
    if nums.len() == 1 {
        return false;
    }

    let mut map = HashMap::new();
    for n in nums {
        assert!(n >= i32::pow(-10, 9) && n <= i32::pow(10, 9));
        if let Entry::Vacant(e) = map.entry(n) {
            e.insert(0);
        } else {
            return true;
        }
    }

    false
}

/// Sort the array and then checks for duplicates by comparing each pair of neighboring elements.
/// Time Complicity: O(n * logn)
/// Memory Complicity: O(n)
pub fn contains_duplicate_sort(nums: Vec<i32>) -> bool {
    assert!(!nums.is_empty() && nums.len() <= usize::pow(10, 5));
    if nums.len() == 1 {
        return false;
    }
    let mut nums = nums;
    nums.sort();

    for i in 0..nums.len() - 1 {
        assert!(nums[i] >= i32::pow(-10, 9) && nums[i] <= i32::pow(10, 9));
        assert!(nums[i + 1] >= i32::pow(-10, 9) && nums[i + 1] <= i32::pow(10, 9));
        if nums[i] == nums[i + 1] {
            return true;
        }
    }

    false
}

/// Use set to detect duplication.
/// Time Complicity: O(n)
/// Memory Complicity: O(n)
pub fn contains_duplicate_set(nums: Vec<i32>) -> bool {
    assert!(!nums.is_empty() && nums.len() <= usize::pow(10, 5));
    if nums.len() == 1 {
        return false;
    }

    let nums_len = nums.len();
    let mut nums_set: BTreeSet<i32> = BTreeSet::new();
    nums_set.extend(nums);

    nums_len != nums_set.len()
}

#[test]
fn test_contains_duplicate() {
    assert!(!contains_duplicate(vec![1]));
    assert!(contains_duplicate(vec![1, 2, 3, 1]));
    assert!(!contains_duplicate(vec![1, 2, 3, 4]));
    assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    assert!(contains_duplicate_sort(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    assert!(contains_duplicate_set(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]))
}
