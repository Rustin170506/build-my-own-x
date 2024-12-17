package questions

func WallsAndGates(rooms [][]int) {
	rows, cols := len(rooms), len(rooms[0])
	visited := make(map[[2]int]bool)
	queue := make([][2]int, 0)

	// Find all gates and start BFS from them simultaneously.
	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			if rooms[i][j] == 0 {
				index := [2]int{i, j}
				queue = append(
					queue,
					index,
				)
				visited[index] = true
			}
		}
	}

	distance := 0
	add := func(row, col int) {
		if row < 0 || row == rows || col < 0 || col == cols || rooms[row][col] == -1 {
			return
		}
		index := [2]int{row, col}
		if _, ok := visited[index]; ok {
			return
		}
		visited[index] = true
		queue = append(queue, index)
	}
	for len(queue) > 0 {
		qLen := len(queue)
		for i := 0; i < qLen; i++ {
			index := queue[0]
			queue = queue[1:]
			row, col := index[0], index[1]
			rooms[row][col] = distance
			directions := [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
			for _, direction := range directions {
				add(row+direction[0], col+direction[1])
			}
		}
		distance++
	}
}
