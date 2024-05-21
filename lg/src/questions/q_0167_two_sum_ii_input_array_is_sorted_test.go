package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_twoSum(t *testing.T) {
	type args struct {
		numbers []int
		target  int
	}
	tests := []struct {
		name string
		args args
		want []int
	}{
		{
			"test-case-1",
			args{[]int{2, 7, 11, 15}, 9},
			[]int{1, 2},
		},
		{
			"test-case-2",
			args{[]int{2, 3, 4}, 6},
			[]int{1, 3},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.want, twoSum(tt.args.numbers, tt.args.target))
		})
	}
}
