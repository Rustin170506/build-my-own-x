package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func buildRandomList(list [][]int) *RandomNode {
	if len(list) == 0 {
		return nil
	}

	nodes := make([]*RandomNode, len(list))
	for i := range list {
		nodes[i] = &RandomNode{Val: list[i][0]}
	}

	for i, pair := range list {
		if i < len(nodes)-1 {
			nodes[i].Next = nodes[i+1]
		}
		if pair[1] != -1 {
			nodes[i].Random = nodes[pair[1]]
		}
	}

	return nodes[0]
}

func TestCopyRandomList(t *testing.T) {
	tests := []struct {
		name string
		list [][]int
	}{
		{
			name: "example 1",
			list: [][]int{{7, 1}, {13, 0}, {11, 4}, {10, 2}, {1, 0}},
		},
		{
			name: "example 2",
			list: [][]int{{1, 1}, {2, 1}},
		},
		{
			name: "example 3",
			list: [][]int{{3, -1}, {3, 0}, {3, -1}},
		},
		{
			name: "empty list",
			list: [][]int{},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			head := buildRandomList(tt.list)
			result := copyRandomList(head)

			// Handle empty list case
			if len(tt.list) == 0 {
				assert.Nil(t, result)
				return
			}

			// Create maps to store node positions
			origMap := make(map[*RandomNode]int)
			copyMap := make(map[*RandomNode]int)

			// Build position maps
			orig := head
			copy := result
			pos := 0
			for orig != nil {
				origMap[orig] = pos
				copyMap[copy] = pos
				pos++
				orig = orig.Next
				copy = copy.Next
			}

			// Verify structure
			orig = head
			copy = result
			for orig != nil {
				// Verify values match
				assert.Equal(t, orig.Val, copy.Val)

				// Verify nodes are different instances
				assert.NotSame(t, orig, copy)

				// Verify random pointers maintain same relationships
				if orig.Random != nil {
					assert.NotNil(t, copy.Random)
					origRandomPos := origMap[orig.Random]
					copyRandomPos := copyMap[copy.Random]
					assert.Equal(t, origRandomPos, copyRandomPos)
				} else {
					assert.Nil(t, copy.Random)
				}

				orig = orig.Next
				copy = copy.Next
			}
			assert.Nil(t, copy)
		})
	}
}
