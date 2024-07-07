package questions

import (
	"testing"

	"github.com/hi-rustin/lg/src/utils"
	"github.com/stretchr/testify/assert"
)

func Test_kthSmallest(t *testing.T) {
	type args struct {
		root *utils.TreeNode
		k    int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "test-case-1",
			args: args{
				root: utils.BuildTree([]int{3, 1, 4, 0, 2}),
				k:    1,
			},
			want: 0,
		},
		{
			name: "test-case-2",
			args: args{
				root: utils.BuildTree([]int{5, 3, 6, 2, 4, -1, -1, 11}),
				k:    3,
			},
			want: 3,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equalf(t, tt.want, kthSmallest(tt.args.root, tt.args.k), "kthSmallest(%v, %v)", tt.args.root, tt.args.k)
			assert.Equalf(t, tt.want, kthSmallestWithStack(tt.args.root, tt.args.k), "kthSmallestWithStack(%v, %v)", tt.args.root, tt.args.k)
		})
	}
}
