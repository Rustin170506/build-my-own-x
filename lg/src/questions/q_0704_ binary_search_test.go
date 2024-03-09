package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestSearch(t *testing.T) {
	require.Equal(t, 0, search([]int{1, 2, 3}, 1))
	require.Equal(t, 4, search([]int{-1, 0, 3, 5, 9, 12}, 9))
	require.Equal(t, -1, search([]int{-1, 0, 3, 5, 9, 12}, 2))
}
