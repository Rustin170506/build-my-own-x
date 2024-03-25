package questions

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestMinCostConnectPoints(t *testing.T) {
	tests := []struct {
		points [][]int
		want   int
	}{
		{
			points: [][]int{{0, 0}, {2, 2}, {3, 10}, {5, 2}, {7, 0}},
			want:   20,
		},
		{
			points: [][]int{{3, 12}, {-2, 5}, {-4, 1}},
			want:   18,
		},
		{
			points: [][]int{{0, 0}, {1, 1}, {1, 0}, {-1, 1}},
			want:   4,
		},
		{
			points: [][]int{{-1000000, -1000000}, {1000000, 1000000}},
			want:   4000000,
		},
		{
			points: [][]int{{0, 0}},
			want:   0,
		},
	}
	for _, tt := range tests {
		t.Run(fmt.Sprintf("%v", tt.points), func(t *testing.T) {
			require.Equal(t, tt.want, minCostConnectPoints(tt.points))
		})
	}
}
