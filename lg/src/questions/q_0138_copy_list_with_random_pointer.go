package questions

type RandomNode struct {
	Val    int
	Next   *RandomNode
	Random *RandomNode
}

func copyRandomList(head *RandomNode) *RandomNode {
	if head == nil {
		return nil
	}

	nodeMap := make(map[*RandomNode]*RandomNode)
	current := head
	for current != nil {
		newNode := &RandomNode{
			Val: current.Val,
		}
		nodeMap[current] = newNode
		current = current.Next
	}

	current = head
	for current != nil {
		newNode := nodeMap[current]
		if current.Next != nil {
			newNode.Next = nodeMap[current.Next]
		}
		if current.Random != nil {
			newNode.Random = nodeMap[current.Random]
		}
		current = current.Next
	}

	return nodeMap[head]
}
