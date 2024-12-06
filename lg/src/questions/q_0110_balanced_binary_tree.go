package questions

import (
	"math"

	"github.com/hi-rustin/lg/src/utils"
)

func helper(root *utils.TreeNode) float64 {
	if root == nil {
		return 0
	}
	return max(helper(root.Left), helper(root.Right)) + 1
}

func isBalanced(root *utils.TreeNode) bool {
	if root == nil {
		return true
	}

	if math.Abs(helper(root.Left)-helper(root.Right)) > 1 {
		return false
	}

	return isBalanced(root.Left) && isBalanced(root.Right)
}
