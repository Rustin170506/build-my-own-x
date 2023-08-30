pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = vec![amount + 1; (amount + 1) as usize];
    dp[0] = 0;

    for a in 1..amount + 1 {
        for c in &coins {
            if a - c >= 0 {
                dp[a as usize] = i32::min(dp[a as usize], 1 + dp[(a - c) as usize]);
            }
        }
    }

    if dp[amount as usize] != amount + 1 {
        dp[amount as usize]
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_change() {
        assert_eq!(coin_change(vec![1, 1], 2), 2);
        assert_eq!(coin_change(vec![2, 1], 2), 1);
    }
}
