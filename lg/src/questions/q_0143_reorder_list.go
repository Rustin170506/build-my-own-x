package questions

import (
	"github.com/hi-rustin/lg/src/utils"
)

func reorderList(head *utils.ListNode) {
	n := 0

	// Calculate the list length.
	current := head
	for current != nil {
		current = current.Next
		n++
	}

	if n == 1 {
		return
	}

	// Find the middle position of the list.
	secondHalfHead := head
	for i := 0; i < n/2; i++ {
		secondHalfHead = secondHalfHead.Next
	}

	// Reverse the second half.
	var prev *utils.ListNode
	current = secondHalfHead
	for current != nil {
		next := current.Next
		current.Next = prev
		prev = current
		current = next
	}

	// Merge two halves.
	first, second := head, prev
	for second.Next != nil {
		tmp := first.Next
		first.Next = second
		first = tmp

		tmp = second.Next
		second.Next = first
		second = tmp
	}
}

func reorderListFastAndSlowPointers(head *utils.ListNode) {
	if head == nil || head.Next == nil {
		return
	}

	// Find the middle of the list
	slow, fast := head, head
	for fast != nil && fast.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next
	}

	// Reverse the second half of the list
	var prev *utils.ListNode
	current := slow
	for current != nil {
		next := current.Next
		current.Next = prev
		prev = current
		current = next
	}

	// Merge the two halves
	first, second := head, prev
	for second.Next != nil {
		tmp := first.Next
		first.Next = second
		first = tmp

		tmp = second.Next
		second.Next = first
		second = tmp
	}
}
