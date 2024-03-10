package questions

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/hi-rustin/lg/src/utils"
)

func TestReverseList(t *testing.T) {
	tests := []struct {
		head *utils.ListNode
		want *utils.ListNode
	}{
		{
			utils.BuildList([]int{1, 2, 3, 4, 5}),
			utils.BuildList([]int{5, 4, 3, 2, 1}),
		},
		{
			utils.BuildList([]int{1, 2}),
			utils.BuildList([]int{2, 1}),
		},
		{
			utils.BuildList([]int{1}),
			utils.BuildList([]int{1}),
		},
		{
			nil,
			nil,
		},
	}
	for _, tt := range tests {
		require.Equal(t, utils.TraverseList(tt.want), utils.TraverseList(reverseList(tt.head)))
	}
}
