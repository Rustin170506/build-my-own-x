package questions

func maxAreaOfIsland(grid [][]int) int {
	rows := len(grid)
	cols := len(grid[0])
	result := 0

	var dfs func(row, col int) int
	dfs = func(row, col int) int {
		if row < 0 || row >= rows || col < 0 || col >= cols || grid[row][col] == 0 {
			return 0
		}

		count := 1
		grid[row][col] = 0
		directions := [][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
		for _, direction := range directions {
			count += dfs(row+direction[0], col+direction[1])
		}

		return count
	}

	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			if grid[i][j] == 1 {
				count := dfs(i, j)
				if count > result {
					result = count
				}
			}
		}
	}

	return result
}
