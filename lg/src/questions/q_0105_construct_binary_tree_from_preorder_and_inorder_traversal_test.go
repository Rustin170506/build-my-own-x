package questions

import (
	"testing"

	"github.com/hi-rustin/lg/src/utils"
	"github.com/stretchr/testify/require"
)

func TestBuildTree(t *testing.T) {
	tests := []struct {
		preorder []int
		inorder  []int
		want     *utils.TreeNode
	}{
		{
			preorder: []int{3, 9, 20, 15, 7},
			inorder:  []int{9, 3, 15, 20, 7},
			want:     utils.BuildTree([]int{3, 9, 20, 15, 7}),
		},
		{
			preorder: []int{1, 2},
			inorder:  []int{2, 1},
			want:     utils.BuildTree([]int{1, 2}),
		},
		{
			preorder: []int{1, 2, 3},
			inorder:  []int{2, 1, 3},
			want:     utils.BuildTree([]int{1, 2, 3}),
		},
	}
	for _, tt := range tests {
		require.Equal(t, utils.TraverseTree(tt.want), utils.TraverseTree(buildTree(tt.preorder, tt.inorder)))
	}
}
