use std::{cmp::Reverse, collections::BinaryHeap};

pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let mut capital_heap: BinaryHeap<Reverse<(i32, i32)>> = capital
        .iter()
        .zip(profits)
        .map(|(a, b)| Reverse((*a, b)))
        .collect();
    let mut w = w;
    let mut max_heap = BinaryHeap::new();

    for _ in 0..k {
        while !capital_heap.is_empty() && capital_heap.peek().unwrap().0 .0 <= w {
            max_heap.push(capital_heap.pop().unwrap().0 .1);
        }
        if max_heap.is_empty() {
            break;
        }
        let max_profit = max_heap.pop().unwrap();
        w += max_profit;
    }
    w
}

#[test]
fn test_find_maximized_capital() {
    assert_eq!(
        find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
        4
    );
    assert_eq!(
        find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]),
        6
    );
    assert_eq!(find_maximized_capital(1, 0, vec![1], vec![0]), 1);
}
