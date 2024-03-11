package questions

import (
	"github.com/hi-rustin/lg/src/utils"
)

func reorderList(head *utils.ListNode) {
	n := 0

	// Get the total length.
	current := head
	for current != nil {
		current = current.Next
		n += 1
	}

	if n == 1 {
		return // No need to reorder.
	}

	// FInd the middle position.
	middle := n / 2
	current = head
	for i := 0; i < middle; i++ {
		current = current.Next
	}

	// Revert the second half.
	var prev *utils.ListNode
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
