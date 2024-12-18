package questions

import "fmt"

func orangesRotting(grid [][]int) int {
	result := -1
	rows := len(grid)
	cols := len(grid[0])
	queue := make([][2]int, 0)
	visited := make(map[string]bool)
	hasFresh := 0
	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			if grid[i][j] == 2 {
				queue = append(queue, [2]int{i, j})
			} else if grid[i][j] == 1 {
				hasFresh++
			}
		}
	}
	if hasFresh == 0 {
		return 0
	}

	directions := [][2]int{
		{1, 0},
		{-1, 0},
		{0, 1},
		{0, -1},
	}

	qLen := len(queue)
	for qLen > 0 {
		orange := queue[0]
		queue = queue[1:]
		row, col := orange[0], orange[1]
		if grid[row][col] == 1 {
			grid[row][col]++
		}

		for _, direction := range directions {
			newRow := row + direction[0]
			newCol := col + direction[1]
			idx := fmt.Sprintf("%d-%d", newRow, newCol)
			if newRow < 0 || newRow >= rows || newCol < 0 || newCol >= cols || visited[idx] {
				continue
			}
			visited[idx] = true
			if grid[newRow][newCol] == 1 {
				hasFresh--
				queue = append(queue, [2]int{newRow, newCol})
			}
		}

		qLen--
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
