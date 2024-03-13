package questions

import "github.com/hi-rustin/lg/src/utils"

func invertTree(root *utils.TreeNode) *utils.TreeNode {
	if root == nil {
		return nil
	}

	left := root.Left
	root.Left = invertTree(root.Right)
	root.Right = invertTree(left)

	return root
}
