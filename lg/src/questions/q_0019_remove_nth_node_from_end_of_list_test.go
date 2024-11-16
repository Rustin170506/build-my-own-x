package questions

import (
	"testing"

	"github.com/hi-rustin/lg/src/utils"
	"github.com/stretchr/testify/assert"
)

func TestRemoveNthFromEnd(t *testing.T) {
	head := utils.BuildList([]int{1, 2, 3, 4, 5})
	assert.Equal(t, []int{1, 2, 3, 5}, utils.TraverseList(removeNthFromEnd(head, 2)))
	head = utils.BuildList([]int{1})
	assert.Equal(t, []int(nil), utils.TraverseList(removeNthFromEnd(head, 1)))
	head = utils.BuildList([]int{1, 2})
	assert.Equal(t, []int{1}, utils.TraverseList(removeNthFromEnd(head, 1)))
	head = utils.BuildList([]int{1, 2})
	assert.Equal(t, []int{2}, utils.TraverseList(removeNthFromEnd(head, 2)))
}
