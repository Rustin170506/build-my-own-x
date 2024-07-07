package questions

import "github.com/hi-rustin/lg/src/utils"

func kthSmallest(root *utils.TreeNode, k int) int {
	elements := make([]int, 0)
	var inorder func(root *utils.TreeNode)
	inorder = func(root *utils.TreeNode) {
		if root == nil {
			return
		}

		inorder(root.Left)
		elements = append(elements, root.Val)
		inorder(root.Right)

	}
	inorder(root)
	return elements[k-1]
}
