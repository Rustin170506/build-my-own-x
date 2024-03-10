package questions

import "github.com/hi-rustin/lg/src/utils"

func reverseList(head *utils.ListNode) *utils.ListNode {
	if head == nil {
		return nil
	}

	prev := head
	current := head.Next
	prev.Next = nil
	for current != nil {
		next := current.Next
		current.Next = prev
		prev = current
		current = next
	}

	return prev
}
