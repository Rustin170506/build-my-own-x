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

#[test]
fn test_remove_covered_intervals() {
    assert_eq!(
        remove_covered_intervals(vec![vec![1, 4], vec![3, 6], vec![2, 8]]),
        2
    );
    assert_eq!(remove_covered_intervals(vec![vec![1, 4], vec![2, 3]]), 1)
}
