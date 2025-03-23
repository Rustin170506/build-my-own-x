use std::{cmp::Reverse, collections::BinaryHeap};

pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let mut trips = trips;
    trips.sort_by(|a, b| a[1].cmp(&b[1]));
    let mut ending_heap = BinaryHeap::new();
    let mut current = 0;
    for trip in trips {
        while !ending_heap.is_empty() {
            let temp: Reverse<(i32, i32)> = ending_heap.pop().unwrap();
            if temp.0 .0 > trip[1] {
                ending_heap.push(temp);
                break;
            } else {
                current -= temp.0 .1;
            }
        }
        if current + trip[0] <= capacity {
            current += trip[0];
            ending_heap.push(Reverse((trip[2], trip[0])));
        } else {
            return false;
        }
    }

    true
}

#[test]
fn test_car_pooling() {
    assert!(!car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 4));
    assert!(car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5));
    assert!(car_pooling(vec![vec![2, 1, 5], vec![3, 5, 7]], 3));
}
