package questions

import (
	"math"

	"github.com/hi-rustin/lg/src/utils"
)

func isValidBST(root *utils.TreeNode) bool {
	var helper func(root *utils.TreeNode, minVal int, maxVal int) bool
	helper = func(root *utils.TreeNode, minVal int, maxVal int) bool {
		if root == nil {
			return true
		}

		rootVal := root.Val
		if rootVal <= minVal || rootVal >= maxVal {
			return false
		}

		return helper(root.Left, minVal, rootVal) && helper(root.Right, rootVal, maxVal)
	}
	return helper(root, math.MinInt, math.MaxInt)
}
