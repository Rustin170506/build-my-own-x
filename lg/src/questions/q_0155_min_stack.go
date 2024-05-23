package questions

import "math"

type MinStack struct {
	inner    [][]int
	minValue int
}

func ConstructorMinStack() MinStack {
	return MinStack{
		inner:    make([][]int, 0),
		minValue: math.MaxInt,
	}
}

func (this *MinStack) Push(val int) {
	lastMin := this.minValue
	this.inner = append(this.inner, []int{val, lastMin})
	if val < this.minValue {
		this.minValue = val
	}
}

func (this *MinStack) Pop() {
	last := this.inner[len(this.inner)-1:]
	this.minValue = last[0][1]
	this.inner = this.inner[:len(this.inner)-1]
}

func (this *MinStack) Top() int {
	return this.inner[len(this.inner)-1:][0][0]
}

func (this *MinStack) GetMin() int {
	return this.minValue
}
