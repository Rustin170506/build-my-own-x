package questions

func searchMatrix(matrix [][]int, target int) bool {
	rows := len(matrix)
	if rows == 0 {
		return false
	}
	cols := len(matrix[0])

	row, col := 0, cols-1

	for row < rows && col >= 0 {
		if matrix[row][col] == target {
			return true
		} else if matrix[row][col] < target {
			row++
		} else {
			col--
		}
	}

	return false
}
