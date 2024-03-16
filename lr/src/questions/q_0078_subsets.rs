pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut subset = vec![];

    dfs(0, &nums, &mut res, &mut subset);

    res
}

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

pub fn subsets_with_stack(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut stack: Vec<(usize, Vec<i32>)> = vec![(0, vec![])];

    while let Some((index, subset)) = stack.pop() {
        if index >= nums.len() {
            res.push(subset);
            continue;
        }

        let mut subset_with_current = subset.clone();
        subset_with_current.push(nums[index]);
        stack.push((index + 1, subset_with_current));

        stack.push((index + 1, subset));
    }

    res
}

#[test]
fn test_subsets() {
    let mut result = subsets(vec![1, 2, 3]);
    let mut expected = vec![
        vec![1, 2, 3],
        vec![1, 2],
        vec![1, 3],
        vec![1],
        vec![2, 3],
        vec![2],
        vec![3],
        vec![],
    ];
    result.sort();
    expected.sort();
    assert_eq!(result, expected);

    let mut result_with_stack = subsets_with_stack(vec![1, 2, 3]);
    result_with_stack.sort();
    assert_eq!(result_with_stack, expected);
}
