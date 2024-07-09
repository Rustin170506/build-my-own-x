package utils

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func BuildTree(elements []int) *TreeNode {
	if len(elements) == 0 {
		return nil
	}

	root := &TreeNode{Val: elements[0]}
	queue := []*TreeNode{root}

	for i := 1; i < len(elements); i += 2 {
		node := queue[0]
		queue = queue[1:]

		if elements[i] != -1 {
			node.Left = &TreeNode{Val: elements[i]}
			queue = append(queue, node.Left)
		}

		if i+1 < len(elements) && elements[i+1] != -1 {
			node.Right = &TreeNode{Val: elements[i+1]}
			queue = append(queue, node.Right)
		}
	}

	return root
}

func TraverseTree(root *TreeNode) []int {
	// level order traversal
	var result []int
	queue := []*TreeNode{root}

	for len(queue) > 0 {
		node := queue[0]
		queue = queue[1:]

		if node == nil {
			continue
		}

		result = append(result, node.Val)
		queue = append(queue, node.Left, node.Right)
	}

	return result
}
