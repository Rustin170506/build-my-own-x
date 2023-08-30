use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let nums_map: HashMap<i32, i32> = nums
        .iter()
        .enumerate()
        .map(|(i, e)| (*e, i as i32))
        .collect();

    for (i, el) in nums.iter().enumerate() {
        let complement = target - el;
        if nums_map.contains_key(&complement) && *nums_map.get(&complement).unwrap() != i as i32 {
            return vec![i as i32, *nums_map.get(&complement).unwrap()];
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn test_two_sum() {
        let tests = vec![
            (vec![2, 7, 11, 15], 9, vec![0, 1]),
            (vec![3, 2, 4], 6, vec![1, 2]),
            (vec![3, 3], 6, vec![0, 1]),
        ];

        for ele in tests {
            assert_eq!(two_sum(ele.0, ele.1), ele.2)
        }
    }
}
