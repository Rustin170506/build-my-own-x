package questions

import "github.com/hi-rustin/lg/src/utils"

func levelOrder(root *utils.TreeNode) [][]int {
	if root == nil {
		return nil
	}

	result := make([][]int, 0)
	queue := make([]*utils.TreeNode, 0)
	queue = append(queue, root)
	for len(queue) > 0 {
		queueLen := len(queue)
		level := make([]int, 0, queueLen)
		for queueLen > 0 {
			node := queue[0]
			level = append(level, node.Val)
			queue = queue[1:]
			if node.Left != nil {
				queue = append(queue, node.Left)
			}
			if node.Right != nil {
				queue = append(queue, node.Right)
			}
			queueLen -= 1
		}
		result = append(result, level)
	}

	return result
}
