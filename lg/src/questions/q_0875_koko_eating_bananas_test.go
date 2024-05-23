package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_minEatingSpeed(t *testing.T) {
	type args struct {
		piles []int
		h     int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "test-case-1",
			args: args{
				piles: []int{3, 6, 7, 11},
				h:     8,
			},
			want: 4,
		},
		{
			name: "test-case-2",
			args: args{
				piles: []int{30, 11, 23, 4, 20},
				h:     5,
			},
			want: 30,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.want, minEatingSpeed(tt.args.piles, tt.args.h))
		})
	}
}
