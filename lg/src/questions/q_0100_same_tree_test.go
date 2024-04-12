package questions

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/hi-rustin/lg/src/utils"
)

func TestIsSameTree(t *testing.T) {
	tests := []struct {
		p      *utils.TreeNode
		q      *utils.TreeNode
		output bool
	}{
		{
			p:      utils.BuildTree([]int{1, 2, 3}),
			q:      utils.BuildTree([]int{1, 2, 3}),
			output: true,
		},
		{
			p:      utils.BuildTree([]int{1, 2}),
			q:      utils.BuildTree([]int{1, -1, 2}),
			output: false,
		},
	}

	for i, test := range tests {
		ret := isSameTree(test.p, test.q)
		require.Equal(t, test.output, ret, i)
		ret = isSameTreeWithLevelTraversal(test.p, test.q)
		require.Equal(t, test.output, ret, i)
	}
}
