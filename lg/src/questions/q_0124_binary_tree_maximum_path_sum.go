package questions

import (
	"math"

	"github.com/hi-rustin/lg/src/utils"
)

func maxPathSum(root *utils.TreeNode) int {
	if root == nil {
		return 0
	}
	result := math.MinInt
	var dfs func(node *utils.TreeNode) int
	dfs = func(node *utils.TreeNode) int {
		if node == nil {
			return 0
		}

		leftSum := max(0, dfs(node.Left))
		rightSum := max(0, dfs(node.Right))

		result = max(result, node.Val+leftSum+rightSum)
		return node.Val + max(leftSum, rightSum)
	}

	dfs(root)
	return result
}
