package questions

import (
	"github.com/hi-rustin/lg/src/utils"
)

func buildTree(preorder []int, inorder []int) *utils.TreeNode {
	if len(preorder) == 0 || len(inorder) == 0 {
		return nil
	}
	root := &utils.TreeNode{
		Val: preorder[0],
	}
	mid := 0
	for i, v := range inorder {
		if v == preorder[0] {
			mid = i
		}
	}

	root.Left = buildTree(preorder[1:mid+1], inorder[:mid])
	root.Right = buildTree(preorder[mid+1:], inorder[mid+1:])

	return root
}
