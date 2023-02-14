pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut subset = vec![];

    fn dfs(index: usize, nums: &Vec<i32>, res: &mut Vec<Vec<i32>>, subset: &mut Vec<i32>) {
        if index >= nums.len() {
            res.push(subset.clone());
            return;
        }

        subset.push(nums[index]);
        dfs(index + 1, nums, res, subset);

        subset.pop();
        dfs(index + 1, nums, res, subset);
    }

    dfs(0, &nums, &mut res, &mut subset);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets() {
        assert_eq!(
            subsets(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 2],
                vec![1, 3],
                vec![1],
                vec![2, 3],
                vec![2],
                vec![3],
                vec![]
            ]
        )
    }
}
