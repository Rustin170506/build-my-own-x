package questions

func coinChange(coins []int, amount int) int {
	dp := make([]int, 0, amount+1)
	for i := 0; i < amount+1; i++ {
		dp = append(dp, amount+1)
	}
	dp[0] = 0

	for a := range dp {
		for _, coin := range coins {
			if a-coin >= 0 {
				dp[a] = min(dp[a], 1+dp[a-coin])
			}
		}
	}
	if dp[amount] != amount+1 {
		return dp[amount]
	}

	return -1
}
