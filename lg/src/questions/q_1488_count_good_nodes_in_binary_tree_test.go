package questions

import (
	"testing"

	"github.com/hi-rustin/lg/src/utils"
	"github.com/stretchr/testify/assert"
)

func Test_goodNodes(t *testing.T) {
	type args struct {
		root *utils.TreeNode
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "Example 1",
			args: args{
				root: utils.BuildTree([]int{3, 1, 4, 3, -1, 1, 5}),
			},
			want: 4,
		},
		{
			name: "Example 2",
			args: args{
				root: utils.BuildTree([]int{3, 3, -1, 4, 2}),
			},
			want: 3,
		},
		{
			name: "Example 3",
			args: args{
				root: utils.BuildTree([]int{2, -1, 4, 10, 8, -1, -1, 4}),
			},
			want: 4,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equalf(t, tt.want, goodNodes(tt.args.root), "goodNodes(%v)", tt.args.root)
		})
	}
}
