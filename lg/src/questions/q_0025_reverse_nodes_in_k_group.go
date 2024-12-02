package questions

import "github.com/hi-rustin/lg/src/utils"

func reverseKGroup(head *utils.ListNode, k int) *utils.ListNode {
	list := make([]int, 0, 2*k)

	current := head
	for current != nil {
		list = append(list, current.Val)
		current = current.Next
	}

	for i := 0; i < len(list); i += k {
		start := i
		end := i + k - 1
		if end >= len(list) {
			break
		}
		for start < end {
			list[start], list[end] = list[end], list[start]
			start++
			end--
		}
	}

	var newHead *utils.ListNode
	next := newHead

	for _, v := range list {
		node := &utils.ListNode{
			Val: v,
		}
		if newHead == nil {
			newHead = node
			next = node
		} else {
			next.Next = node
			next = node
		}
	}

	return newHead
}
