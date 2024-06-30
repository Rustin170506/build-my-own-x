use std::collections::HashMap;

pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut premute = vec![];
    let mut counter = HashMap::new();
    for n in &nums {
        *counter.entry(*n).or_insert(0) += 1;
    }

    dfs(&mut counter, &mut premute, &mut result, nums.len());
    result
}

fn dfs(
    counter: &mut HashMap<i32, i32>,
    premute: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
    total: usize,
) {
    if premute.len() == total {
        result.push(premute.clone());
        return;
    }

    let keys: Vec<i32> = counter.keys().cloned().collect();
    for &n in &keys {
        if let Some(&count) = counter.get(&n) {
            if count > 0 {
                premute.push(n);
                *counter.get_mut(&n).unwrap() -= 1;
                dfs(counter, premute, result, total);

                premute.pop();
                *counter.get_mut(&n).unwrap() += 1;
            }
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 2];
    let mut result = permute_unique(nums);
    result.sort();
    let expected = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
    assert_eq!(result, expected);
}
