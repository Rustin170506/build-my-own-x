package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestContainsDuplicate(t *testing.T) {
	assert.False(t, containsDuplicate([]int{1, 2, 3}))
	assert.True(t, containsDuplicate([]int{1, 2, 2}))
}
