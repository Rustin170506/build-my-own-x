package questions

import (
	"testing"

	"github.com/hi-rustin/lg/src/utils"
	"github.com/stretchr/testify/require"
)

func TestAddTwoNumbers(t *testing.T) {
	l1 := utils.BuildList([]int{2, 4, 3})
	l2 := utils.BuildList([]int{5, 6, 4})

	head := addTwoNumbers(l1, l2)
	require.Equal(t, []int{7, 0, 8}, utils.TraverseList(head))

	l1 = utils.BuildList([]int{0})
	l2 = utils.BuildList([]int{0})
	head = addTwoNumbers(l1, l2)
	require.Equal(t, []int{0}, utils.TraverseList(head))

	l1 = utils.BuildList([]int{9, 9, 9, 9, 9, 9, 9})
	l2 = utils.BuildList([]int{9, 9, 9, 9})
	head = addTwoNumbers(l1, l2)
	require.Equal(t, []int{8, 9, 9, 9, 0, 0, 0, 1}, utils.TraverseList(head))
}
