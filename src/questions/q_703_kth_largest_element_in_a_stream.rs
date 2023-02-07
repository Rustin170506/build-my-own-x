use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: i32,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        Self {
            heap: nums.iter().map(|v| Reverse(*v)).collect::<Vec<_>>().into(),
            k,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        while self.heap.len() != self.k as usize {
            self.heap.pop();
        }

        self.heap.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_703() {
        let mut kth_largest = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth_largest.add(3), 4);
        assert_eq!(kth_largest.add(5), 5);
        assert_eq!(kth_largest.add(10), 5);
        assert_eq!(kth_largest.add(9), 8);
        assert_eq!(kth_largest.add(4), 8);
    }
}
