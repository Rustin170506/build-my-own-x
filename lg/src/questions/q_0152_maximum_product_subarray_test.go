package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_maxProduct(t *testing.T) {
	type args struct {
		nums []int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "test case 0",
			args: args{nums: []int{2, 3, -2, 4}},
			want: 6,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.want, maxProduct(tt.args.nums))
		})
	}
}
