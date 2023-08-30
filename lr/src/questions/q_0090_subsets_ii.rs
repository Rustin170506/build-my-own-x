pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut nums = nums;
    nums.sort();

    fn backtrack(nums: &mut Vec<i32>, i: usize, result: &mut Vec<Vec<i32>>, subset: &mut Vec<i32>) {
        if i == nums.len() {
            result.push(subset.clone());
            return;
        }

        subset.push(nums[i]);
        backtrack(nums, i + 1, result, subset);
        subset.pop();

        let mut i = i;
        while i + 1 < nums.len() && nums[i] == nums[i + 1] {
            i += 1;
        }
        backtrack(nums, i + 1, result, subset);
    }

    backtrack(&mut nums, 0, &mut result, &mut vec![]);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets_with_dup() {
        assert_eq!(
            subsets_with_dup(vec![1, 2, 2]),
            vec![
                vec![1, 2, 2],
                vec![1, 2],
                vec![1],
                vec![2, 2],
                vec![2],
                vec![]
            ]
        );
    }
}
