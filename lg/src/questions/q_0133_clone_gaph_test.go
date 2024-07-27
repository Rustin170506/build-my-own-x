package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestCloneGraph(t *testing.T) {
	tests := []struct {
		name string
		node *Node
	}{
		{
			name: "testcase 1",
			node: &Node{
				Val: 1,
				Neighbors: []*Node{
					{
						Val: 2,
						Neighbors: []*Node{
							{
								Val: 4,
								Neighbors: []*Node{
									{
										Val: 1,
									},
									{
										Val: 3,
									},
								},
							},
						},
					},
					{
						Val: 3,
						Neighbors: []*Node{
							{
								Val: 4,
								Neighbors: []*Node{
									{
										Val: 1,
									},
									{
										Val: 2,
									},
								},
							},
						},
					},
				},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			clone := cloneGraph(tt.node)

			visited := make(map[*Node]*Node)
			var dfs func(node, clone *Node)
			dfs = func(node, clone *Node) {
				if node == nil {
					return
				}
				if visited[node] != nil {
					require.Equal(t, visited[node], clone)
					return
				}
				visited[node] = clone

				require.Equal(t, node.Val, clone.Val)
				require.NotEqual(t, node, clone)

				for i := range node.Neighbors {
					dfs(node.Neighbors[i], clone.Neighbors[i])
				}
			}
			dfs(tt.node, clone)
			clone1 := cloneGraphBfs(tt.node)
			dfs(tt.node, clone1)
		})
	}
}
