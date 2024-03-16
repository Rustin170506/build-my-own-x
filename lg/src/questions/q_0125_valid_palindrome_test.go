package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestValidPalindrome(t *testing.T) {
	assert.Equal(t, true, isPalindrome("A man, a plan, a canal: Panama"))
	assert.Equal(t, false, isPalindrome("race a car"))
	assert.Equal(t, false, isPalindrome("0P"))
}
