package questions

import (
	"fmt"
	"sort"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestSubsets(t *testing.T) {
	tests := []struct {
		nums []int
		want [][]int
	}{
		{
			nums: []int{1, 2, 3},
			want: [][]int{
				{},
				{1},
				{2},
				{1, 2},
				{3},
				{1, 3},
				{2, 3},
				{1, 2, 3},
			},
		},
		{
			nums: []int{0},
			want: [][]int{
				{},
				{0},
			},
		},
	}
	for _, tt := range tests {
		got := subsets(tt.nums)

		// Sort the result for comparison.
		for _, arr := range got {
			sort.Ints(arr)
		}
		for _, arr := range tt.want {
			sort.Ints(arr)
		}
		sort.Slice(got, func(i, j int) bool {
			return fmt.Sprint(got[i]) < fmt.Sprint(got[j])
		})
		sort.Slice(tt.want, func(i, j int) bool {
			return fmt.Sprint(tt.want[i]) < fmt.Sprint(tt.want[j])
		})

		require.Equal(t, tt.want, got)
	}
}
