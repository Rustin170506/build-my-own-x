pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    for i in 1..intervals.len() {
        let i1 = &intervals[i - 1];
        let i2 = &intervals[i];

        if i1[1] > i2[0] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_attend_meetings() {
        assert!(!can_attend_meetings(vec![
            vec![0, 30],
            vec![5, 10],
            vec![15, 20]
        ]));
        assert!(can_attend_meetings(vec![vec![7, 10], vec![2, 4]]));
    }
}
