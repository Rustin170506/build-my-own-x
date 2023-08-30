pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }

    let (mut start, mut end) = (0, nums.len() - 1);

    while start <= end {
        let mid = (start + end) / 2;
        match nums[mid].cmp(&target) {
            std::cmp::Ordering::Less => start = mid + 1,
            std::cmp::Ordering::Equal => return mid as i32,
            std::cmp::Ordering::Greater => {
                if mid == 0 {
                    return mid as i32;
                }
                end = mid - 1;
            }
        }
    }
    start as i32
}

#[cfg(test)]
mod tests {
    use super::search_insert;

    #[test]
    fn test_search_insert() {
        assert_eq!(search_insert(vec![], 0), -1);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 0), 0);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
