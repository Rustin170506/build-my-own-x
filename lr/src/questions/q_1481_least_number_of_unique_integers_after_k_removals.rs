use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
    let mut counter = HashMap::new();
    for num in arr {
        *counter.entry(num).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::new();
    for &count in counter.values() {
        heap.push(Reverse(count));
    }

    let mut k = k;
    while k > 0 {
        if let Some(Reverse(smallest)) = heap.pop() {
            if k >= smallest {
                k -= smallest;
            } else {
                heap.push(Reverse(smallest));
                break;
            }
        }
    }

    heap.len() as i32
}

#[test]
fn test_find_least_num_of_unique_ints() {
    assert_eq!(find_least_num_of_unique_ints(vec![5, 5, 4], 1), 1);
    assert_eq!(
        find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3),
        2
    );
}
