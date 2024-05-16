use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut result = 0;
    let nums_set: HashSet<&i32> = HashSet::from_iter(nums.iter());

    for n in &nums {
        if nums_set.get(&(n - 1)).is_none() {
            let mut temp_count = 1;
            let mut temp = *n;
            while nums_set.get(&(temp + 1)).is_some() {
                temp_count += 1;
                temp += 1;
            }
            if temp_count > result {
                result = temp_count;
            }
        }
    }

    result
}

#[test]
fn test_longest_consecutive() {
    assert_eq!(longest_consecutive(vec![]), 0);
    assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
}
