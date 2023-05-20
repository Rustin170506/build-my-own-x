package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestIsAnagram(t *testing.T) {
	assert.True(t, isAnagram("abc", "abc"))
	assert.False(t, isAnagram("aac", "abc"))
	assert.False(t, isAnagram("aa", "abc"))
}
