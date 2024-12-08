package questions

import (
	"math"

	"github.com/hi-rustin/lg/src/utils"
)

func isValidBST(root *utils.TreeNode) bool {
	if root == nil {
		return false
	}

	var dfs func(node *utils.TreeNode, left, right int) bool
	dfs = func(node *utils.TreeNode, left, right int) bool {
		if node == nil {
			return true
		}

		if node.Val <= left || node.Val >= right {
			return false
		}

		return dfs(node.Left, left, node.Val) && dfs(node.Right, node.Val, right)
	}

	return dfs(root, math.MinInt, math.MaxInt)
}
