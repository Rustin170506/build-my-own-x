package questions

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/hi-rustin/lg/src/utils"
)

func Test_lowestCommonAncestor(t *testing.T) {
	type args struct {
		root *utils.TreeNode
		p    *utils.TreeNode
		q    *utils.TreeNode
	}
	tests := []struct {
		name string
		args args
		want *utils.TreeNode
	}{
		{
			name: "test-case-1",
			args: args{
				root: utils.BuildTree([]int{6, 2, 8, 0, 4, 7, 9, -1, -1, 3, 5}),
				p:    &utils.TreeNode{Val: 2},
				q:    &utils.TreeNode{Val: 8},
			},
			want: &utils.TreeNode{Val: 6},
		},
	}
	for _, tt := range tests {
		require.Equal(t, tt.want.Val, lowestCommonAncestor(tt.args.root, tt.args.p, tt.args.q).Val)
		require.Equal(t, tt.want.Val, lowestCommonAncestorRecursive(tt.args.root, tt.args.p, tt.args.q).Val)
	}
}
