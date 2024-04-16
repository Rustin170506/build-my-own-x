package questions

func pacificAtlantic(heights [][]int) [][]int {
	rows := len(heights)
	cols := len(heights[0])
	pacific := make(map[[2]int]bool)
	atlantic := make(map[[2]int]bool)

	var dfs func(row, col int, visited map[[2]int]bool, preHeight int)
	dfs = func(row, col int, visited map[[2]int]bool, preHeight int) {
		if row < 0 || row >= rows || col < 0 || col >= cols || heights[row][col] < preHeight {
			return
		}
		if _, ok := visited[[2]int{row, col}]; ok {
			return
		}

		visited[[2]int{row, col}] = true
		directions := [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
		for _, direction := range directions {
			r, c := row+direction[0], col+direction[1]
			dfs(r, c, visited, heights[row][col])
		}
	}
	for col := 0; col < cols; col++ {
		dfs(0, col, pacific, heights[0][col])
		dfs(rows-1, col, atlantic, heights[rows-1][col])
	}

	for row := 0; row < rows; row++ {
		dfs(row, 0, pacific, heights[row][0])
		dfs(row, cols-1, atlantic, heights[row][cols-1])
	}

	result := make([][]int, 0)
	for row := 0; row < rows; row++ {
		for col := 0; col < cols; col++ {
			index := [2]int{row, col}
			if pacific[index] && atlantic[index] {
				result = append(result, index[0:2])
			}
		}
	}
	return result
}
