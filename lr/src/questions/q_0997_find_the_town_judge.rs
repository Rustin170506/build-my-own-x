use std::collections::HashMap;

pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let mut incoming_map = HashMap::new();
    let mut outcoming_map = HashMap::new();
    for t in trust {
        let (src, dest) = (t[0], t[1]);
        *incoming_map.entry(dest).or_insert(0) += 1;
        *outcoming_map.entry(src).or_insert(0) += 1;
    }

    for i in 1..=n {
        let (&incoming, &outcoming) = (
            incoming_map.get(&i).unwrap_or(&0),
            outcoming_map.get(&i).unwrap_or(&0),
        );

        if incoming == n - 1 && outcoming == 0 {
            return i;
        }
    }

    -1
}

#[test]
fn test_find_judge() {
    let n = 2;
    let trust = vec![vec![1, 2]];
    assert_eq!(find_judge(n, trust), 2);

    let n = 3;
    let trust = vec![vec![1, 3], vec![2, 3]];
    assert_eq!(find_judge(n, trust), 3);

    let n = 3;
    let trust = vec![vec![1, 3], vec![2, 3], vec![3, 1]];
    assert_eq!(find_judge(n, trust), -1);

    let n = 3;
    let trust = vec![vec![1, 2], vec![2, 3]];
    assert_eq!(find_judge(n, trust), -1);

    let n = 4;
    let trust = vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]];
    assert_eq!(find_judge(n, trust), 3);
}
