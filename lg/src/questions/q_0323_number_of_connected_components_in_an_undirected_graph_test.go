package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCountComponents(t *testing.T) {
	type args struct {
		n     int
		edges [][]int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "test-case-1",
			args: args{
				n: 5,
				edges: [][]int{
					{0, 1},
					{1, 2},
					{3, 4},
				},
			},
			want: 2,
		},
		{
			name: "test-case-2",
			args: args{
				n: 5,
				edges: [][]int{
					{0, 1},
					{1, 2},
					{2, 3},
					{3, 4},
				},
			},
			want: 1,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equalf(t, tt.want, CountComponents(tt.args.n, tt.args.edges), "CountComponents(%v, %v)", tt.args.n, tt.args.edges)
		})
	}
}
