package questions

import "github.com/emirpasic/gods/v2/trees/binaryheap"

type heapNode struct {
	time int
	row  int
	col  int
}

func swimInWater(grid [][]int) int {
	n := len(grid)
	visited := make(map[[2]int]struct{})
	directions := [][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}

	minHeap := binaryheap.NewWith(func(n1 heapNode, n2 heapNode) int {
		if n1.time > n2.time {
			return 1
		} else if n1.time == n2.time {
			return 0
		} else {
			return -1
		}
	})
	minHeap.Push(heapNode{grid[0][0], 0, 0})

	for !minHeap.Empty() {
		node, _ := minHeap.Pop()
		if node.row == n-1 && node.col == n-1 {
			return node.time
		}

		for _, direction := range directions {
			row := direction[0] + node.row
			col := direction[1] + node.col
			if row < 0 || row >= n || col < 0 || col >= n {
				continue
			}
			if _, ok := visited[[2]int{row, col}]; ok {
				continue
			}
			visited[[2]int{row, col}] = struct{}{}
			newNode := heapNode{
				time: max(node.time, grid[row][col]),
				row:  row,
				col:  col,
			}
			minHeap.Push(newNode)
		}

	}

	panic("unreachable")
}
