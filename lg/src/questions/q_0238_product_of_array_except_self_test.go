package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_productExceptSelf(t *testing.T) {
	type args struct {
		nums []int
	}
	tests := []struct {
		name string
		args args
		want []int
	}{
		{
			"test-case-1",
			args{[]int{1, 2, 3, 4}},
			[]int{24, 12, 8, 6},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.want, productExceptSelf(tt.args.nums))
		})
	}
}
