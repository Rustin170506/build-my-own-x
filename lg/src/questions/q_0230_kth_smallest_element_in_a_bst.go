package questions

import (
	"github.com/emirpasic/gods/v2/stacks/arraystack"
	"github.com/hi-rustin/lg/src/utils"
)

func kthSmallest(root *utils.TreeNode, k int) int {
	elements := make([]int, 0)
	var inorder func(root *utils.TreeNode)
	inorder = func(root *utils.TreeNode) {
		if root == nil {
			return
		}
		inorder(root.Left)
		elements = append(elements, root.Val)
		inorder(root.Right)
	}
	inorder(root)
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
