package questions

type Node struct {
	Val       int
	Neighbors []*Node
}

func cloneGraph(node *Node) *Node {
	if node == nil {
		return nil
	}

	cloned := make(map[*Node]struct{}, 4)
	var dfs func(n *Node) *Node
	dfs = func(n1 *Node) *Node {
		if n1 == nil {
			return nil
		}
		if _, ok := cloned[n1]; ok {
			return nil
		}

		cloned[n1] = struct{}{}
		newNode := &Node{Val: n1.Val, Neighbors: make([]*Node, 0)}
		for _, neighbor := range n1.Neighbors {
			newNode.Neighbors = append(newNode.Neighbors, dfs(neighbor))
		}

		return newNode
	}

	return dfs(node)
}
