package questions

func exist(board [][]byte, word string) bool {
	row, col := len(board), len(board[0])
	current := make([]byte, 0, len(word))
	visited := make(map[[2]int]bool)
	found := false
	var backtrace func(i, j, k int)
	backtrace = func(i, j, k int) {
		if found {
			return
		}
		_, ok := visited[[2]int{i, j}]
		if i < 0 || i >= row || j < 0 || j >= col || ok || board[i][j] != word[k] {
			return
		}

		current = append(current, board[i][j])
		visited[[2]int{i, j}] = true

		if string(current) == word {
			found = true
		} else {
			directions := [][]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
			for _, direction := range directions {
				backtrace(i+direction[0], j+direction[1], k+1)
			}
		}

		if !found {
			current = current[:len(current)-1]
			delete(visited, [2]int{i, j})
		}
	}

	for i := 0; i < row; i++ {
		for j := 0; j < col; j++ {
			if board[i][j] == word[0] {
				backtrace(i, j, 0)
			}
		}
	}

	return found
}
