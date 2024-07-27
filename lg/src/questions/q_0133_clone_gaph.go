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
