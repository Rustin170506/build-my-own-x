package questions

func findRedundantConnection(edges [][]int) []int {
	nodes := make(map[int][]int)
	visited := make(map[int]struct{})
	for _, edge := range edges {
		if _, ok := nodes[edge[0]]; !ok {
			nodes[edge[0]] = make([]int, 0)
		}
		if _, ok := nodes[edge[1]]; !ok {
			nodes[edge[1]] = make([]int, 0)
		}
	}

	var dfs func(node, parent int) bool
	dfs = func(node, parent int) bool {
		visited[node] = struct{}{}
		for _, neighbor := range nodes[node] {
			if neighbor == parent {
				continue
			}
			if _, ok := visited[neighbor]; ok || !dfs(neighbor, node) {
				return false
			}
		}
		return true
	}

	for _, edge := range edges {
		visited = make(map[int]struct{})
		nodes[edge[0]] = append(nodes[edge[0]], edge[1])
		nodes[edge[1]] = append(nodes[edge[1]], edge[0])
		if !dfs(edge[0], -1) {
			return edge
		}
	}

	return nil
}
