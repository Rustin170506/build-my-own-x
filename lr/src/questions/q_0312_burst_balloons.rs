pub fn max_coins(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.insert(0, 1);
    nums.push(1);
    let n = nums.len();
    let mut dp = vec![vec![0; n]; n];
    for len in 1..=n {
        for i in 0..n - len {
            let j = i + len;
            for k in i + 1..j {
                dp[i][j] = dp[i][j].max(dp[i][k] + dp[k][j] + nums[i] * nums[k] * nums[j]);
            }
        }
    }
    dp[0][n - 1]
}

#[test]
fn test_max_coins() {
    assert_eq!(max_coins(vec![3, 1, 5, 8]), 167);
}
