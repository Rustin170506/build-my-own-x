package questions

import (
	"testing"

	"github.com/hi-rustin/lg/src/utils"
	"github.com/stretchr/testify/require"
)

func TestReverseKGroup(t *testing.T) {
	list := utils.BuildList([]int{1, 2, 3, 4, 5})
	require.Equal(t, []int{2, 1, 4, 3, 5}, utils.TraverseList(reverseKGroup(list, 2)))
	require.Equal(t, []int{3, 2, 1, 4, 5}, utils.TraverseList(reverseKGroup(list, 3)))
}
