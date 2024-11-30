package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestLRUCache(t *testing.T) {
	cache := ConstructorCache(2)
	cache.Put(1, 1)
	cache.Put(2, 2)
	require.Equal(t, 1, cache.Get(1))
	require.Equal(t, 2, cache.Get(2))
	cache.Put(3, 3)
	require.Equal(t, -1, cache.Get(1))
	require.Equal(t, 3, cache.Get(3))
	cache.Put(4, 4)
	require.Equal(t, -1, cache.Get(2))
	require.Equal(t, 3, cache.Get(3))
	require.Equal(t, 4, cache.Get(4))
}
