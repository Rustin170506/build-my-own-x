package questions

import "github.com/emirpasic/gods/v2/trees/binaryheap"

type KthLargest struct {
	pq *binaryheap.Heap[int]
	k  int
}

func Constructor1(k int, nums []int) KthLargest {
	pq := binaryheap.New[int]()
	pq.Push(nums...)
	return KthLargest{
		pq: pq,
		k:  k,
	}

}

func (this *KthLargest) Add(val int) int {
	this.pq.Push(val)
	for this.pq.Size() > this.k {
		this.pq.Pop()
	}

	kth, _ := this.pq.Peek()
	return kth
}
