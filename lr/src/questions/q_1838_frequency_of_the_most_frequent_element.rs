pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    let (mut l, mut r) = (0, 0);
    let (mut res, mut total) = (0, 0_usize);

    while r < nums.len() {
        total += nums[r] as usize;
        while nums[r] as usize * (r - l + 1) > total + k as usize {
            total -= nums[l] as usize;
            l += 1;
        }

        res = res.max(r - l + 1);
        r += 1;
    }

    res as i32
}

#[test]
fn test_max_frequency() {
    assert_eq!(max_frequency(vec![1, 2, 4], 5), 3);
    assert_eq!(max_frequency(vec![1, 4, 8, 13], 5), 2);
    assert_eq!(max_frequency(vec![3, 9, 6], 2), 1);
}
