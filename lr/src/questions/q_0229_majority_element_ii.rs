use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for num in &nums {
        map.entry(*num).and_modify(|count| *count += 1).or_default();
    }

    let majority = (nums.len() / 3) as i32;
    let mut result: Vec<i32> = map
        .into_iter()
        .filter(|(_, count)| count >= &majority)
        .map(|(num, _)| num)
        .collect();
    result.sort_unstable();
    result
}

#[test]
fn test_majority_element() {
    let nums = vec![3, 2, 3];
    assert_eq!(majority_element(nums), vec![3]);
    let nums = vec![1];
    assert_eq!(majority_element(nums), vec![1]);
    let nums = vec![1, 2];
    assert_eq!(majority_element(nums), vec![1, 2]);
}
