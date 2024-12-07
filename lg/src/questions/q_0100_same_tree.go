package questions

import "github.com/hi-rustin/lg/src/utils"

func isSameTree(p *utils.TreeNode, q *utils.TreeNode) bool {
	if p == nil && q == nil {
		return true
	}
	if p == nil || q == nil {
		return false
	}
	if p.Val != q.Val {
		return false
	}

	return isSameTree(p.Left, q.Left) && isSameTree(p.Right, q.Right)
}

func isSameTreeWithLevelTraversal(p *utils.TreeNode, q *utils.TreeNode) bool {
	queue1 := []*utils.TreeNode{p}
	queue2 := []*utils.TreeNode{q}

	for len(queue1) != 0 && len(queue2) != 0 {
		node1, node2 := queue1[0], queue2[0]
		queue1, queue2 = queue1[1:], queue2[1:]

		if node1 == nil && node2 == nil {
			continue
		}
		if node1 == nil || node2 == nil {
			return false
		}
		if node1.Val != node2.Val {
			return false
		}

		queue1 = append(queue1, node1.Left, node1.Right)
		queue2 = append(queue2, node2.Left, node2.Right)
	}

	return len(queue1) == 0 && len(queue2) == 0
}
