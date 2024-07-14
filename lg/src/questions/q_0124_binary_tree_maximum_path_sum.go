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
	var dfs func(root *utils.TreeNode) int
	dfs = func(root *utils.TreeNode) int {
		if root == nil {
			return 0
		}

		leftSum := max(0, dfs(root.Left))
		rightSum := max(0, dfs(root.Right))
		result = max(result, root.Val+leftSum+rightSum)
		return root.Val + max(leftSum, rightSum)
	}
	dfs(root)
	return result
}
