package questions

import "github.com/hi-rustin/lg/src/utils"

func mergeKLists(lists []*utils.ListNode) *utils.ListNode {
	if len(lists) == 0 {
		return nil
	}
	if len(lists) == 1 {
		return lists[0]
	}

	for len(lists) > 1 {
		list1 := lists[0]
		lists = lists[1:]
		list2 := lists[0]
		lists = lists[1:]
		lists = append(lists, MergeTwoLists(list1, list2))
	}
	return lists[0]
}

func MergeTwoLists(list1 *utils.ListNode, list2 *utils.ListNode) *utils.ListNode {
	if list1 == nil && list2 == nil {
		return nil
	}
	if list1 == nil {
		return list2
	}
	if list2 == nil {
		return list1
	}

	var head *utils.ListNode
	var tail *utils.ListNode

	for list1 != nil && list2 != nil {
		var current *utils.ListNode
		if list1.Val < list2.Val {
			current = list1
			list1 = list1.Next
		} else {
			current = list2
			list2 = list2.Next
		}
		if head == nil {
			head = current
			tail = current
		} else {
			tail.Next = current
			tail = current
		}
	}

	if list1 != nil {
		if head == nil {
			head = list1
		} else {
			tail.Next = list1
		}
	}
	if list2 != nil {
		if head == nil {
			head = list2
		} else {
			tail.Next = list2
		}
	}

	return head
}
