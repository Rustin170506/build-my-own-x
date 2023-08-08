use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq)]
struct MinInterval(i32, i32);

impl Eq for MinInterval {}

impl PartialOrd for MinInterval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

impl Ord for MinInterval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut result = HashMap::new();
    let mut i = 0;
    let mut min_heap = BinaryHeap::new();

    let mut queries_copy = queries.clone();
    queries_copy.sort();

    for q in queries_copy {
        while i < intervals.len() && intervals[i][0] <= q {
            let (l, r) = (intervals[i][0], intervals[i][1]);
            min_heap.push(MinInterval(r - l + 1, r));
            i += 1;
        }

        while !min_heap.is_empty() && min_heap.peek().unwrap().1 < q {
            min_heap.pop();
        }

        let r = {
            if !min_heap.is_empty() {
                min_heap.peek().unwrap().0
            } else {
                -1
            }
        };
        result.insert(q, r);
    }

    queries.iter().map(|q| *result.get(q).unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_interval() {
        assert_eq!(
            min_interval(
                vec![vec![1, 4], vec![2, 4], vec![3, 6], vec![4, 4]],
                vec![2, 3, 4, 5]
            ),
            vec![3, 3, 1, 4]
        );
    }
}
