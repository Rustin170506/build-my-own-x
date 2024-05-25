package questions

import "github.com/hi-rustin/lg/src/utils"

func lowestCommonAncestor(root, p, q *utils.TreeNode) *utils.TreeNode {
	if root == nil {
		return nil
	}

	current := root
	for current != nil {
		val := current.Val
		if p.Val < val && q.Val < val {
			current = current.Left
		} else if p.Val > val && q.Val > val {
			current = current.Right
		} else {
			return current
		}
	}

	panic("unreachable")
}
