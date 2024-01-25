use std::cmp::Ordering;

pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for i in 0..intervals.len() {
        let interval = &intervals[i];
        let mut covered = false;
        for j in 0..intervals.len() {
            if i == j {
                continue;
            }
            let temp = &intervals[j];
            if temp[0] <= interval[0] && temp[1] >= interval[1] {
                covered = true;
                break;
            }
        }
        if !covered {
            result += 1;
        }
    }
    return result;
}

pub fn remove_covered_intervals_v2(intervals: Vec<Vec<i32>>) -> i32 {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| {
        if a[0] < b[0] {
            Ordering::Less
        } else if a[0] > b[0] {
            Ordering::Greater
        } else if a[1] < b[1] {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    let mut count = 0;
    let mut end = std::i32::MIN;
    for interval in intervals {
        if interval[1] > end {
            end = interval[1];
            count += 1;
        }
    }

    count
}

#[test]
fn test_remove_covered_intervals() {
    assert_eq!(
        remove_covered_intervals(vec![vec![1, 4], vec![3, 6], vec![2, 8]]),
        2
    );
    assert_eq!(remove_covered_intervals(vec![vec![1, 4], vec![2, 3]]), 1);
    assert_eq!(
        remove_covered_intervals_v2(vec![vec![1, 4], vec![3, 6], vec![2, 8]]),
        2
    );
    assert_eq!(remove_covered_intervals_v2(vec![vec![1, 4], vec![2, 3]]), 1);
}
