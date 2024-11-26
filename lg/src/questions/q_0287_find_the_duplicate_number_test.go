package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestFindDuplicate(t *testing.T) {
	nums := []int{1, 3, 4, 2, 2}
	ans := findDuplicate(nums)
	require.Equal(t, 2, ans)
}
