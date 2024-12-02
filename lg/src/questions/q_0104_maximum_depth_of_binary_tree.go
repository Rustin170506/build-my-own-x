package questions

import "github.com/hi-rustin/lg/src/utils"

func maxDepth(root *utils.TreeNode) int {
	if root == nil {
		return 0
	}

	level := 0
	queue := []*utils.TreeNode{root}

	for len(queue) != 0 {
		children := []*utils.TreeNode{}

		for _, n := range queue {
			if n.Left != nil {
				children = append(children, n.Left)
			}

			if n.Right != nil {
				children = append(children, n.Right)
			}
		}
		level += 1
		queue = children
	}

	return level
}

func maxDepthDFS(root *utils.TreeNode) int {
	if root == nil {
		return 0
	}

	return 1 + max(maxDepthDFS(root.Left), maxDepthDFS(root.Right))
}
