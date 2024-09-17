package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_swimInWater(t *testing.T) {
	type args struct {
		grid [][]int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "test 1",
			args: args{
				grid: [][]int{{0, 2}, {1, 3}},
			},
			want: 3,
		},
		{
			name: "test 2",
			args: args{
				grid: [][]int{{0, 1, 2, 3, 4}, {24, 23, 22, 21, 5}, {12, 13, 14, 15, 16}, {11, 17, 18, 19, 20}, {10, 9, 8, 7, 6}},
			},
			want: 16,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.want, swimInWater(tt.args.grid))
		})
	}
}
