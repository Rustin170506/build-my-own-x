package questions

import (
	"testing"

	"github.com/hi-rustin/lg/src/utils"
	"github.com/stretchr/testify/assert"
)

func Test_isValidBST(t *testing.T) {
	type args struct {
		root *utils.TreeNode
	}
	tests := []struct {
		name string
		args args
		want bool
	}{
		{
			name: "test-case-1",
			args: args{
				root: utils.BuildTree([]int{2, 1, 3}),
			},
			want: true,
		},
		{
			name: "test-case-2",
			args: args{
				root: utils.BuildTree([]int{5, 1, 4, -1, -1, 3, 6}),
			},
			want: false,
		},
		{
			name: "test-case-3",
			args: args{
				root: utils.BuildTree([]int{5, 4, 6, -1, -1, 3, 7}),
			},
			want: false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equalf(t, tt.want, isValidBST(tt.args.root), "isValidBST(%v)", tt.args.root)
		})
	}
}
