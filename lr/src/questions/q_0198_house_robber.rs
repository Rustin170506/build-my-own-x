pub fn rob(nums: Vec<i32>) -> i32 {
    let (mut rob1, mut rbo2) = (0, 0);

    for n in nums {
        let tmep = i32::max(rob1 + n, rbo2);
        rob1 = rbo2;
        rbo2 = tmep;
    }

    rbo2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }
}
