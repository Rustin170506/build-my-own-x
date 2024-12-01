package questions

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/hi-rustin/lg/src/utils"
)

func TestInvertTree(t *testing.T) {
	tests := []struct {
		root     *utils.TreeNode
		expected []int
	}{
		{
			root:     utils.BuildTree([]int{4, 2, 7, 1, 3, 6, 9}),
			expected: []int{4, 7, 2, 9, 6, 3, 1},
		},
		{
			root:     utils.BuildTree([]int{2, 1, 3}),
			expected: []int{2, 3, 1},
		},
		{
			root:     utils.BuildTree([]int{}),
			expected: nil,
		},
	}

	for _, test := range tests {
		require.Equal(t, test.expected, utils.TraverseTree(invertTree(test.root)))
	}
}

func TestInvertTreeBreadthFirst(t *testing.T) {
	tests := []struct {
		root     *utils.TreeNode
		expected []int
	}{
		{
			root:     utils.BuildTree([]int{4, 2, 7, 1, 3, 6, 9}),
			expected: []int{4, 7, 2, 9, 6, 3, 1},
		},
		{
			root:     utils.BuildTree([]int{2, 1, 3}),
			expected: []int{2, 3, 1},
		},
		{
			root:     utils.BuildTree([]int{}),
			expected: nil,
		},
	}

	for _, test := range tests {
		require.Equal(t, test.expected, utils.TraverseTree(invertTreeBreadthFirst(test.root)))
	}
}
