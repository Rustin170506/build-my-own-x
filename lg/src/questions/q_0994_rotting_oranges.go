package questions

func orangesRotting(grid [][]int) int {
	result := -1
	rows := len(grid)
	cols := len(grid[0])
	queue := make([][2]int, 0, len(grid))
	hasFresh := 0
	visited := make([][]bool, len(grid))
	for i := 0; i < len(grid); i++ {
		visited[i] = make([]bool, len(grid[i]))
	}
	for row := 0; row < rows; row++ {
		for col := 0; col < cols; col++ {
			if grid[row][col] == 2 {
				queue = append(queue, [2]int{row, col})
			}
			if grid[row][col] == 1 {
				hasFresh += 1
			}
		}
	}
	if hasFresh == 0 {
		return 0
	}
	if len(queue) == 0 {
		return -1
	}

	qLen := len(queue)
	for qLen != 0 {
		node := queue[0]
		queue = queue[1:]
		row, col := node[0], node[1]
		if grid[row][col] == 1 {
			grid[row][col] = 2
		}

		directions := [][2]int{
			{0, 1},
			{0, -1},
			{1, 0},
			{-1, 0},
		}
		for _, direction := range directions {
			r, c := row+direction[0], col+direction[1]
			if r < 0 || r >= rows || c < 0 || c >= cols || visited[r][c] {
				continue
			}
			visited[r][c] = true
			if grid[r][c] == 1 {
				hasFresh -= 1
				queue = append(queue, [2]int{r, c})
			}
		}
		qLen -= 1
		if qLen == 0 {
			result += 1
			qLen = len(queue)
		}

	}
	if hasFresh != 0 {
		return -1
	}

	return result
}
