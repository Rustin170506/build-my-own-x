package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_orangesRotting(t *testing.T) {
	type args struct {
		grid [][]int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "testcase 1",
			args: args{
				grid: [][]int{
					{2, 1, 1},
					{1, 1, 0},
					{0, 1, 1},
				},
			},
			want: 4,
		},
		{
			name: "testcase 2",
			args: args{
				grid: [][]int{
					{2, 1, 1},
					{0, 1, 0},
					{1, 0, 1},
				},
			},
			want: -1,
		},
		{
			name: "testcase 3",
			args: args{
				grid: [][]int{
					{0, 2, 2},
				},
			},
			want: 0,
		},
		{
			name: "testcase 5",
			args: args{
				grid: [][]int{
					{1},
				},
			},
			want: -1,
		},
		{
			name: "testcase 6",
			args: args{
				grid: [][]int{
					{1, 2},
				},
			},
			want: 1,
		},
		{
			name: "testcase 7",
			args: args{
				grid: [][]int{
					{2, 1, 1},
					{1, 1, 1},
					{0, 1, 2},
				},
			},
			want: 2,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equalf(t, tt.want, orangesRotting(tt.args.grid), "orangesRotting(%v)", tt.args.grid)
		})
	}
}
