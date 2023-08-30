use std::collections::BinaryHeap;

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap = BinaryHeap::from(nums);

    for _ in 0..k - 1 {
        heap.pop().unwrap();
    }

    heap.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_largest() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;
        let expected = 5;
        assert_eq!(find_kth_largest(nums, k), expected);

        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let k = 4;
        let expected = 4;
        assert_eq!(find_kth_largest(nums, k), expected);
    }
}
