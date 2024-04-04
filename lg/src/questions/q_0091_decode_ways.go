package questions

import "strings"

func numDecodings(s string) int {
	dp := make(map[int]int)
	dp[len(s)] = 1

	var dfs func(i int) int
	dfs = func(i int) int {
		if n, ok := dp[i]; ok {
			return n
		}
		if s[i] == '0' {
			return 0
		}

		res := dfs(i + 1)
		if (i+1) < len(s) && (s[i] == '1' || (s[i] == '2' && strings.ContainsRune("0123456", rune(s[i+1])))) {
			res += dfs(i + 2)
		}
		dp[i] = res

		return res
	}

	return dfs(0)
}
