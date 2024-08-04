package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_findRedundantConnection(t *testing.T) {
	type args struct {
		edges [][]int
	}
	tests := []struct {
		name string
		args args
		want []int
	}{
		{
			name: "testcase 1",
			args: args{
				edges: [][]int{
					{1, 2},
					{1, 3},
					{2, 3},
				},
			},
			want: []int{2, 3},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equalf(t, tt.want, findRedundantConnection(tt.args.edges), "findRedundantConnection(%v)", tt.args.edges)
		})
	}
}
