package questions

import (
	"testing"

	"github.com/hi-rustin/lg/src/utils"
	"github.com/stretchr/testify/assert"
)

func Test_deleteNode(t *testing.T) {
	type args struct {
		root *utils.TreeNode
		key  int
	}
	tests := []struct {
		name string
		args args
		want *utils.TreeNode
	}{
		{
			name: "case1",
			args: args{
				root: utils.BuildTree([]int{5, 3, 6, 2, 4, -1, 7}),
				key:  3,
			},
			want: utils.BuildTree([]int{5, 4, 6, 2, -1, -1, 7}),
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equalf(t, tt.want, deleteNode(tt.args.root, tt.args.key), "deleteNode(%v, %v)", tt.args.root, tt.args.key)
		})
	}
}
