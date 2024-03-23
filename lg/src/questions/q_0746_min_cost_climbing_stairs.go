package questions

func minCostClimbingStairs(cost []int) int {
	cache := make([]int, len(cost))
	for i := range cost {
		cache[i] = -1
	}

	var dfs func(index int) int
	dfs = func(index int) int {
		if index >= len(cost) {
			return 0
		}

		if cache[index] != -1 {
			return cache[index]
		}

		cache[index] = cost[index] + min(dfs(index+1), dfs(index+2))

		return cache[index]
	}

	return min(dfs(0), dfs(1))
}

func minCostClimbingStairsDP(cost []int) int {
	first, second := cost[0], cost[1]
	for i := 2; i < len(cost); i++ {
		first, second = second, cost[i]+min(first, second)
	}

	return min(first, second)
}
