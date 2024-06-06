package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_lengthOfLIS(t *testing.T) {
	type args struct {
		nums []int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{"test", args{[]int{10, 9, 2, 5, 3, 7, 101, 18}}, 4},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.want, lengthOfLIS(tt.args.nums))
			require.Equal(t, tt.want, lengthOfLISRecursive(tt.args.nums))
		})
	}
}
