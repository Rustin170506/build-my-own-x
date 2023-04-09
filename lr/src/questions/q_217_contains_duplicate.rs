use std::collections::{hash_map::Entry, HashMap};

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        return false;
    }

    let mut map = HashMap::new();

    for n in nums {
        if let Entry::Vacant(e) = map.entry(n) {
            e.insert(0);
        } else {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert!(!contains_duplicate(vec![1]));
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
        assert!(!contains_duplicate(vec![1, 2, 3, 4]));
        assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    }
}
