package questions

import (
	"testing"

	"github.com/hi-rustin/lg/src/utils"
	"github.com/stretchr/testify/assert"
)

func Test_isSubtree(t *testing.T) {
	type args struct {
		root    *utils.TreeNode
		subRoot *utils.TreeNode
	}
	tests := []struct {
		name string
		args args
		want bool
	}{
		{
			name: "Example 1",
			args: args{
				root:    utils.BuildTree([]int{3, 4, 5, 1, 2}),
				subRoot: utils.BuildTree([]int{4, 1, 2}),
			},
			want: true,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equalf(t, tt.want, isSubtree(tt.args.root, tt.args.subRoot), "isSubtree(%v, %v)", tt.args.root, tt.args.subRoot)
		})
	}
}
