package questions

type Node struct {
	Val       int
	Neighbors []*Node
}

func cloneGraph(node *Node) *Node {
	if node == nil {
		return nil
	}

	nodeMap := make(map[*Node]*Node, 4)
	var dfs func(node *Node) *Node
	dfs = func(node *Node) *Node {
		if n, ok := nodeMap[node]; ok {
			return n
		}

		newNode := &Node{
			Val:       node.Val,
			Neighbors: make([]*Node, 0, len(node.Neighbors)),
		}
		nodeMap[node] = newNode

		for _, n := range node.Neighbors {
			newNode.Neighbors = append(newNode.Neighbors, dfs(n))
		}

		return newNode
	}

	return dfs(node)
}
