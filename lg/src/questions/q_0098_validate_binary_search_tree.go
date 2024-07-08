package questions

import (
	"math"

	"github.com/hi-rustin/lg/src/utils"
)

func isValidBST(root *utils.TreeNode) bool {
	var helper func(root *utils.TreeNode, minVal int, maxVal int) bool
	helper = func(root *utils.TreeNode, left int, right int) bool {
		if root == nil {
			return true
		}

		rootVal := root.Val
		if rootVal <= left || rootVal >= right {
			return false
		}

		return helper(root.Left, left, rootVal) && helper(root.Right, rootVal, right)
	}
	return helper(root, math.MinInt, math.MaxInt)
}
