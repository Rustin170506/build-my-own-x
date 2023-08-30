pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut new_interval = new_interval;
    for (i, interval) in intervals.iter().enumerate() {
        if new_interval[1] < interval[0] {
            result.push(new_interval);
            let mut rest = intervals[i..].to_vec();
            result.append(&mut rest);
            return result;
        } else if new_interval[0] > interval[1] {
            result.push(interval.clone());
        } else {
            new_interval = vec![
                i32::min(new_interval[0], interval[0]),
                i32::max(new_interval[1], interval[1]),
            ];
        }
    }

    result.push(new_interval);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        assert_eq!(
            vec![vec![1, 5], vec![6, 9]],
            insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5])
        );
        assert_eq!(
            vec![vec![1, 2], vec![3, 10], vec![12, 16]],
            insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            )
        );
    }
}
