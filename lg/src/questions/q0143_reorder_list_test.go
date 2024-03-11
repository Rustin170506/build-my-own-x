package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"

	"github.com/hi-rustin/lg/src/utils"
)

func TestReorderList(t *testing.T) {
	head := utils.BuildList([]int{1, 2, 3, 4})
	reorderList(head)
	assert.Equal(t, []int{1, 4, 2, 3}, utils.TraverseList(head))
	head = utils.BuildList([]int{1, 2, 3, 4, 5})
	reorderList(head)
	assert.Equal(t, []int{1, 5, 2, 4, 3}, utils.TraverseList(head))
	head = utils.BuildList([]int{1, 2, 3, 4, 5, 6})
	reorderList(head)
	assert.Equal(t, []int{1, 6, 2, 5, 3, 4}, utils.TraverseList(head))
	head = utils.BuildList([]int{1})
	reorderList(head)
	assert.Equal(t, []int{1}, utils.TraverseList(head))
	head = utils.BuildList([]int{1, 2})
	reorderList(head)
	assert.Equal(t, []int{1, 2}, utils.TraverseList(head))
	head = utils.BuildList([]int{1, 2, 3, 4})
	reorderListFastAndSlowPointers(head)
	assert.Equal(t, []int{1, 4, 2, 3}, utils.TraverseList(head))
	head = utils.BuildList([]int{1, 2, 3, 4, 5})
	reorderListFastAndSlowPointers(head)
	assert.Equal(t, []int{1, 5, 2, 4, 3}, utils.TraverseList(head))
	head = utils.BuildList([]int{1, 2, 3, 4, 5, 6})
	reorderListFastAndSlowPointers(head)
	assert.Equal(t, []int{1, 6, 2, 5, 3, 4}, utils.TraverseList(head))
	head = utils.BuildList([]int{1})
	reorderListFastAndSlowPointers(head)
	assert.Equal(t, []int{1}, utils.TraverseList(head))
	head = utils.BuildList([]int{1, 2})
	reorderListFastAndSlowPointers(head)
	assert.Equal(t, []int{1, 2}, utils.TraverseList(head))
}
