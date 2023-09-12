use std::collections::HashSet;

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut window = HashSet::new();
    let mut l = 0;

    for r in 0..nums.len() {
        if (r - l) as i32 > k {
            window.remove(&nums[l]);
            l += 1;
        }
        if window.contains(&nums[r]) {
            return true;
        }
        window.insert(nums[r]);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_nearby_duplicate() {
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true);
        assert_eq!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
    }
}
