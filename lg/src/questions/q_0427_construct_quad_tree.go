package questions

type QuadNode struct {
	Val         bool
	IsLeaf      bool
	TopLeft     *QuadNode
	TopRight    *QuadNode
	BottomLeft  *QuadNode
	BottomRight *QuadNode
}

func construct(grid [][]int) *QuadNode {
	var dfs func(n, r, c int) *QuadNode
	dfs = func(n, r, c int) *QuadNode {
		allSame := true
		for i := 0; i < n; i++ {
			for j := 0; j < n; j++ {
				if grid[r][c] != grid[r+i][c+j] {
					allSame = false
					break
				}
			}
		}
		if allSame {
			return &QuadNode{
				Val:    grid[r][c] == 1,
				IsLeaf: true,
			}
		}

		n = n / 2
		topLeft := dfs(n, r, c)
		topRight := dfs(n, r, c+n)
		bottomLeft := dfs(n, r+n, c)
		bottomRight := dfs(n, r+n, c+n)
		return &QuadNode{
			Val:         false,
			IsLeaf:      false,
			TopLeft:     topLeft,
			TopRight:    topRight,
			BottomLeft:  bottomLeft,
			BottomRight: bottomRight,
		}

	}

	return dfs(len(grid), 0, 0)
}
