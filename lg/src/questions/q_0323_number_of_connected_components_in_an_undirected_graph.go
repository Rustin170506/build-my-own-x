package questions

func CountComponents(n int, edges [][]int) int {
	result := 0
	neighbors := make(map[int][]int)
	for _, edge := range edges {
		neighbors[edge[0]] = append(neighbors[edge[0]], edge[1])
		neighbors[edge[1]] = append(neighbors[edge[1]], edge[0])
	}
	visited := make(map[int]bool)
	var dfs func(node int, parent int)
	dfs = func(node int, parent int) {
		if _, ok := visited[node]; ok {
			return
		}
		for _, nei := range neighbors[node] {
			if nei != parent {
				dfs(nei, node)
			}
		}
		visited[node] = true
	}

	for i := 0; i < n; i++ {
		if _, ok := visited[i]; !ok {
			result++
			dfs(i, -1)
		}
	}

	return result
}
