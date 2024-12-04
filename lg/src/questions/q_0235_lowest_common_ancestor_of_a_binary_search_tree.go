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

func lowestCommonAncestorRecursive(root, p, q *utils.TreeNode) *utils.TreeNode {
	if root == nil {
		return nil
	}
	if p.Val < root.Val && q.Val < root.Val {
		return lowestCommonAncestorRecursive(root.Left, p, q)
	}
	if p.Val > root.Val && q.Val > root.Val {
		return lowestCommonAncestorRecursive(root.Right, p, q)
	}
	return root
}
