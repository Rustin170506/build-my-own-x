package questions

import "github.com/hi-rustin/lg/src/utils"

func rightSideView(root *utils.TreeNode) []int {
	if root == nil {
		return nil
	}
	result := make([]int, 0)
	q := make([]*utils.TreeNode, 0)
	q = append(q, root)
	for len(q) != 0 {
		qLen := len(q)
		for i := 0; i < qLen; i++ {
			node := q[0]
			q = q[1:]
			if i == qLen-1 {
				result = append(result, node.Val)
			}
			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}
	}

	return result
}
