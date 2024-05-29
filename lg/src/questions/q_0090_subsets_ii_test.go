package questions

import (
	"fmt"
	"sort"
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_subsetsWithDup(t *testing.T) {
	type args struct {
		nums []int
	}
	tests := []struct {
		name string
		args args
		want [][]int
	}{
		{
			name: "testcase 1",
			args: args{
				nums: []int{1, 2, 2},
			},
			want: [][]int{
				{},
				{1},
				{1, 2},
				{1, 2, 2},
				{2},
				{2, 2},
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := subsetsWithDup(tt.args.nums)

			// Sort the 2d array to make it easier to compare the result
			sort.Slice(got, func(i, j int) bool {
				return fmt.Sprint(got[i]) < fmt.Sprint(got[j])
			})
			sort.Slice(tt.want, func(i, j int) bool {
				return fmt.Sprint(tt.want[i]) < fmt.Sprint(tt.want[j])
			})
			require.Equal(t, tt.want, got)
		})
	}
}
