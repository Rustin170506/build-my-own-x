package questions

import (
	"container/heap"
	"math"
)

func networkDelayTime(times [][]int, n int, k int) int {
	// Create adjacency list from times
	adjList := make([][][2]int, n+1)
	for _, time := range times {
		u, v, w := time[0], time[1], time[2]
		adjList[u] = append(adjList[u], [2]int{v, w})
	}

	// Initialize distances with infinity
	dist := make([]int, n+1)
	for i := range dist {
		dist[i] = math.MaxInt32
	}
	dist[k] = 0

	// Initialize priority queue with source node
	pq := make(PriorityQueue, 0)
	heap.Init(&pq)
	heap.Push(&pq, [2]int{k, 0})

	// Dijkstra's algorithm
	for pq.Len() > 0 {
		node := heap.Pop(&pq).([2]int)
		u, d := node[0], node[1]
		if d > dist[u] {
			continue
		}
		for _, edge := range adjList[u] {
			v, w := edge[0], edge[1]
			if newDist := d + w; newDist < dist[v] {
				dist[v] = newDist
				heap.Push(&pq, [2]int{v, newDist})
			}
		}
	}

	// Find the maximum distance
	max := 0
	for i := 1; i <= n; i++ {
		if dist[i] == math.MaxInt32 {
			return -1
		}
		if dist[i] > max {
			max = dist[i]
		}
	}

	return max
}

// PriorityQueue implements heap.Interface for a priority queue of [2]int.
type PriorityQueue [][2]int

func (pq PriorityQueue) Len() int           { return len(pq) }
func (pq PriorityQueue) Less(i, j int) bool { return pq[i][1] < pq[j][1] }
func (pq PriorityQueue) Swap(i, j int)      { pq[i], pq[j] = pq[j], pq[i] }

func (pq *PriorityQueue) Push(x interface{}) {
	item := x.([2]int)
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[0 : n-1]
	return item
}
