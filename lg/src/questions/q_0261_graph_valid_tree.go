package questions

func ValidTree(n int, edges [][]int) bool {
	neighbor := make(map[int][]int)
	visited := make(map[int]struct{})
	for _, edge := range edges {
		neighbor[edge[0]] = append(neighbor[edge[0]], edge[1])
		neighbor[edge[1]] = append(neighbor[edge[1]], edge[0])
	}

	var dfs func(node int, parent int) bool
	dfs = func(node int, parent int) bool {
		if _, ok := visited[node]; ok {
			return false
		}
		visited[node] = struct{}{}
		for _, nei := range neighbor[node] {
			if nei != parent && !dfs(nei, node) {
				return false
			}
		}
		return true
	}
	if !dfs(0, -1) {
		return false
	}

	for i := 0; i < n; i++ {
		if _, ok := visited[i]; !ok {
			return false
		}
	}

	return true
}
