pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut nums = nums;
    nums.sort();

    for (i, n) in nums.iter().enumerate() {
        if i > 0 && *n == nums[i - 1] {
            continue;
        }

        let (mut left, mut right) = (i + 1, nums.len() - 1);
        while left < right {
            let temp = n + nums[left] + nums[right];
            match temp.cmp(&0) {
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Equal => {
                    result.push(vec![*n, nums[left], nums[right]]);
                    left += 1;
                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }
                std::cmp::Ordering::Greater => right -= 1,
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        assert_eq!(three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }
}
