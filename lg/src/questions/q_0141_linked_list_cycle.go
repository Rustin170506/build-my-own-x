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

	cycleMap := make(map[*utils.ListNode]struct{})
	current := head
	for current != nil {
		if _, ok := cycleMap[current]; ok {
			return true
		}
		cycleMap[current] = struct{}{}
		current = current.Next
	}

	return false
}
