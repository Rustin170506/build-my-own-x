package questions

import (
	"testing"

	"github.com/hi-rustin/lg/src/utils"
	"github.com/stretchr/testify/assert"
)

func Test_levelOrder(t *testing.T) {
	type args struct {
		root *utils.TreeNode
	}
	tests := []struct {
		name string
		args args
		want [][]int
	}{
		{
			name: "Example 1",
			args: args{
				root: utils.BuildTree([]int{3, 9, 20, -1, -1, 15, 7}),
			},
			want: [][]int{{3}, {9, 20}, {15, 7}},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equalf(t, tt.want, levelOrder(tt.args.root), "levelOrder(%v)", tt.args.root)
		})
	}
}
