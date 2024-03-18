package questions

import "fmt"

func numIslands(grid [][]byte) int {
	if len(grid) == 0 {
		return 0
	}
	result := 0
	visited := make(map[string]struct{})
	rows, cols := len(grid), len(grid[0])
	bfs := func(i int, j int) {
		queue := make([][]int, 0)
		visited[fmt.Sprintf("%d:%d", i, j)] = struct{}{}
		queue = append(queue, []int{i, j})
		for len(queue) > 0 {
			curr := queue[0]
			queue = queue[1:]
			i, j = curr[0], curr[1]
			directions := [][]int{{0, -1}, {0, 1}, {1, 0}, {-1, 0}}
			for _, d := range directions {
				newRow := d[0] + i
				newCol := d[1] + j
				if newRow < rows && newRow >= 0 && newCol < cols && newCol >= 0 && grid[newRow][newCol] == '1' {
					if _, ok := visited[fmt.Sprintf("%d:%d", newRow, newCol)]; !ok {
						visited[fmt.Sprintf("%d:%d", newRow, newCol)] = struct{}{}
						queue = append(queue, []int{newRow, newCol})
					}
				}
			}
		}

	}

	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			if grid[i][j] == '1' {
				if _, ok := visited[fmt.Sprintf("%d:%d", i, j)]; !ok {
					bfs(i, j)
					result += 1
				}
			}
		}
	}

	return result
}

func numIslandsDFS(grid [][]byte) int {
	if len(grid) == 0 {
		return 0
	}
	result := 0
	visited := make(map[string]struct{})
	rows, cols := len(grid), len(grid[0])
	var dfs func(i int, j int)
	dfs = func(i int, j int) {
		if i >= rows || i < 0 || j >= cols || j < 0 || grid[i][j] == '0' {
			return
		}
		if _, ok := visited[fmt.Sprintf("%d:%d", i, j)]; ok {
			return
		}
		visited[fmt.Sprintf("%d:%d", i, j)] = struct{}{}
		directions := [][]int{{0, -1}, {0, 1}, {1, 0}, {-1, 0}}
		for _, d := range directions {
			newRow := d[0] + i
			newCol := d[1] + j
			dfs(newRow, newCol)
		}
	}

	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			if grid[i][j] == '1' {
				if _, ok := visited[fmt.Sprintf("%d:%d", i, j)]; !ok {
					dfs(i, j)
					result += 1
				}
			}
		}
	}

	return result
}
