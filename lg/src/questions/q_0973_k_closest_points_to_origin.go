package questions

import (
	"container/heap"
	"math"
)

func kClosest(points [][]int, k int) [][]int {
	distanceHeap := &pointHeap{}
	heap.Init(distanceHeap)
	result := make([][]int, 0)
	for _, p := range points {
		x, y := p[0], p[1]
		distance := math.Sqrt(float64(x*x + y*y))
		heap.Push(distanceHeap, point{
			distance: distance,
			x:        x,
			y:        y,
		})
	}

	for i := 0; i < k; i++ {
		p := heap.Pop(distanceHeap).(point)
		result = append(result, []int{p.x, p.y})
	}
	return result
}

type point struct {
	distance float64
	x        int
	y        int
}

type pointHeap []point

func (h pointHeap) Len() int {
	return len(h)
}

func (h pointHeap) Less(i, j int) bool {
	return h[i].distance < h[j].distance
}

func (h pointHeap) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
}

func (h *pointHeap) Push(x interface{}) {
	*h = append(*h, x.(point))
}

func (h *pointHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}
