package questions

import (
	"github.com/emirpasic/gods/v2/stacks/arraystack"
	"github.com/hi-rustin/lg/src/utils"
)

func kthSmallest(root *utils.TreeNode, k int) int {
	elements := make([]int, 0)
	var inoder func(node *utils.TreeNode)
	inoder = func(node *utils.TreeNode) {
		if node == nil {
			return
		}
		inoder(node.Left)
		elements = append(elements, node.Val)
		inoder(node.Right)
	}
	inoder(root)
	return elements[k-1]
}

func kthSmallestWithStack(root *utils.TreeNode, k int) int {
	n := 0
	stack := arraystack.New[*utils.TreeNode]()
	current := root
	for current != nil || !stack.Empty() {
		for current != nil {
			stack.Push(current)
			current = current.Left
		}
		current, _ = stack.Pop()
		n++
		if n == k {
			return current.Val
		}
		current = current.Right
	}
	panic("unreachable")
}
