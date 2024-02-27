package questions

type ListNode struct {
	Val  int
	Next *ListNode
}

func buildCycleList(arr []int, pos int) *ListNode {
	var head, tail, cycleNode *ListNode

	for i, val := range arr {
		newNode := &ListNode{Val: val}
		if head == nil {
			head = newNode
		} else {
			tail.Next = newNode
		}
		tail = newNode

		if i == pos {
			cycleNode = newNode
		}
	}

	if pos != -1 {
		tail.Next = cycleNode
	}

	return head
}

func hasCycle(head *ListNode) bool {
	if head == nil {
		return false
	}
	nodeMap := make(map[*ListNode]struct{}, 2)
	for head != nil {
		if _, ok := nodeMap[head]; ok {
			return true
		}
		nodeMap[head] = struct{}{}
		head = head.Next
	}
	return false
}
