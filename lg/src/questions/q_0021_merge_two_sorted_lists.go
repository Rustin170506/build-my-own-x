package questions

import "github.com/hi-rustin/lg/src/utils"

func mergeTwoLists(list1 *utils.ListNode, list2 *utils.ListNode) *utils.ListNode {
	var head, tail *utils.ListNode

	for list1 != nil && list2 != nil {
		var next *utils.ListNode
		if list1.Val <= list2.Val {
			next = list1
			list1 = list1.Next
		} else {
			next = list2
			list2 = list2.Next
		}

		if head == nil {
			head = next
		} else {
			tail.Next = next
		}
		tail = next
	}

	if list1 != nil {
		if head == nil {
			head = list1
		} else {
			tail.Next = list1
		}
	} else if list2 != nil {
		if head == nil {
			head = list2
		} else {
			tail.Next = list2
		}
	}

	return head
}
