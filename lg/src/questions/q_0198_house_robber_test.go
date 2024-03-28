package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestRob(t *testing.T) {
	tests := []struct {
		in   []int
		want int
	}{
		{[]int{1, 2, 3, 1}, 4},
		{[]int{2, 7, 9, 3, 1}, 12},
		{[]int{2, 1, 1, 2}, 4},
		{[]int{1, 2, 1, 1}, 3},
	}

	for i, tt := range tests {
		got := rob(tt.in)
		require.Equal(t, tt.want, got, "test case %d failed", i)
	}
}
