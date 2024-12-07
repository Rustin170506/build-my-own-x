package questions

import "github.com/hi-rustin/lg/src/utils"

func rightSideView(root *utils.TreeNode) []int {
	if root == nil {
		return nil
	}

	result := make([]int, 0)
	queue := make([]*utils.TreeNode, 0)
	queue = append(queue, root)
	for len(queue) > 0 {
		queueLen := len(queue)
		for queueLen > 0 {
			node := queue[0]
			queue = queue[1:]
			if queueLen == 1 {
				result = append(result, node.Val)
			}
			if node.Left != nil {
				queue = append(queue, node.Left)
			}
			if node.Right != nil {
				queue = append(queue, node.Right)
			}
			queueLen--
		}
	}

	return result
}
