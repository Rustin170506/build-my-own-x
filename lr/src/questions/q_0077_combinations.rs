pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    backtrack(&mut result, n, k, 1, &mut vec![]);
    result
}

fn backtrack(result: &mut Vec<Vec<i32>>, n: i32, k: i32, start: i32, comb: &mut Vec<i32>) {
    if comb.len() == k as usize {
        result.push(comb.clone())
    }

    for i in start..=n {
        comb.push(i);
        backtrack(result, n, k, i + 1, comb);
        comb.pop();
    }
}

#[test]
fn test_combine() {
    let n = 4;
    let k = 2;
    let result = combine(n, k);
    assert_eq!(
        result,
        vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4]
        ]
    );
}
