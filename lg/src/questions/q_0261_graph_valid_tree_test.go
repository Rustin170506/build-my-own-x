package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestValidTree(t *testing.T) {
	type args struct {
		n     int
		edges [][]int
	}
	tests := []struct {
		name string
		args args
		want bool
	}{
		{
			name: "test-case-1",
			args: args{
				n: 5,
				edges: [][]int{
					{0, 1},
					{0, 2},
					{0, 3},
					{1, 4},
				},
			},
			want: true,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equalf(t, tt.want, ValidTree(tt.args.n, tt.args.edges), "ValidTree(%v, %v)", tt.args.n, tt.args.edges)
		})
	}
}
