package questions

import "github.com/hi-rustin/lg/src/utils"

func addTwoNumbers(l1 *utils.ListNode, l2 *utils.ListNode) *utils.ListNode {
	if l1 == nil {
		return l2
	}
	if l2 == nil {
		return l1
	}
	newVal := l1.Val + l2.Val
	remain := 0
	if newVal > 9 {
		remain = 1
		newVal = newVal % 10
	}
	newHead := &utils.ListNode{
		Val: newVal,
	}
	current := newHead
	l1 = l1.Next
	l2 = l2.Next
	for l1 != nil && l2 != nil {
		newVal := l1.Val + l2.Val + remain
		remain = 0
		if newVal > 9 {
			remain = 1
			newVal = newVal % 10
		}
		newNode := &utils.ListNode{
			Val: newVal,
		}
		current.Next = newNode
		current = current.Next
		l1 = l1.Next
		l2 = l2.Next
	}

	for l1 != nil || l2 != nil {
		newVal := remain
		if l1 != nil {
			newVal += l1.Val
			l1 = l1.Next
		}
		if l2 != nil {
			newVal += l2.Val
			l2 = l2.Next
		}

		remain = 0
		if newVal > 9 {
			remain = 1
			newVal = newVal % 10
		}

		current.Next = &utils.ListNode{Val: newVal}
		current = current.Next
	}

	if remain > 0 {
		current.Next = &utils.ListNode{Val: remain}
	}

	return newHead
}
