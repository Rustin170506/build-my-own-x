package questions

import (
	"github.com/emirpasic/gods/v2/queues/arrayqueue"
	"github.com/hi-rustin/lg/src/utils"
)

func invertTree(root *utils.TreeNode) *utils.TreeNode {
	if root == nil {
		return nil
	}

	left := root.Left
	right := root.Right
	root.Left = invertTree(right)
	root.Right = invertTree(left)

	return root
}

func invertTreeBreadthFirst(root *utils.TreeNode) *utils.TreeNode {
	if root == nil {
		return nil
	}

	queue := arrayqueue.New[*utils.TreeNode]()
	queue.Enqueue(root)

	for queue.Size() > 0 {
		current, _ := queue.Dequeue()
		current.Left, current.Right = current.Right, current.Left

		if current.Left != nil {
			queue.Enqueue(current.Left)
		}
		if current.Right != nil {
			queue.Enqueue(current.Right)
		}
	}

	return root
}
