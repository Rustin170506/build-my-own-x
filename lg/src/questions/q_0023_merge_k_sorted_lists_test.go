package questions

import (
	"testing"

	"github.com/hi-rustin/lg/src/utils"
	"github.com/stretchr/testify/require"
)

func TestMergeKLists(t *testing.T) {
	lists := []*utils.ListNode{
		utils.BuildList([]int{1, 4, 5}),
		utils.BuildList([]int{1, 3, 4}),
		utils.BuildList([]int{2, 6}),
	}

	merged := mergeKLists(lists)
	require.Equal(t, []int{1, 1, 2, 3, 4, 4, 5, 6}, utils.TraverseList(merged))
}
