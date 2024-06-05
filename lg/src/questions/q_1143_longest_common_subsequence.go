package questions

func longestCommonSubsequence(text1 string, text2 string) int {
	var dfs func(i, j int) int
	dfs = func(i, j int) int {
		if i >= len(text1) || j >= len(text2) {
			return 0
		}

		if text1[i] == text2[j] {
			return 1 + dfs(i+1, j+1)
		} else {
			return max(dfs(i+1, j), dfs(i, j+1))
		}
	}
	return dfs(0, 0)
}

func longestCommonSubsequenceIteration(text1 string, text2 string) int {
	m, n := len(text1), len(text2)
	dp := make([][]int, m+1)
	for i := range dp {
		dp[i] = make([]int, n+1)
	}

	for i := m - 1; i >= 0; i-- {
		for j := n - 1; j >= 0; j-- {
			if text1[i] == text2[j] {
				dp[i][j] = 1 + dp[i+1][j+1]
			} else {
				dp[i][j] = max(dp[i+1][j], dp[i][j+1])
			}
		}
	}
	return dp[0][0]
}
