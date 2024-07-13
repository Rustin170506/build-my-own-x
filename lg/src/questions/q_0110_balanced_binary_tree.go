package questions

import (
	"math"

	"github.com/hi-rustin/lg/src/utils"
)

func isBalanced(root *utils.TreeNode) bool {
	var helper func(root *utils.TreeNode) int
	helper = func(node *utils.TreeNode) int {
		if node == nil {
			return 0
		}
		leftHeight := helper(node.Left)
		rightHeight := helper(node.Right)
		return max(leftHeight, rightHeight) + 1
	}
	if root == nil {
		return true
	}

	leftHeight := helper(root.Left)
	rightHeight := helper(root.Right)
	if math.Abs(float64(leftHeight-rightHeight)) > 1 {
		return false
	}

	return isBalanced(root.Left) && isBalanced(root.Right)
}
