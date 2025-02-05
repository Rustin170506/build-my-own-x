pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();

    let mut res = vec![];
    let mut quad = vec![];

    fn k_sum(
        k: usize,
        start: usize,
        res: &mut Vec<Vec<i32>>,
        quad: &mut Vec<i32>,
        nums: &[i32],
        target: i64,
    ) {
        if k != 2 {
            if nums.len() < k {
                return;
            }
            for i in start..=nums.len() - k {
                if i > start && nums[i] == nums[i - 1] {
                    continue;
                }
                quad.push(nums[i]);
                k_sum(k - 1, i + 1, res, quad, nums, target - nums[i] as i64);
                quad.pop();
            }
            return;
        }

        let (mut l, mut r) = (start, nums.len() - 1);
        while l < r {
            let sum = nums[l] as i64 + nums[r] as i64;
            match sum.cmp(&target) {
                std::cmp::Ordering::Less => l += 1,
                std::cmp::Ordering::Equal => {
                    let mut temp = quad.clone();
                    temp.push(nums[l]);
                    temp.push(nums[r]);
                    res.push(temp);
                    l += 1;
                    while l < r && nums[l] == nums[l - 1] {
                        l += 1
                    }
                }
                std::cmp::Ordering::Greater => r -= 1,
            }
        }
    }

    k_sum(4, 0, &mut res, &mut quad, &nums, target as i64);

    res
}

#[test]
fn test_four_sum() {
    let nums = vec![1, 0, -1, 0, -2, 2];
    let target = 0;
    let res = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
    assert_eq!(four_sum(nums, target), res);

    let nums = vec![0];
    let target = 0;
    let res: Vec<Vec<i32>> = vec![];
    assert_eq!(four_sum(nums, target), res);
}
