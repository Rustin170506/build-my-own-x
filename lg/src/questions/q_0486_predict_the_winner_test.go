package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_predictTheWinner(t *testing.T) {
	type args struct {
		nums []int
	}
	tests := []struct {
		name string
		args args
		want bool
	}{
		{
			name: "testcase 1",
			args: args{
				nums: []int{1, 5, 2},
			},
			want: false,
		},
		{
			name: "testcase 2",
			args: args{
				nums: []int{1, 5, 233, 7},
			},
			want: true,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.want, predictTheWinner(tt.args.nums))
		})
	}
}
