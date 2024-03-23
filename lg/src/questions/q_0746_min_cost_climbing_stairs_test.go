package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestMinCostClimbingStairs(t *testing.T) {
	tests := []struct {
		testName string
		input    []int
		expected int
	}{
		{
			testName: "Example 1",
			input:    []int{10, 15, 20},
			expected: 15,
		},
		{
			testName: "Example 2",
			input:    []int{1, 100, 1, 1, 1, 100, 1, 1, 100, 1},
			expected: 6,
		},
	}

	for _, test := range tests {
		t.Run(test.testName, func(t *testing.T) {
			got := minCostClimbingStairs(test.input)
			require.Equal(t, test.expected, got)
		})
	}
}
