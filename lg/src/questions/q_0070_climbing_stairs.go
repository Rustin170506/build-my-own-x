package questions

func climbStairs(n int) int {
	cache := make(map[int]int, 0)
	var dfs func(n1 int) int
	dfs = func(n1 int) int {
		if n1 > n {
			return 0
		}
		if n1 == n {
			return 1
		}
		if c, ok := cache[n1]; ok {
			return c
		}

		result := dfs(n1+1) + dfs(n1+2)
		cache[n1] = result
		return result
	}

	return dfs(0)
}

func climbStairsDP(n int) int {
	one, two := 1, 1

	for i := 0; i < n-1; i++ {
		one, two = one+two, one
	}

	return one
}
