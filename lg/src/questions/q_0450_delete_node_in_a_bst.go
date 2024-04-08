package questions

import "github.com/hi-rustin/lg/src/utils"

func deleteNode(root *utils.TreeNode, key int) *utils.TreeNode {
	// Empty node.
	if root == nil {
		return nil
	}

	// Cannot find this node.
	parent, node := findNode(root, key)
	if node == nil {
		return root
	}

	// Leaf node.
	if node.Left == nil && node.Right == nil {
		// This means deleting root node.
		if parent == nil {
			return nil
		}
		if parent.Left == node {
			parent.Left = nil
		} else {
			parent.Right = nil
		}
	} else if node.Left == nil || node.Right == nil { // Only one child.
		var child *utils.TreeNode
		if node.Left != nil {
			child = node.Left
		} else {
			child = node.Right
		}
		// This means deleting root node.
		if parent == nil {
			return child
		}
		if parent.Left == node {
			parent.Left = child
		} else {
			parent.Right = child
		}
	} else {
		// Has two children.
		_, minNode := findSmallest(node.Right)
		minValue := minNode.Val
		// Delete the smallest node in the right subtree recursively.
		deleteNode(root, minValue)
		node.Val = minValue
	}

	return root
}

func findSmallest(root *utils.TreeNode) (*utils.TreeNode, *utils.TreeNode) {
	if root == nil {
		return nil, nil
	}

	var parent *utils.TreeNode
	current := root
	for current.Left != nil {
		parent = current
		current = current.Left
	}

	return parent, current
}

func findNode(root *utils.TreeNode, value int) (*utils.TreeNode, *utils.TreeNode) {
	var parent *utils.TreeNode
	current := root
	for current != nil {
		if value > current.Val {
			parent = current
			current = current.Right
		} else if value < current.Val {
			parent = current
			current = current.Left
		} else {
			return parent, current
		}
	}
	return nil, nil
}
