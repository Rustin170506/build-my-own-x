package questions

import "github.com/hi-rustin/lg/src/utils"

func getIntersectionNode(headA, headB *utils.ListNode) *utils.ListNode {
	a, b := headA, headB
	for a != b {
		if a == nil {
			a = headB
		} else {
			a = a.Next
		}

		if b == nil {
			b = headA
		} else {
			b = b.Next
		}
	}
	return a
}
