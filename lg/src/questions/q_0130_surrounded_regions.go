package questions

func solve(board [][]byte) {
	rows, cols := len(board), len(board[0])
	directions := [][]int{
		{1, 0},
		{-1, 0},
		{0, 1},
		{0, -1},
	}

	var markBorderConnected func(r, c int)
	markBorderConnected = func(r, c int) {
		if r < 0 || r >= rows || c < 0 || c >= cols || board[r][c] != 'O' {
			return
		}
		board[r][c] = 'B'
		for _, direction := range directions {
			markBorderConnected(r+direction[0], c+direction[1])
		}
	}

	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if (r == 0 || r == rows-1 || c == 0 || c == cols-1) && board[r][c] == 'O' {
				markBorderConnected(r, c)
			}
		}
	}

	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if board[r][c] == 'O' {
				board[r][c] = 'X'
			}
		}
	}

	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if board[r][c] == 'B' {
				board[r][c] = 'O'
			}
		}
	}
}
