use std::{cmp::Reverse, collections::BinaryHeap};

pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
    let mut meetings = meetings;
    meetings.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut available: BinaryHeap<Reverse<i32>> = (0..n).map(Reverse).collect();
    let mut used: BinaryHeap<Reverse<(i64, i32)>> = BinaryHeap::new(); // Changed to i64 for time
    let mut count = vec![0; n as usize];

    for m in meetings {
        let start = m[0] as i64; // Convert to i64 to handle larger time values
        let duration = m[1] - m[0];

        // Release rooms that have finished
        while let Some(&Reverse((end_time, room))) = used.peek() {
            if start >= end_time {
                used.pop();
                available.push(Reverse(room));
            } else {
                break;
            }
        }

        if available.is_empty() {
            // All rooms are occupied, use the one that will be available first
            let Reverse((end_time, room)) = used.pop().unwrap();
            used.push(Reverse((end_time + duration as i64, room)));
            count[room as usize] += 1;
        } else {
            // Use an available room
            let Reverse(room) = available.pop().unwrap();
            used.push(Reverse((start + duration as i64, room)));
            count[room as usize] += 1;
        }
    }

    count
        .iter()
        .enumerate()
        .max_by_key(|&(i, &count)| (count, -(i as i32))) // Break ties by smaller room number
        .unwrap()
        .0 as i32
}

#[test]
fn test_most_booked() {
    assert_eq!(
        most_booked(2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]]),
        0
    );
    assert_eq!(
        most_booked(
            3,
            vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]]
        ),
        1
    );
    assert_eq!(
        most_booked(2, vec![vec![0, 10], vec![1, 2], vec![2, 3], vec![3, 4]]),
        1
    );
}
