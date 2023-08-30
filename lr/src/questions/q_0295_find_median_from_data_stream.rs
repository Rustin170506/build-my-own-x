use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    small_heap: BinaryHeap<i32>,
    large_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            small_heap: BinaryHeap::new(),
            large_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.small_heap.push(num);
        // Make sure every num small is <= every num in large.
        if !self.small_heap.is_empty()
            && !self.large_heap.is_empty()
            && *self.small_heap.peek().unwrap() > self.large_heap.peek().unwrap().0
        {
            let item = self.small_heap.pop().unwrap();
            self.large_heap.push(Reverse(item));
        }

        if self.small_heap.len() > self.large_heap.len() + 1 {
            let item = self.small_heap.pop().unwrap();
            self.large_heap.push(Reverse(item));
        }

        if self.large_heap.len() > self.small_heap.len() + 1 {
            let item = self.large_heap.pop().unwrap().0;
            self.small_heap.push(item);
        }
    }

    fn find_median(&self) -> f64 {
        if self.small_heap.len() > self.large_heap.len() {
            return *self.small_heap.peek().unwrap() as f64;
        }

        if self.large_heap.len() > self.small_heap.len() {
            return self.large_heap.peek().unwrap().0 as f64;
        }

        (*self.small_heap.peek().unwrap() + self.large_heap.peek().unwrap().0) as f64 / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_295() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(1);
        median_finder.add_num(2);
        assert_eq!(1.5, median_finder.find_median());
        median_finder.add_num(3);
        assert_eq!(2.0, median_finder.find_median());
    }
}
