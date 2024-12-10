package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestKthLargest(t *testing.T) {
	tests := []struct {
		k      int
		nums   []int
		values []int
		want   []int
	}{
		{
			k:      3,
			nums:   []int{4, 5, 8, 2},
			values: []int{3, 5, 10, 9, 4},
			want:   []int{4, 5, 5, 8, 8},
		},
		{
			k:      1,
			nums:   []int{},
			values: []int{-3, -2, -4, 0, 4},
			want:   []int{-3, -2, -2, 0, 4},
		},
	}

	for _, tt := range tests {
		got := Constructor1(tt.k, tt.nums)
		for i, v := range tt.values {
			require.Equal(t, tt.want[i], got.Add(v))
		}
	}
}
