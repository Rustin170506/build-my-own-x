package questions

func orangesRotting(grid [][]int) int {
	result := -1
	rows := len(grid)
	cols := len(grid[0])

	hashFresh := false
	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			if grid[i][j] == 1 {
				hashFresh = true
				break
			}
		}
	}

	if !hashFresh {
		return 0
	}

	queue := make([][2]int, 0, 4)
	visited := make([][]bool, rows)
	for i := 0; i < rows; i++ {
		visited[i] = make([]bool, cols)
	}
	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			if grid[i][j] == 2 {
				queue = append(queue, [2]int{i, j})
				visited[i][j] = true
			}
		}
	}

	qLen := len(queue)
	for qLen != 0 {
		node := queue[0]
		queue = queue[1:]
		row, col := node[0], node[1]
		if grid[row][col] == 1 {
			grid[row][col] = 2
		}

		directions := [][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
		for _, direction := range directions {
			r, c := direction[0], direction[1]
			r = row + r
			c = col + c
			if r < 0 || r >= rows || c < 0 || c >= cols || visited[r][c] {
				continue
			}
			visited[r][c] = true
			if grid[r][c] == 1 {
				queue = append(queue, [2]int{r, c})
			}
		}
		qLen -= 1
		if qLen == 0 {
			result += 1
			qLen = len(queue)
		}
	}

	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			if grid[i][j] == 1 {
				return -1
			}
		}
	}

	return result
}
