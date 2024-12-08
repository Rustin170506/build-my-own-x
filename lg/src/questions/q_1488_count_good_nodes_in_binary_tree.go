package questions

import (
	"github.com/hi-rustin/lg/src/utils"
)

func goodNodes(root *utils.TreeNode) int {
	if root == nil {
		return 0
	}

	var dfs func(node *utils.TreeNode, maxVal int) int
	dfs = func(node *utils.TreeNode, maxVal int) int {
		if node == nil {
			return 0
		}

		good := 0
		if node.Val >= maxVal {
			good += 1
			maxVal = node.Val
		}
		return good + dfs(node.Left, maxVal) + dfs(node.Right, maxVal)
	}

	return dfs(root, root.Val)
}
