pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut res = *nums.iter().max().unwrap();
    let (mut cur_min, mut cur_max) = (1, 1);

    for n in nums {
        if n == 0 {
            cur_min = 1;
            cur_max = 1;
        }

        let temp = cur_max * n;
        cur_max = temp.max(n * cur_min).max(n);
        cur_min = temp.min(n * cur_min).min(n);

        res = i32::max(res, cur_max)
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product() {
        assert_eq!(max_product(vec![2, 3, -2, 4]), 6);
    }
}
