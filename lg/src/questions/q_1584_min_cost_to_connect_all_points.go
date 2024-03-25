package questions

import (
	"container/heap"
	"math"
)

type intHeap [][]int

func (h intHeap) Len() int {
	return len(h)
}

func (h intHeap) Less(i int, j int) bool {
	return h[i][0] < h[j][0]
}

func (h intHeap) Swap(i int, j int) {
	h[i], h[j] = h[j], h[i]
}

func (h *intHeap) Push(x any) {
	*h = append(*h, x.([]int))
}

func (h *intHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

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
	minHeap := &intHeap{[]int{0, 0}}

	for minHeap.Len() > 0 {
		min := heap.Pop(minHeap).([]int)
		distance, node := min[0], min[1]
		if _, ok := visited[node]; ok {
			continue
		}

		res += distance
		visited[node] = struct{}{}

		for _, nei := range adj[node] {
			if _, ok := visited[nei[1]]; ok {
				continue
			}

			heap.Push(minHeap, []int{nei[0], nei[1]})
		}
	}

	return res
}
