package questions

type Node struct {
	Val       int
	Neighbors []*Node
}

func cloneGraph(node *Node) *Node {
	if node == nil {
		return nil
	}

	cloned := make(map[*Node]*Node)
	var dfs func(current *Node) *Node
	dfs = func(current *Node) *Node {
		if current == nil {
			return nil
		}

		if n, ok := cloned[current]; ok {
			return n
		}

		newNode := &Node{
			Val:       current.Val,
			Neighbors: make([]*Node, len(current.Neighbors)),
		}
		cloned[current] = newNode

		for i, neighbor := range current.Neighbors {
			newNode.Neighbors[i] = dfs(neighbor)
		}

		return newNode
	}

	return dfs(node)
}

func cloneGraphBfs(node *Node) *Node {
	if node == nil {
		return nil
	}

	result := &Node{Val: node.Val, Neighbors: make([]*Node, 0)}
	cloned := map[*Node]*Node{node: result}
	queue := []*Node{node}
	for len(queue) > 0 {
		n := queue[0]
		queue = queue[1:]
		for _, neighbor := range n.Neighbors {
			if _, ok := cloned[neighbor]; !ok {
				cloned[neighbor] = &Node{Val: neighbor.Val, Neighbors: make([]*Node, 0)}
				queue = append(queue, neighbor)
			}
			cloned[n].Neighbors = append(cloned[n].Neighbors, cloned[neighbor])
		}
	}
	return result
}
