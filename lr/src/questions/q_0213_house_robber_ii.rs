pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    i32::max(do_rob(&nums[1..]), do_rob(&nums[..nums.len() - 1]))
}

fn do_rob(nums: &[i32]) -> i32 {
    let (mut rob1, mut rob2) = (0, 0);

    for n in nums {
        let temp = i32::max(rob1 + n, rob2);
        rob1 = rob2;
        rob2 = temp
    }

    rob2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(rob(vec![2, 2]), 2);
        assert_eq!(rob(vec![2]), 2);
        assert_eq!(rob(vec![2, 3, 2]), 3);
    }
}
