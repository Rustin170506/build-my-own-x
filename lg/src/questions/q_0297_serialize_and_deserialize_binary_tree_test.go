package questions

import (
	"testing"

	"github.com/hi-rustin/lg/src/utils"
	"github.com/stretchr/testify/assert"
)

func TestCodec_deserialize(t *testing.T) {
	type args struct {
		data string
	}
	tests := []struct {
		name string
		args args
		want *utils.TreeNode
	}{
		{
			name: "Example 1",
			args: args{
				data: "1,2,null,null,3,4,null,null,5,null,null",
			},
			want: utils.BuildTree([]int{1, 2, 3, -1, -1, 4, 5}),
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			this := &Codec{}
			assert.Equalf(t, utils.TraverseTree(tt.want), utils.TraverseTree(this.deserialize(tt.args.data)), "deserialize(%v)", tt.args.data)
		})
	}
}

func TestCodec_serialize(t *testing.T) {
	type args struct {
		root *utils.TreeNode
	}
	tests := []struct {
		name string
		args args
		want string
	}{
		{
			name: "Example 1",
			args: args{
				root: utils.BuildTree([]int{1, 2, 3, -1, -1, 4, 5}),
			},
			want: "1,2,null,null,3,4,null,null,5,null,null",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			this := &Codec{}
			assert.Equalf(t, tt.want, this.serialize(tt.args.root), "serialize(%v)", tt.args.root)
		})
	}
}
