pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut output = vec![intervals[0].clone()];

    for interval in intervals.iter().skip(1) {
        let last_end = output.last().unwrap()[1];
        if interval[0] <= last_end {
            let last = output.len() - 1;
            output[last][1] = i32::max(last_end, interval[1]);
        } else {
            output.push(interval.clone());
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        assert_eq!(
            merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
        assert_eq!(merge(vec![vec![1, 4], vec![4, 5]]), vec![vec![1, 5]]);
    }
}
