package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestNumIslands(t *testing.T) {
	tests := []struct {
		grid   [][]byte
		expect int
	}{
		{
			grid: [][]byte{
				[]byte("11110"),
				[]byte("11010"),
				[]byte("11000"),
				[]byte("00000"),
			},
			expect: 1,
		},
		{
			grid: [][]byte{
				[]byte("11000"),
				[]byte("11000"),
				[]byte("00100"),
				[]byte("00011"),
			},
			expect: 3,
		},
	}

	for _, test := range tests {
		require.Equal(t, test.expect, numIslands(test.grid))
	}
}
