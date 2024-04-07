package questions

import (
	"testing"

	"github.com/hi-rustin/lg/src/utils"
	"github.com/stretchr/testify/assert"
)

func Test_diameterOfBinaryTree(t *testing.T) {
	type args struct {
		root *utils.TreeNode
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "case1",
			args: args{root: utils.BuildTree([]int{1, 2, 3, 4, 5})},
			want: 3,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equalf(t, tt.want, diameterOfBinaryTree(tt.args.root), "diameterOfBinaryTree(%v)", tt.args.root)
		})
	}
}
