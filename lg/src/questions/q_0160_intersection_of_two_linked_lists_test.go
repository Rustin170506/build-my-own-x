package questions

import (
	"testing"

	"github.com/hi-rustin/lg/src/utils"
	"github.com/stretchr/testify/assert"
)

func TestGetIntersectionNode(t *testing.T) {
	// Test case 1: Intersection at node with value 8
	intersection := &utils.ListNode{Val: 8, Next: &utils.ListNode{Val: 10}}
	headA := &utils.ListNode{Val: 3, Next: &utils.ListNode{Val: 7, Next: intersection}}
	headB := &utils.ListNode{Val: 99, Next: &utils.ListNode{Val: 1, Next: intersection}}

	result := getIntersectionNode(headA, headB)
	assert.Equal(t, intersection, result)

	// Test case 2: No intersection
	headA = &utils.ListNode{Val: 1, Next: &utils.ListNode{Val: 2, Next: &utils.ListNode{Val: 3}}}
	headB = &utils.ListNode{Val: 4, Next: &utils.ListNode{Val: 5, Next: &utils.ListNode{Val: 6}}}

	result = getIntersectionNode(headA, headB)
	assert.Nil(t, result)

	// Test case 3: Intersection at the beginning
	intersection = &utils.ListNode{Val: 1, Next: &utils.ListNode{Val: 2, Next: &utils.ListNode{Val: 3}}}
	headA = intersection
	headB = intersection

	result = getIntersectionNode(headA, headB)
	assert.Equal(t, intersection, result)

	// Test case 4: One list is empty
	headA = nil
	headB = &utils.ListNode{Val: 1, Next: &utils.ListNode{Val: 2, Next: &utils.ListNode{Val: 3}}}

	result = getIntersectionNode(headA, headB)
	assert.Nil(t, result)

	// Test case 5: Both lists are empty
	headA = nil
	headB = nil

	result = getIntersectionNode(headA, headB)
	assert.Nil(t, result)
}
