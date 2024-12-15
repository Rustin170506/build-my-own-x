package questions

import "fmt"

func maxAreaOfIsland(grid [][]int) int {
	if len(grid) == 0 {
		return 0
	}
	rows := len(grid)
	cols := len(grid[0])
	directions := [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	visited := make(map[string]struct{})
	var dfs func(row, col int) int
	dfs = func(row, col int) int {
		if row < 0 || row >= rows || col < 0 || col >= cols || grid[row][col] == 0 {
			return 0
		}
		idx := fmt.Sprintf("%d-%d", row, col)
		if _, ok := visited[idx]; ok {
			return 0
		}

		visited[idx] = struct{}{}
		res := 1
		for _, direction := range directions {
			newRow, newCol := row+direction[0], col+direction[1]
			res += dfs(newRow, newCol)
		}

		return res
	}

	result := 0
	for row := 0; row < rows; row++ {
		for col := 0; col < cols; col++ {
			idx := fmt.Sprintf("%d-%d", row, col)
			if _, ok := visited[idx]; !ok && grid[row][col] == 1 {
				result = max(result, dfs(row, col))
			}
		}
	}

	return result
}
