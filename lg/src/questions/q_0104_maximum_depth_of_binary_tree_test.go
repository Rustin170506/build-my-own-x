package questions

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/hi-rustin/lg/src/utils"
)

func TestMaxDepth(t *testing.T) {
	require.Equal(t, maxDepth(utils.BuildTree([]int{3, 9, 20, -1, -1, 15, 7})), 3)
}
