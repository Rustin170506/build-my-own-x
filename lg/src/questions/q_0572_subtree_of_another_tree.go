package questions

import "github.com/hi-rustin/lg/src/utils"

func isSubtree(root *utils.TreeNode, subRoot *utils.TreeNode) bool {
	var isSameTree func(root1 *utils.TreeNode, root2 *utils.TreeNode) bool
	isSameTree = func(root1 *utils.TreeNode, root2 *utils.TreeNode) bool {
		if root1 == nil && root2 == nil {
			return true
		}
		if root1 == nil || root2 == nil {
			return false
		}
		if root1.Val != root2.Val {
			return false
		}

		return isSameTree(root1.Left, root2.Left) && isSameTree(root1.Right, root2.Right)
	}

	found := false
	var dfs func(node *utils.TreeNode)
	dfs = func(node *utils.TreeNode) {
		if node == nil {
			return
		}

		if node.Val == subRoot.Val {
			if !found {
				found = isSameTree(node, subRoot)
			}
		}
		dfs(node.Left)
		dfs(node.Right)
	}

	dfs(root)
	return found
}
