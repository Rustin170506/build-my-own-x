package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestMinStack(t *testing.T) {
	minStack := ConstructorMinStack()
	minStack.Push(-2)
	minStack.Push(0)
	minStack.Push(-3)
	require.Equal(t, -3, minStack.GetMin())
	minStack.Pop()
	require.Equal(t, 0, minStack.Top())
	require.Equal(t, -2, minStack.GetMin())
}
