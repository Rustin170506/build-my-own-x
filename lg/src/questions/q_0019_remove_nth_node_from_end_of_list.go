package questions

import "github.com/hi-rustin/lg/src/utils"

func removeNthFromEnd(head *utils.ListNode, n int) *utils.ListNode {
	// Handle empty list
	if head == nil {
		return nil
	}

	totalLen := 0
	current := head
	for current != nil {
		current = current.Next
		totalLen++
	}

	prevIndex := totalLen - n
	if prevIndex < 0 {
		return head
	}

	// Use dummy node to handle removal of first node
	dummy := &utils.ListNode{Next: head}
	// Start from dummy node
	prev := dummy
	for i := 0; i < prevIndex; i++ {
		prev = prev.Next
	}

	prev.Next = prev.Next.Next
	return dummy.Next
}
