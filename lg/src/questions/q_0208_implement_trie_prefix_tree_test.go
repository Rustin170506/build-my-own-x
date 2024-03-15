package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestTrie(t *testing.T) {
	trie := Constructor()
	trie.Insert("apple")
	require.True(t, trie.Search("apple"))
	require.False(t, trie.Search("app"))
	require.True(t, trie.StartsWith("app"))
	trie.Insert("app")
	require.True(t, trie.Search("app"))
}
