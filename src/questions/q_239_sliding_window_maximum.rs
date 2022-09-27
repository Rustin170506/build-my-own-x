use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let mut deque: VecDeque<usize> = VecDeque::new();

    let mut i: usize = 0;
    let mut j: usize = 0;

    while j < nums.len() {
        while deque.back().is_some() && nums[*deque.back().unwrap()] < nums[j] {
            deque.pop_back();
        }
        deque.push_back(j);

        if i > *deque.front().unwrap() {
            deque.pop_front();
        }

        if (j + 1) >= k as usize {
            result.push(nums[*deque.front().unwrap()]);
            i += 1;
        }
        j += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(
            max_sliding_window(vec![9, 10, 9, -7, -4, -8, 2, -6], 5),
            vec![10, 10, 9, 2]
        );
        assert_eq!(max_sliding_window(vec![3, 2, 1], 2), vec![3, 2]);
        assert_eq!(max_sliding_window(vec![1, 2], 2), vec![2]);
        assert_eq!(
            max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(max_sliding_window(vec![1], 1), vec![1]);
    }
}
