pub fn jump(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let (mut l, mut r) = (0, 0);

    while r < nums.len() - 1 {
        let mut farthest = 0;

        for (i, n) in nums.iter().enumerate().take(r + 1).skip(l) {
            farthest = i32::max(farthest, i as i32 + n);
        }

        l = r + 1;
        r = farthest as usize;
        result += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump() {
        assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
    }
}
