package questions

import "github.com/hi-rustin/lg/src/utils"

func levelOrder(root *utils.TreeNode) [][]int {
	if root == nil {
		return nil
	}

	result := make([][]int, 0)

	q := make([]*utils.TreeNode, 0)
	q = append(q, root)
	for len(q) != 0 {
		qLen := len(q)
		levelNodes := make([]int, 0)
		for i := 0; i < qLen; i++ {
			node := q[0]
			levelNodes = append(levelNodes, node.Val)
			q = q[1:]
			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}
		result = append(result, levelNodes)
	}
	return result
}
