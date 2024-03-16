package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"

	"github.com/hi-rustin/lg/src/utils"
)

func TestHasCycle(t *testing.T) {
	tests := []struct {
		testName string
		head     *utils.ListNode
		expected bool
	}{
		{
			testName: "test #1",
			head:     buildCycleList([]int{3, 2, 0, -4}, 1),
			expected: true,
		},
		{
			testName: "test #2",
			head:     buildCycleList([]int{1, 2}, 0),
			expected: true,
		},
		{
			testName: "test #3",
			head:     buildCycleList([]int{1}, -1),
			expected: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.testName, func(t *testing.T) {
			actual := hasCycle(tt.head)
			assert.Equal(t, tt.expected, actual)
		})
	}
}
