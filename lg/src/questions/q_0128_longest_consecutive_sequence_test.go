package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_longestConsecutive(t *testing.T) {
	type args struct {
		nums []int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "test case 1",
			args: args{
				nums: []int{100, 4, 200, 1, 3, 2},
			},
			want: 4,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.want, longestConsecutive(tt.args.nums))
		})
	}
}
