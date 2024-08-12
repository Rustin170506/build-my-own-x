package questions

import (
	"math"

	"github.com/emirpasic/gods/v2/trees/binaryheap"
)

func minCostConnectPoints(points [][]int) int {
	adj := make(map[int][][]int, len(points))

	for i := range points {
		x1, y1 := points[i][0], points[i][1]
		for j := i; j < len(points); j++ {
			x2, y2 := points[j][0], points[j][1]
			distance := int(math.Abs(float64(x1-x2)) + math.Abs(float64(y1-y2)))
			if _, ok := adj[j]; !ok {
				adj[j] = make([][]int, 0, len(points))
			}
			adj[j] = append(adj[j], []int{distance, i})
			if _, ok := adj[i]; !ok {
				adj[i] = make([][]int, 0, len(points))
			}
			adj[i] = append(adj[i], []int{distance, j})
		}
	}

	res := 0
	visited := make(map[int]struct{})
	minHeap := binaryheap.NewWith(func(x, y [2]int) int {
		return x[0] - y[0]
	})
	minHeap.Push([2]int{0, 0})

	for !minHeap.Empty() {
		minValue, _ := minHeap.Pop()
		distance, node := minValue[0], minValue[1]
		if _, ok := visited[node]; ok {
			continue
		}

		res += distance
		visited[node] = struct{}{}

		for _, nei := range adj[node] {
			if _, ok := visited[nei[1]]; ok {
				continue
			}

			minHeap.Push([2]int{nei[0], nei[1]})
		}
	}

	return res
}
