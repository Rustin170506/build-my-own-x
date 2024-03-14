package utils

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func BuildTree(elements []int) *TreeNode {
	return buildTree(elements, 0)
}

func buildTree(elements []int, index int) *TreeNode {
	if index >= len(elements) || elements[index] == -1 {
		return nil
	}

	node := &TreeNode{Val: elements[index]}
	node.Left = buildTree(elements, 2*index+1)
	node.Right = buildTree(elements, 2*index+2)

	return node
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
