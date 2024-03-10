package utils

type ListNode struct {
	Val  int
	Next *ListNode
}

func BuildList(elements []int) *ListNode {
	if len(elements) == 0 {
		return nil
	}

	head := &ListNode{
		Val:  elements[0],
		Next: nil,
	}
	current := head
	for _, e := range elements[1:] {
		newNode := &ListNode{
			Val:  e,
			Next: nil,
		}
		current.Next = newNode
		current = newNode
	}

	return head
}

func TraverseList(head *ListNode) []int {
	var result []int
	for head != nil {
		result = append(result, head.Val)
		head = head.Next
	}
	return result
}
