package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestTwoSum(t *testing.T) {
	assert.Equal(t, []int{1, 2}, twoSum([]int{1, 2, 3}, 5))
	assert.Equal(t, []int{1, 2}, twoSum([]int{3, 2, 4}, 6))
	assert.Equal(t, []int{0, 1}, twoSum([]int{3, 3}, 6))
	assert.Equal(t, []int{1, 2}, twoSum([]int{2, 5, 5, 11}, 10))
}
