pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut intervals = intervals;

    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut result = 0;

    let mut prev_end = intervals[0][1];

    for v in intervals.iter().skip(1) {
        if prev_end <= v[0] {
            prev_end = v[1];
        } else {
            result += 1;
            prev_end = i32::min(prev_end, v[1]);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_erase_overlap_intervals() {
        assert_eq!(
            erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            1
        );
        assert_eq!(
            erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
            2
        );
        assert_eq!(erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]), 0);
    }
}
