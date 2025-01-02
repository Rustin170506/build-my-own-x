package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstruct(t *testing.T) {
	tests := []struct {
		name     string
		grid     [][]int
		expected *QuadNode
	}{
		{
			name: "4x4 grid with different quadrants",
			grid: [][]int{
				{1, 1, 0, 0},
				{1, 1, 0, 0},
				{0, 0, 1, 1},
				{0, 0, 1, 1},
			},
			expected: &QuadNode{
				Val:    false,
				IsLeaf: false,
				TopLeft: &QuadNode{
					Val:    true,
					IsLeaf: true,
				},
				TopRight: &QuadNode{
					Val:    false,
					IsLeaf: true,
				},
				BottomLeft: &QuadNode{
					Val:    false,
					IsLeaf: true,
				},
				BottomRight: &QuadNode{
					Val:    true,
					IsLeaf: true,
				},
			},
		},
		{
			name: "4x4 grid with all ones",
			grid: [][]int{
				{1, 1, 1, 1},
				{1, 1, 1, 1},
				{1, 1, 1, 1},
				{1, 1, 1, 1},
			},
			expected: &QuadNode{
				Val:    true,
				IsLeaf: true,
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := construct(tt.grid)
			assert.Equal(t, tt.expected, result)
		})
	}
}
