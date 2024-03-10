package questions

import "github.com/hi-rustin/lg/src/utils"

func buildCycleList(arr []int, pos int) *utils.ListNode {
	var head, tail, cycleNode *utils.ListNode

	for i, val := range arr {
		newNode := &utils.ListNode{Val: val}
		if head == nil {
			head = newNode
		} else {
			tail.Next = newNode
		}
		tail = newNode

		if i == pos {
			cycleNode = newNode
		}
	}

	if pos != -1 {
		tail.Next = cycleNode
	}

	return head
}

func hasCycle(head *utils.ListNode) bool {
	if head == nil {
		return false
	}
	nodeMap := make(map[*utils.ListNode]struct{}, 2)
	for head != nil {
		if _, ok := nodeMap[head]; ok {
			return true
		}
		nodeMap[head] = struct{}{}
		head = head.Next
	}
	return false
}
